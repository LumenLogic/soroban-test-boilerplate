#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test_hello_contract_execution() {
    // 1. Initialize the mock environment
    let env = Env::default();

    // 2. Register the contract
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // 3. Invoke the contract function
    let words = client.hello(&symbol_short!("Dev"));

    // 4. Assert the expected outcome
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}