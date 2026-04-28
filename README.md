# ♿ KayaAccess

**KayaAccess** is a decentralized identity and financial aid distribution platform for PWD communities in the Philippines, built on Stellar.

## 📝 Problem
Persons with Disabilities (PWDs) in the Philippines often face delayed cash assistance and denial of mandated merchant discounts. Physical PWD IDs are easily counterfeited, making it difficult for local pharmacies, groceries, and LGUs to verify authenticity. This leads to fraud, slow processing, and "leakage" in government fund distribution.

## ✅ Solution
KayaAccess lets the LGU anchor a cryptographically hashed PWD ID on-chain. Merchants can instantly verify a user's validity in under 5 seconds via a simple web interface. Once verified, the LGU can disburse programmed USDC/XLM aid directly to the user's wallet with near-zero fees ($0.00001), ensuring support reaches the intended recipient without middle-man delays.

## 🎥 Demo Flow
1.  **Admin Setup:** LGU Admin connects Freighter wallet and initializes the contract.
2.  **Registration:** Admin hashes a PWD ID and links it to the user's Stellar address.
3.  **Verification:** A merchant (pharmacy/grocery) calls `verify_id` to confirm discount eligibility.
4.  **Disbursement:** Admin calls `disburse_aid` to send USDC automatically to the verified PWD wallet.

## 🏗️ Architecture
```text
Browser (React + Vite)
  |-- Freighter Wallet API      (Signing transactions)
  |-- @stellar/stellar-sdk      (RPC calls, transaction building)
  |-- Soroban RPC               (On-chain state reads/writes)

Stellar Testnet
  |-- KayaAccess Smart Contract (ID Registry & Disbursement logic)
  |-- USDC Token Contract       (Stellar Asset Contract / SEP-41)

📂 Project Structure

kaya-access-proj/
├── contracts/
│   └── kaya-access/
│       ├── src/
│       │   ├── lib.rs          # ID Registry & Aid Disbursement logic
│       │   └── test.rs         # Logic & Security tests
│       └── Cargo.toml
├── Cargo.toml
└── README.md

✨ Stellar Features Used

| Feature | Usage |
| :--- | :--- |
| **Soroban Smart Contracts** | Core registry for tamper-proof ID verification and payout logic. |
| **Stellar Asset Contract (SAC)** | Integration with USDC/XLM for secure, stable aid disbursement. |
| **Events** | Real-time logging of ID verifications for merchant and government audits. |
| **Auth & Security** | `env.mock_all_auths()` and address-based authorization to prevent unauthorized aid disbursement. |

🚀 Smart Contract

**The contract is deployed and verified on the Stellar Testnet.**

**Contract ID:** `CANNROOD4KQLWKQ6FYPD2GSBGBORDNTXSMEPF7ZH37ALQFFOESYA5O`
**Explorer Link:** [View on Stellar.Expert](https://stellar.expert/explorer/testnet/contract/CANNROOD4KQLWKGKQ6FYPD2GSBGBORDNTXSMEPF7ZH37ALQFFOESYA5O)

🛠️ Build & Test
Build the contract

```bash
stellar contract build

##  Run unit tests
stellar contract test

##  📄 License
This project is licensed under the MIT License.
