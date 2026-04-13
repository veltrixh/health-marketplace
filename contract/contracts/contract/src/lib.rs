#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol, Vec, Map,
    log,
};


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceCategory {
    Consultation,
    Diagnostics,
    Pharmacy,
    MentalHealth,
    Nutrition,
    FitnessCoaching,
    Telemedicine,
}

/// Status lifecycle of a health service listing
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

/// Status of a booking
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BookingStatus {
    Pending,
    Confirmed,
    Completed,
    Cancelled,
    Disputed,
}

/// A health service listed by a provider
#[contracttype]
#[derive(Clone, Debug)]
pub struct HealthService {
    pub id: u64,
    pub provider: Address,
    pub title: String,
    pub description: String,
    pub category: ServiceCategory,
    pub price_xlm: i128,          // price in stroops (1 XLM = 10_000_000 stroops)
    pub duration_minutes: u32,
    pub status: ServiceStatus,
    pub rating_sum: u32,
    pub rating_count: u32,
    pub created_at: u64,
}

/// A booking made by a patient for a service
#[contracttype]
#[derive(Clone, Debug)]
pub struct Booking {
    pub id: u64,
    pub service_id: u64,
    pub patient: Address,
    pub provider: Address,
    pub status: BookingStatus,
    pub scheduled_at: u64,        // Unix timestamp
    pub amount_paid: i128,
    pub notes: String,
    pub created_at: u64,
}

/// Provider profile
#[contracttype]
#[derive(Clone, Debug)]
pub struct ProviderProfile {
    pub address: Address,
    pub name: String,
    pub credentials: String,
    pub is_verified: bool,
    pub total_services: u64,
    pub total_bookings: u64,
    pub joined_at: u64,
}

// ─────────────────────────────────────────────
//  Storage Keys
// ─────────────────────────────────────────────

const ADMIN: Symbol         = symbol_short!("ADMIN");
const SVC_COUNT: Symbol     = symbol_short!("SVC_CNT");
const BOOK_COUNT: Symbol    = symbol_short!("BOOK_CNT");
const PLATFORM_FEE: Symbol  = symbol_short!("PLT_FEE");    // basis points (e.g. 250 = 2.5%)
const ESCROW_BAL: Symbol    = symbol_short!("ESCROW");

#[contracttype]
pub enum DataKey {
    Service(u64),
    Booking(u64),
    Provider(Address),
    PatientBookings(Address),
    ProviderServices(Address),
    ProviderBookings(Address),
}

// ─────────────────────────────────────────────
//  Contract
// ─────────────────────────────────────────────

#[contract]
pub struct HealthMarketplace;

#[contractimpl]
impl HealthMarketplace {

    // ─── Admin / Initialisation ───────────────────────────────────────────

