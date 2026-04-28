#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, token};

#[contracttype]
pub enum DataKey {
    Admin,
    PwdRecord(Address), // Maps a user's wallet address to their PWD ID Hash
}

#[contract]
pub struct KayaAccessContract;

#[contractimpl]
impl KayaAccessContract {
    /// Initializes the contract with an LGU/DSWD Admin.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Registers a PWD ID hash to a user's wallet. Only the Admin can call this.
    /// Rejects if the user is already registered to prevent duplicate entries.
    pub fn register_id(env: Env, user: Address, id_hash: BytesN<32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth(); // Ensures only LGU can register IDs

        let record_key = DataKey::PwdRecord(user.clone());
        if env.storage().persistent().has(&record_key) {
            panic!("PWD ID already registered for this wallet");
        }

        env.storage().persistent().set(&record_key, &id_hash);
    }

    /// Verifies if a user has a valid PWD ID registered.
    /// Returns true if valid, emits an event for off-chain tracking.
    pub fn verify_id(env: Env, user: Address) -> bool {
        let record_key = DataKey::PwdRecord(user.clone());
        let is_valid = env.storage().persistent().has(&record_key);
        
        // Emit an event for the frontend/merchant web app
        env.events().publish((soroban_sdk::symbol_short!("verify"), user), is_valid);
        
        is_valid
    }

    /// Disburses financial aid (USDC/XLM) to a verified PWD wallet.
    /// Fails if the wallet is not a verified PWD.
    pub fn disburse_aid(env: Env, token_address: Address, user: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        // Check if user is a verified PWD before sending funds
        let record_key = DataKey::PwdRecord(user.clone());
        if !env.storage().persistent().has(&record_key) {
            panic!("Cannot disburse aid: User is not a verified PWD");
        }

        // Transfer tokens from the Admin (LGU Treasury) to the PWD
        let client = token::Client::new(&env, &token_address);
        client.transfer(&admin, &user, &amount);
    }
}


mod test;
