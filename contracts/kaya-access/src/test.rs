#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient;

#[test]
fn test_happy_path_register_and_disburse() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, KayaAccessContract);
    let client = KayaAccessContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let id_hash = BytesN::from_array(&env, &[1u8; 32]);

    // Setup Token
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract(token_admin.clone());
    let token = TokenClient::new(&env, &token_contract);
    let token_admin_client = StellarAssetClient::new(&env, &token_contract);
    
    // Mint tokens to LGU Admin
    token_admin_client.mint(&admin, &1000);

    // Execute MVP Flow
    client.initialize(&admin);
    client.register_id(&user, &id_hash);
    
    assert_eq!(client.verify_id(&user), true);

    client.disburse_aid(&token_contract, &user, &500);

    // Verify user received the aid
    assert_eq!(token.balance(&user), 500);
}

#[test]
#[should_panic(expected = "PWD ID already registered for this wallet")]
fn test_edge_case_duplicate_registration() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, KayaAccessContract);
    let client = KayaAccessContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let id_hash = BytesN::from_array(&env, &[1u8; 32]);

    client.initialize(&admin);
    
    // First registration succeeds
    client.register_id(&user, &id_hash);
    
    // Second registration of the same user fails
    client.register_id(&user, &id_hash); 
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, KayaAccessContract);
    let client = KayaAccessContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let id_hash = BytesN::from_array(&env, &[5u8; 32]); // Specific mock hash

    client.initialize(&admin);
    client.register_id(&user, &id_hash);

    // Verify state directly via verify_id boolean check
    assert_eq!(client.verify_id(&user), true);
    
    // Unregistered user should return false
    let unregistered_user = Address::generate(&env);
    assert_eq!(client.verify_id(&unregistered_user), false);
}
