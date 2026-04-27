## Kaya Access
___________________________________________________________________________________________________________________________
## Problem: 
Persons with Disabilities (PWDs) in the Philippines often face delayed cash assistance and denial of mandated merchant discounts because physical PWD IDs are easily counterfeited, making it difficult for local pharmacies, groceries, and local government units (LGUs) to verify authenticity.
___________________________________________________________________________________________________________________________
## Solution
KayaAccess uses a Soroban smart contract to anchor cryptographically hashed PWD IDs on-chain, allowing merchants to instantly verify an ID's authenticity via a web app, while enabling the LGU or DSWD (Department of Social Welfare and Development) to disburse programmed XLM/USDC cash aid directly and transparently to verified PWD wallets.
____________________________________________________________________________________________________________________________

## Demo Flow

## Architecture
* **Smart Contract:** Built with Rust and the Soroban SDK.
* **Frontend:** React (deployed via Vite) connecting to the Stellar Testnet.
* **Wallet Integration:** Freighter extension.

## Contract Functions
* `initialize()`: Sets up the initial state and admin permissions.
* `register_id()`: Anchors the cryptographic hash of a PWD ID to the ledger.
* `verify_id()`: Allows merchants/LGUs to authenticate an ID against the on-chain hash.
* `disburse_aid()`: Facilitates transparent financial aid distribution.

## Build and Deploy Instructions
**1. Build the Contract:**
`stellar contract build --manifest-path contracts/hello-world/Cargo.toml`

**2. Deploy to Testnet:**
`stellar contract deploy --wasm target/wasm32v1-none/release/kaya_access.wasm --network testnet --source stellar-ide-default`


## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
__________________________________________________________________________________________________________________________
## STELLAR FEATURES USED
Soroban smart contracts (Core registry, tamper-proof verification, and conditional payout logic)
USDC / XLM transfers (For direct cash aid distribution)
___________________________________________________________________________________________________________________________
## TARGET USERS
Who: PWDs receiving government aid, Local merchants (pharmacy/grocery owners), LGU/DSWD officers.
Where: Philippines (e.g., Bangsamoro Autonomous Region / Mindanao).
Why they care: PWDs get faster access to discounts and aid; merchants avoid fraud; government ensures transparent, leak-proof fund distribution.
___________________________________________________________________________________________________________________________
## CORE FEATURE (MVP) 
An LGU administrator registers a PWD's hashed ID to their Stellar wallet on-chain. A merchant queries the verify_id function to instantly confirm validity before applying a discount, and the LGU calls disburse_aid to send USDC automatically and securely to the verified PWD wallet.
____________________________________________________________________________________________________________________________
## Smart Contract
Deployed on Stellar testnet:

Contract ID:
CANNROOD4KQLWKGKQ6FYPD2GSBGBORDNTXSMEPF7ZH37ALQFFOESYA5O

https://stellar.expert/explorer/testnet/tx/bea5553f49b817595e5ca11a071bcf6b25110f200639739363d925ae3e16193a

<img width="1888" height="892" alt="image" src="https://github.com/user-attachments/assets/0ad8a5ab-96dc-48ab-9cf7-fb70f0ce7186" />

