#![cfg(test)]

// GCEDG23LK46PHGXIY63E3ELQGBX6VHQ4EWLYT7FMLOOCIS3ZY2ITHDXB
// SA3FD3NYLOV3JPXQYC66LLGIEBYQ4NSSG4RUNMMB53BVF75XCKNPZR3O

extern crate std;

use std::println;

use ed25519_dalek::SigningKey;
use soroban_sdk::{testutils::Address as _, token, xdr::ToXdr, Address, BytesN, Env};

use crate::{Contract, ContractArgs, ContractClient};

fn generate_smol_issuer(env: &Env) -> BytesN<32> {
    let signing_key = SigningKey::generate(&mut rand::thread_rng());
    BytesN::from_array(env, &signing_key.verifying_key().to_bytes())
}

fn create_contract<'a>(
    env: &Env,
    smol_issuer: &BytesN<32>,
    comet_factory: &Address,
) -> ContractClient<'a> {
    let contract_id = env.register(
        Contract,
        ContractArgs::__constructor(smol_issuer, comet_factory),
    );
    ContractClient::new(env, &contract_id)
}

#[test]
fn test_mint() {
    let env = Env::from_ledger_snapshot_file("snapshot.json");

    env.mock_all_auths();

    // Create test addresses
    let admin = Address::from_str(
        &env,
        "GCEDG23LK46PHGXIY63E3ELQGBX6VHQ4EWLYT7FMLOOCIS3ZY2ITHDXB",
    );
    let admin_bytes = admin.clone().to_xdr(&env);
    let mut admin_array = [0u8; 32];
    admin_bytes.slice(12..).copy_into_slice(&mut admin_array);
    let smol_issuer = BytesN::from_array(&env, &admin_array);

    // println!("Admin array: {:?}", admin_array);

    // let test = env.register_stellar_asset_contract_v2(admin);
    // let test_client = token::Client::new(&env, &test.address());

    // println!("Test: {:?}", test_client.symbol());

    // let smol_issuer = generate_smol_issuer(&env);
    let comet_factory = Address::generate(&env);
    let user = Address::generate(&env);

    // Create contract using helper function
    let client = create_contract(&env, &smol_issuer, &comet_factory);

    // Call the mint function
    let minted_token_address = client.mint(&user);
    let minted_token_client = token::Client::new(&env, &minted_token_address);

    // Verify that a token address was returned (should be a valid address)
    assert!(minted_token_address.exists());
    println!("Minted token: {:?}", minted_token_client.symbol());

    // Verify that the mint function can be called multiple times without error and that the addresses are different
    let second_mint_address = client.mint(&user);
    let second_mint_client = token::Client::new(&env, &second_mint_address);

    assert_ne!(minted_token_address, second_mint_address);
    println!("Second mint token: {:?}", second_mint_client.symbol());
}
