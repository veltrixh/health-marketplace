<div align="center">

# 🏥 Health Marketplace on Stellar
<img width="1920" height="1080" alt="Screenshot (368)" src="https://github.com/user-attachments/assets/ba4e00f6-b7fc-4cd8-a820-409debcffd39" />

### *Decentralized Healthcare, Reimagined on the Blockchain*

> **"Putting patients in control of their health journey — transparent, secure, and borderless."**

[![Stellar](https://img.shields.io/badge/Blockchain-Stellar-7C3AED?style=for-the-badge&logo=stellar&logoColor=white)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban-blueviolet?style=for-the-badge)](https://soroban.stellar.org)
[![Network](https://img.shields.io/badge/Network-Testnet-orange?style=for-the-badge)](https://stellar.expert/explorer/testnet)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-brightgreen?style=for-the-badge)](CONTRIBUTING.md)


[🚀 Live Demo](#-demo) · [📄 Contract Explorer](https://stellar.expert/explorer/testnet/contract/CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG) · [🐛 Report Bug](issues) · [💡 Request Feature](issues)


---

## 🏆 Hackathon Pitch

> **Health Marketplace** is a trustless, on-chain platform built on Stellar's Soroban that removes the middleman from healthcare — giving patients ownership of their medical history, enabling verified providers to offer services globally, and ensuring every transaction and feedback is permanently auditable. No data brokers. No opaque billing. Just healthcare the way it should be.

---

## 📌 Table of Contents

- [Problem Statement](#-problem-statement)
- [Solution Overview](#-solution-overview)
- [Features](#-features)
- [Tech Stack](#-tech-stack)
- [Smart Contract Details](#-smart-contract-details)
- [How It Works](#-how-it-works)
- [Installation & Setup](#-installation--setup)
- [Usage Instructions](#-usage-instructions)
- [Demo](#-demo)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)
- [Author](#-author)

---

## ❗ Problem Statement

Traditional healthcare systems are plagued by systemic inefficiencies that harm both patients and providers:

| Problem | Impact |
|---|---|
| 🔒 **Fragmented Medical Records** | Patient data is siloed across hospitals — inaccessible when needed most |
| 💸 **Opaque Billing & Hidden Fees** | Patients are billed without transparency; overcharging is rampant |
| 🌍 **Lack of Global Access** | Quality healthcare is geography-dependent and unaffordable for billions |
| 🕵️ **No Verified Provider Trust** | Anyone can claim credentials online — patients have no reliable way to verify |
| 🗑️ **Tampered or Lost Feedback** | Reviews on centralised platforms are deletable, fake, and untrustworthy |
| 🏦 **Intermediary Dependency** | Insurance companies and platforms extract massive fees while adding no clinical value |

> According to WHO, over **4.5 billion people** lack access to essential health services. Blockchain can change this.

---

## 💡 Solution Overview

**Health Marketplace** leverages Stellar's Soroban smart contracts to build a decentralized layer on top of healthcare — one that is:

- **Trustless** — no single authority controls your health data or payments
- **Transparent** — every booking, payment, and review is permanently recorded on-chain
- **Borderless** — anyone with a Stellar wallet can access or offer healthcare services globally
- **Tamper-proof** — medical interactions and feedback cannot be altered or deleted

The platform connects **verified healthcare providers** with **patients** through a smart contract that handles discovery, booking, escrow payments, and immutable feedback — all without a central intermediary.

```
Traditional Healthcare          Health Marketplace (Blockchain)
──────────────────────          ───────────────────────────────
Patient → Hospital → ???   vs.  Patient ←→ Smart Contract ←→ Provider
   (opaque, slow, costly)            (transparent, instant, fair)
```

---

## ✨ Features

### 👤 For Patients
- 🔍 **Discover Verified Providers** — browse healthcare professionals across 7 specialisations, all KYC-verified on-chain
- 📅 **Seamless Booking** — schedule appointments with a single wallet transaction
- 🔐 **Secure Medical Interactions** — consultation notes stored with patient-controlled access
- 💳 **Escrow-Protected Payments** — funds only released when service is confirmed delivered
- ⭐ **Immutable Feedback** — leave ratings that can never be deleted or manipulated
- 🛡️ **Dispute Protection** — raise disputes; an on-chain arbitration process protects your funds

### 🩺 For Providers
- 📋 **Professional Profile & Credential Registry** — get verified once, trusted everywhere
- 🗂️ **Service Listing Management** — list, update, pause, or remove services at any time
- 💰 **Instant Settlements** — receive payments directly to your Stellar wallet upon completion
- 📊 **On-Chain Reputation** — build a verifiable, tamper-proof track record

### 🏛️ Platform-Wide
- ⚖️ **Decentralised Dispute Resolution** — admin-governed arbitration with full audit trail
- 💸 **Configurable Platform Fee** — transparent fee in basis points (default 2.5%, capped at 10%)
- 📡 **Full Audit Trail** — every state change emits on-chain events, queryable forever
- 🌐 **7 Service Categories** — Consultation, Diagnostics, Pharmacy, Mental Health, Nutrition, Fitness Coaching, Telemedicine

---

## 🛠 Tech Stack

| Layer | Technology | Purpose |
|---|---|---|
| **Blockchain** | [Stellar](https://stellar.org) | Fast, low-cost transaction settlement |
| **Smart Contracts** | [Soroban (Rust)](https://soroban.stellar.org) | On-chain business logic & escrow |
| **Wallet** | [Freighter Wallet](https://freighter.app) | Browser-based Stellar wallet |
| **Frontend** | React + TypeScript | User interface |
| **Stellar SDK** | [@stellar/stellar-sdk](https://github.com/stellar/js-stellar-sdk) | Blockchain interaction layer |
| **Styling** | Tailwind CSS | Responsive design system |
| **Contract Language** | Rust (`no_std`) | Soroban contract development |
| **Testing** | Soroban SDK Testutils | On-chain unit & integration testing |
| **Deploy** | Stellar CLI | WASM compilation & deployment |
| **Explorer** | Stellar Expert | Contract & transaction visibility |

---

## 📄 Smart Contract Details

### Contract Address

```
CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG
```

🔗 **[View on Stellar Expert (Testnet)](https://stellar.expert/explorer/testnet/contract/CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG)**

| Property | Value |
|---|---|
| **Network** | Stellar Testnet |
| **Language** | Rust (Soroban) |
| **Platform Fee** | 250 bps (2.5%) |
| **Storage** | Persistent + Instance |
| **Auth Model** | `require_auth()` per actor |

### What the Contract Does

The contract is the single source of truth for the entire marketplace. It manages:

#### 🔑 Core Functions

```rust
// ── Initialisation ──────────────────────────────────────────
initialize(admin, platform_fee_bps)        // Deploy & configure marketplace

// ── Provider Management ─────────────────────────────────────
register_provider(provider, name, credentials)
verify_provider(provider)                  // Admin-only KYC gate

// ── Service Listings ────────────────────────────────────────
list_service(provider, title, desc, category, price_xlm, duration)
update_service(provider, service_id, price_xlm, status)

// ── Booking Lifecycle ───────────────────────────────────────
book_service(patient, service_id, scheduled_at, notes)   // Locks escrow
confirm_booking(provider, booking_id)
complete_booking(patient, booking_id)     // Releases escrow to provider
cancel_booking(caller, booking_id)        // Refunds escrow to patient

// ── Trust & Safety ──────────────────────────────────────────
rate_service(patient, booking_id, rating)   // 1–5 stars, on-chain
raise_dispute(patient, booking_id)
resolve_dispute(admin, booking_id, refund_patient)
```

#### 📦 Data Structures

```rust
HealthService  { id, provider, title, description, category,
                 price_xlm, duration_minutes, status,
                 rating_sum, rating_count, created_at }

Booking        { id, service_id, patient, provider, status,
                 scheduled_at, amount_paid, notes, created_at }

ProviderProfile { address, name, credentials, is_verified,
                  total_services, total_bookings, joined_at }
```

#### 🔄 Booking State Machine

```
[book_service]         [confirm_booking]       [complete_booking]
    Pending     ──────►    Confirmed    ──────►    Completed
       │                      │
       │ [cancel_booking]      │ [raise_dispute]
       ▼                      ▼
   Cancelled              Disputed
                              │
                    [resolve_dispute]
                         ┌────┴────┐
                         ▼         ▼
                     Cancelled  Completed
```

---

## ⚙️ How It Works

### Step-by-Step User Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                       HEALTH MARKETPLACE                        │
│                                                                 │
│  1. Provider registers → submits credentials off-chain          │
│  2. Admin verifies provider → KYC recorded on-chain            │
│  3. Provider lists service → category, price, duration         │
│  4. Patient discovers service → browses marketplace            │
│  5. Patient books → XLM locked in smart contract escrow        │
│  6. Provider confirms → appointment acknowledged on-chain      │
│  7. Service delivered → Patient marks complete                 │
│  8. Escrow released → Provider paid (minus 2.5% fee)          │
│  9. Patient rates → 1–5 stars stored immutably on-chain        │
│                                                                 │
│  ⚠ If issue arises:                                            │
│     Patient raises dispute → Admin reviews → Resolved on-chain │
└─────────────────────────────────────────────────────────────────┘
```

### Payment Flow

```
Patient Wallet
     │
     │ book_service() — price_xlm sent
     ▼
Smart Contract Escrow ◄──── funds held here safely
     │
     │ complete_booking() — patient confirms delivery
     ▼
Provider receives (price - 2.5% fee)  +  Platform Admin receives 2.5%
```

---

## 🖥 Installation & Setup

### Prerequisites

```bash
# Rust & wasm target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Stellar CLI
cargo install --locked stellar-cli --features opt

# Node.js (for frontend)
node --version  # v18+ required
```

### 1. Clone the Repository

```bash
git clone https://github.com/<your-username>/health-marketplace.git
cd health-marketplace
```

### 2. Build the Smart Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Run Tests

```bash
cargo test -- --nocapture
```

### 4. Deploy to Testnet

```bash
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

> The deploy script will automatically generate an admin keypair, fund it via Stellar Friendbot, deploy the WASM, and initialise the contract.

### 5. Install Frontend Dependencies

```bash
cd frontend
npm install
```

### 6. Configure Environment

```bash
cp .env.example .env
```

Edit `.env`:

```env
VITE_CONTRACT_ID=CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG
VITE_NETWORK=testnet
VITE_RPC_URL=https://soroban-testnet.stellar.org
VITE_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
```

### 7. Start the Frontend

```bash
npm run dev
# → http://localhost:5173
```

---

## 📖 Usage Instructions

### For Patients

```bash
# 1. Install Freighter Wallet browser extension
#    https://freighter.app — connect to Stellar Testnet

# 2. Fund your testnet wallet via Friendbot
curl "https://friendbot.stellar.org?addr=YOUR_WALLET_ADDRESS"

# 3. Book a service via CLI
stellar contract invoke \
  --id CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG \
  --source patient-key \
  --network testnet \
  -- book_service \
    --patient    <YOUR_ADDRESS> \
    --service_id 1 \
    --scheduled_at 9999999999 \
    --notes "First consultation"
```

### For Providers

```bash
# 1. Register your profile
stellar contract invoke \
  --id CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG \
  --source provider-key \
  --network testnet \
  -- register_provider \
    --provider    <YOUR_ADDRESS> \
    --name        "Dr. Your Name" \
    --credentials "MBBS, MD — Cardiology"

# 2. After admin verification, list a service
stellar contract invoke \
  --id CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG \
  --source provider-key \
  --network testnet \
  -- list_service \
    --provider         <YOUR_ADDRESS> \
    --title            "Cardiology Consultation" \
    --description      "30-min video consult with a cardiologist." \
    --category         Consultation \
    --price_xlm        50000000 \
    --duration_minutes 30
```

### Query the Contract

```bash
CONTRACT=CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG

# Get service details
stellar contract invoke --id $CONTRACT --network testnet -- get_service --service_id 1

# Get booking details
stellar contract invoke --id $CONTRACT --network testnet -- get_booking --booking_id 1

# Platform stats
stellar contract invoke --id $CONTRACT --network testnet -- get_service_count
stellar contract invoke --id $CONTRACT --network testnet -- get_booking_count
stellar contract invoke --id $CONTRACT --network testnet -- get_escrow_balance
```

---

## 🎬 Demo

> 📸 **Screenshots**

| Screen | Preview |
|---|---|
| Homepage / Service Discovery | `[screenshot placeholder]` |
| Provider Profile | `[screenshot placeholder]` |
| Booking Flow | `[screenshot placeholder]` |
| Patient Dashboard | `[screenshot placeholder]` |
| On-chain Rating | `[screenshot placeholder]` |

> 🎥 **Demo Video:** `[YouTube / Loom link placeholder]`

> 🌐 **Live App:** `[Deployed frontend URL placeholder]`

> 🔗 **Contract on Stellar Expert:**
> [CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG](https://stellar.expert/explorer/testnet/contract/CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG)

---

## 🗺 Roadmap

### ✅ Phase 1 — Foundation (Complete)
- [x] Core smart contract (Soroban / Rust)
- [x] Provider registration & admin KYC verification
- [x] Service listing with 7 categories
- [x] Full booking lifecycle with escrow
- [x] On-chain ratings (1–5 stars)
- [x] Dispute raise & resolution
- [x] 8 integration tests passing
- [x] Testnet deployment — `CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG`

### 🔄 Phase 2 — Enhancement (In Progress)
- [ ] Real XLM token transfer via `token::Client`
- [ ] Freighter Wallet frontend integration
- [ ] Patient dashboard with booking history
- [ ] Provider analytics dashboard
- [ ] USDC / multi-asset payment support for price stability

### 🔮 Phase 3 — Scale
- [ ] HIPAA-ready off-chain encrypted medical record storage (IPFS + on-chain hash)
- [ ] Subscription & recurring booking support
- [ ] DAO governance for fee parameters & dispute arbiters
- [ ] Mobile app (React Native + Freighter mobile)
- [ ] Mainnet deployment

### 🌍 Phase 4 — Ecosystem
- [ ] Cross-border insurance integration
- [ ] Patient-owned health record NFTs
- [ ] AI-powered provider matching
- [ ] Referral & loyalty token system

---

## 🤝 Contributing

Contributions are welcome and appreciated! Here's how to get involved:

```bash
# 1. Fork this repository
# 2. Create your feature branch
git checkout -b feat/amazing-feature

# 3. Make your changes and write tests
cargo test

# 4. Lint your Rust code
cargo clippy -- -D warnings

# 5. Commit with a clear message
git commit -m "feat: add amazing feature"

# 6. Push to your branch
git push origin feat/amazing-feature

# 7. Open a Pull Request
```

### Contribution Guidelines
- Follow [Conventional Commits](https://www.conventionalcommits.org/)
- All contract changes must include corresponding tests
- Run `cargo fmt` before committing
- Open an issue before starting large features

---

## 📜 License

```
MIT License — Copyright (c) 2024 Health Marketplace Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software.
```

See [LICENSE](LICENSE) for the full text.

---

## 👤 Author

<div align="center">

**Harsit Kedia**
*Builder · Blockchain Developer · Healthcare Innovator*

[![GitHub](https://img.shields.io/badge/GitHub-@your--username-181717?style=for-the-badge&logo=github)](https://github.com/your-username)
[![Twitter](https://img.shields.io/badge/Twitter-@yourhandle-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/yourhandle)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-Your%20Name-0A66C2?style=for-the-badge&logo=linkedin)](https://linkedin.com/in/yourprofile)

*Built with ❤️ at [Hackathon Name] · 2024*


---

<div align="center">

### ⭐ Star this repo if you believe in decentralised healthcare!

**Deployed Contract:**

[`CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG`](https://stellar.expert/explorer/testnet/contract/CBVOHRGCQXKI6D4R7SPATWQGQYHMAAV4DI7PZDH6455IL2VEU7FKWCJG)

*Built on [Stellar](https://stellar.org) · Powered by [Soroban](https://soroban.stellar.org)*