    /// Initialise the marketplace. Must be called once by the deployer.
    pub fn initialize(env: Env, admin: Address, platform_fee_bps: u32) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Already initialized");
        }
        admin.require_auth();

        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&SVC_COUNT, &0u64);
        env.storage().instance().set(&BOOK_COUNT, &0u64);
        env.storage().instance().set(&ESCROW_BAL, &0i128);

        // Fee capped at 10 % (1000 bps)
        let fee = platform_fee_bps.min(1000);
        env.storage().instance().set(&PLATFORM_FEE, &fee);

        log!(&env, "HealthMarketplace initialized by {}", admin);
    }

    // ─── Provider Management ──────────────────────────────────────────────

    /// Register as a healthcare provider
    pub fn register_provider(
        env: Env,
        provider: Address,
        name: String,
        credentials: String,
    ) {
        provider.require_auth();

        let profile = ProviderProfile {
            address: provider.clone(),
            name,
            credentials,
            is_verified: false,
            total_services: 0,
            total_bookings: 0,
            joined_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Provider(provider.clone()), &profile);

        log!(&env, "Provider registered: {}", provider);
    }

    /// Admin verifies a provider (KYC / credential check done off-chain)
    pub fn verify_provider(env: Env, provider: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let mut profile: ProviderProfile = env
            .storage()
            .persistent()
            .get(&DataKey::Provider(provider.clone()))
            .expect("Provider not found");

        profile.is_verified = true;
        env.storage()
            .persistent()
            .set(&DataKey::Provider(provider.clone()), &profile);

        log!(&env, "Provider verified: {}", provider);
    }

    // ─── Service Listings ─────────────────────────────────────────────────

    /// List a new health service
    pub fn list_service(
        env: Env,
        provider: Address,
        title: String,
        description: String,
        category: ServiceCategory,
        price_xlm: i128,
        duration_minutes: u32,
    ) -> u64 {
        provider.require_auth();

        // Only verified providers may list services
        let profile: ProviderProfile = env
            .storage()
            .persistent()
            .get(&DataKey::Provider(provider.clone()))
            .expect("Provider not registered");
        if !profile.is_verified {
            panic!("Provider not verified. Please complete KYC first.");
        }
        if price_xlm <= 0 {
            panic!("Price must be positive");
        }

        let id: u64 = env.storage().instance().get(&SVC_COUNT).unwrap_or(0);
        let new_id = id + 1;

        let service = HealthService {
            id: new_id,
            provider: provider.clone(),
            title,
            description,
            category,
            price_xlm,
            duration_minutes,
            status: ServiceStatus::Active,
            rating_sum: 0,
            rating_count: 0,
            created_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Service(new_id), &service);
        env.storage().instance().set(&SVC_COUNT, &new_id);

        // Track provider's services
        let mut svc_ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::ProviderServices(provider.clone()))
            .unwrap_or(Vec::new(&env));
        svc_ids.push_back(new_id);
        env.storage()
            .persistent()
            .set(&DataKey::ProviderServices(provider.clone()), &svc_ids);

        // Update provider stats
        let mut p = profile;
        p.total_services += 1;
        env.storage()
            .persistent()
            .set(&DataKey::Provider(provider), &p);

        log!(&env, "Service listed with id {}", new_id);
        new_id
    }

    /// Update an existing service listing
    pub fn update_service(
        env: Env,
        provider: Address,
        service_id: u64,
        price_xlm: i128,
        status: ServiceStatus,
    ) {
        provider.require_auth();

        let mut service: HealthService = env
            .storage()
            .persistent()
            .get(&DataKey::Service(service_id))
            .expect("Service not found");

        if service.provider != provider {
            panic!("Not the service owner");
        }

        service.price_xlm = price_xlm;
        service.status = status;
        env.storage()
            .persistent()
            .set(&DataKey::Service(service_id), &service);
    }

    // ─── Booking Flow ─────────────────────────────────────────────────────

    /// Patient books a service. Payment amount must be sent with the transaction.
    pub fn book_service(
        env: Env,
        patient: Address,
        service_id: u64,
        scheduled_at: u64,
        notes: String,
    ) -> u64 {
        patient.require_auth();

        let service: HealthService = env
            .storage()
            .persistent()
            .get(&DataKey::Service(service_id))
            .expect("Service not found");

        if service.status != ServiceStatus::Active {
            panic!("Service is not currently available");
        }
        if scheduled_at <= env.ledger().timestamp() {
            panic!("Scheduled time must be in the future");
        }

        let book_id: u64 = env.storage().instance().get(&BOOK_COUNT).unwrap_or(0);
        let new_id = book_id + 1;

        let booking = Booking {
            id: new_id,
            service_id,
            patient: patient.clone(),
            provider: service.provider.clone(),
            status: BookingStatus::Pending,
            scheduled_at,
            amount_paid: service.price_xlm,
            notes,
            created_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Booking(new_id), &booking);
        env.storage().instance().set(&BOOK_COUNT, &new_id);

        // Update escrow balance (in a real deployment, XLM transfer happens here)
        let escrow: i128 = env.storage().instance().get(&ESCROW_BAL).unwrap_or(0);
        env.storage()
            .instance()
            .set(&ESCROW_BAL, &(escrow + service.price_xlm));

        // Track bookings per patient & provider
        let mut patient_bkgs: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::PatientBookings(patient.clone()))
            .unwrap_or(Vec::new(&env));
        patient_bkgs.push_back(new_id);
        env.storage()
            .persistent()
            .set(&DataKey::PatientBookings(patient), &patient_bkgs);

        let mut prov_bkgs: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::ProviderBookings(service.provider.clone()))
            .unwrap_or(Vec::new(&env));
        prov_bkgs.push_back(new_id);
        env.storage()
            .persistent()
            .set(&DataKey::ProviderBookings(service.provider.clone()), &prov_bkgs);

        log!(&env, "Booking {} created for service {}", new_id, service_id);
        new_id
    }

    /// Provider confirms a pending booking
    pub fn confirm_booking(env: Env, provider: Address, booking_id: u64) {
        provider.require_auth();

        let mut booking: Booking = env
            .storage()
            .persistent()
            .get(&DataKey::Booking(booking_id))
            .expect("Booking not found");

        if booking.provider != provider {
            panic!("Not the booking provider");
        }
        if booking.status != BookingStatus::Pending {
            panic!("Booking is not in Pending state");
        }

        booking.status = BookingStatus::Confirmed;
        env.storage()
            .persistent()
            .set(&DataKey::Booking(booking_id), &booking);
    }

    /// Mark a booking as completed and release escrow to provider (minus platform fee)
    pub fn complete_booking(env: Env, patient: Address, booking_id: u64) {
        patient.require_auth();

        let mut booking: Booking = env
            .storage()
            .persistent()
            .get(&DataKey::Booking(booking_id))
            .expect("Booking not found");

        if booking.patient != patient {
            panic!("Not the booking patient");
        }
        if booking.status != BookingStatus::Confirmed {
            panic!("Booking must be Confirmed before completing");
        }

        let fee_bps: u32 = env
            .storage()
            .instance()
            .get(&PLATFORM_FEE)
            .unwrap_or(250);
        let platform_cut = booking.amount_paid * fee_bps as i128 / 10_000;
        let provider_payout = booking.amount_paid - platform_cut;

        // In production: token::Client::transfer to provider & admin
        // Here we update the ledger state and emit a log
        log!(
            &env,
            "Booking {} completed. Provider payout: {} stroops. Platform fee: {} stroops.",
            booking_id,
            provider_payout,
            platform_cut
        );

        // Reduce escrow
        let escrow: i128 = env.storage().instance().get(&ESCROW_BAL).unwrap_or(0);
        env.storage()
            .instance()
            .set(&ESCROW_BAL, &(escrow - booking.amount_paid));

        booking.status = BookingStatus::Completed;
        env.storage()
            .persistent()
            .set(&DataKey::Booking(booking_id), &booking);

        // Update provider stats
        let mut profile: ProviderProfile = env
            .storage()
            .persistent()
            .get(&DataKey::Provider(booking.provider.clone()))
            .expect("Provider not found");
        profile.total_bookings += 1;
        env.storage()
            .persistent()
            .set(&DataKey::Provider(booking.provider.clone()), &profile);
    }

    /// Cancel a booking (patient or provider can cancel)
    pub fn cancel_booking(env: Env, caller: Address, booking_id: u64) {
        caller.require_auth();

        let mut booking: Booking = env
            .storage()
            .persistent()
            .get(&DataKey::Booking(booking_id))
            .expect("Booking not found");

        if booking.patient != caller && booking.provider != caller {
            panic!("Caller is not party to this booking");
        }
        if matches!(
            booking.status,
            BookingStatus::Completed | BookingStatus::Cancelled
        ) {
            panic!("Booking already finalised");
        }

        // Refund escrow
        let escrow: i128 = env.storage().instance().get(&ESCROW_BAL).unwrap_or(0);
        env.storage()
            .instance()
            .set(&ESCROW_BAL, &(escrow - booking.amount_paid));

        booking.status = BookingStatus::Cancelled;
        env.storage()
            .persistent()
            .set(&DataKey::Booking(booking_id), &booking);

        log!(&env, "Booking {} cancelled. Refund: {} stroops", booking_id, booking.amount_paid);
    }

    // ─── Ratings ─────────────────────────────────────────────────────────

    /// Patient rates a completed service (1–5 stars)
    pub fn rate_service(
        env: Env,
        patient: Address,
        booking_id: u64,
        rating: u32,
    ) {
        patient.require_auth();

        if rating < 1 || rating > 5 {
            panic!("Rating must be between 1 and 5");
        }

        let booking: Booking = env
            .storage()
            .persistent()
            .get(&DataKey::Booking(booking_id))
            .expect("Booking not found");

        if booking.patient != patient {
            panic!("Only the patient can rate this booking");
        }
        if booking.status != BookingStatus::Completed {
            panic!("Can only rate completed bookings");
        }

        let mut service: HealthService = env
            .storage()
            .persistent()
            .get(&DataKey::Service(booking.service_id))
            .expect("Service not found");

        service.rating_sum += rating;
        service.rating_count += 1;
        env.storage()
            .persistent()
            .set(&DataKey::Service(booking.service_id), &service);

        log!(
            &env,
            "Service {} rated {}★ (avg: {}/5 from {} reviews)",
            booking.service_id,
            rating,
            service.rating_sum / service.rating_count,
            service.rating_count
        );
    }

    // ─── Read-Only Queries ────────────────────────────────────────────────

    pub fn get_service(env: Env, service_id: u64) -> HealthService {
        env.storage()
            .persistent()
            .get(&DataKey::Service(service_id))
            .expect("Service not found")
    }

    pub fn get_booking(env: Env, booking_id: u64) -> Booking {
        env.storage()
            .persistent()
            .get(&DataKey::Booking(booking_id))
            .expect("Booking not found")
    }

    pub fn get_provider(env: Env, provider: Address) -> ProviderProfile {
        env.storage()
            .persistent()
            .get(&DataKey::Provider(provider))
            .expect("Provider not found")
    }

    pub fn get_patient_bookings(env: Env, patient: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::PatientBookings(patient))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_provider_services(env: Env, provider: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::ProviderServices(provider))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_service_count(env: Env) -> u64 {
        env.storage().instance().get(&SVC_COUNT).unwrap_or(0)
    }

    pub fn get_booking_count(env: Env) -> u64 {
        env.storage().instance().get(&BOOK_COUNT).unwrap_or(0)
    }

    pub fn get_platform_fee(env: Env) -> u32 {
        env.storage().instance().get(&PLATFORM_FEE).unwrap_or(250)
    }

    pub fn get_escrow_balance(env: Env) -> i128 {
        env.storage().instance().get(&ESCROW_BAL).unwrap_or(0)
    }

    pub fn get_service_avg_rating(env: Env, service_id: u64) -> u32 {
        let service: HealthService = env
            .storage()
            .persistent()
            .get(&DataKey::Service(service_id))
            .expect("Service not found");
        if service.rating_count == 0 {
            0
        } else {
            service.rating_sum / service.rating_count
        }
    }

    // ─── Admin Controls ───────────────────────────────────────────────────

    pub fn update_platform_fee(env: Env, admin: Address, new_fee_bps: u32) {
        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if stored_admin != admin {
            panic!("Caller is not admin");
        }
        admin.require_auth();
        let fee = new_fee_bps.min(1000);
        env.storage().instance().set(&PLATFORM_FEE, &fee);
    }

    pub fn raise_dispute(env: Env, patient: Address, booking_id: u64) {
        patient.require_auth();

        let mut booking: Booking = env
            .storage()
            .persistent()
            .get(&DataKey::Booking(booking_id))
            .expect("Booking not found");

        if booking.patient != patient {
            panic!("Only the patient can raise a dispute");
        }
        if booking.status != BookingStatus::Confirmed {
            panic!("Can only dispute confirmed bookings");
        }

        booking.status = BookingStatus::Disputed;
        env.storage()
            .persistent()
            .set(&DataKey::Booking(booking_id), &booking);

        log!(&env, "Dispute raised for booking {}", booking_id);
    }

    pub fn resolve_dispute(
        env: Env,
        admin: Address,
        booking_id: u64,
        refund_patient: bool,
    ) {
        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if stored_admin != admin {
            panic!("Caller is not admin");
        }
        admin.require_auth();

        let mut booking: Booking = env
            .storage()
            .persistent()
            .get(&DataKey::Booking(booking_id))
            .expect("Booking not found");

        if booking.status != BookingStatus::Disputed {
            panic!("Booking is not in disputed state");
        }

        // Adjust escrow
        let escrow: i128 = env.storage().instance().get(&ESCROW_BAL).unwrap_or(0);
        env.storage()
            .instance()
            .set(&ESCROW_BAL, &(escrow - booking.amount_paid));

        booking.status = if refund_patient {
            log!(&env, "Dispute resolved: refund to patient for booking {}", booking_id);
            BookingStatus::Cancelled
        } else {
            log!(&env, "Dispute resolved: funds released to provider for booking {}", booking_id);
            BookingStatus::Completed
        };

        env.storage()
            .persistent()
            .set(&DataKey::Booking(booking_id), &booking);
    }
}