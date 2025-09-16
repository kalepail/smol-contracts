#![cfg(test)]

// GCEDG23LK46PHGXIY63E3ELQGBX6VHQ4EWLYT7FMLOOCIS3ZY2ITHDXB
// SA3FD3NYLOV3JPXQYC66LLGIEBYQ4NSSG4RUNMMB53BVF75XCKNPZR3O

mod comet_factory {
    soroban_sdk::contractimport!(file = "../../ext/comet_factory.wasm");
}
mod comet {
    soroban_sdk::contractimport!(file = "../../ext/comet.wasm");
}

use comet::{Client as CometClient, WASM as COMET_WASM};
use comet_factory::{Client as CometFactoryClient, WASM as COMET_FACTORY_WASM};

use core::i128;
use std::println;

use ed25519_dalek::SigningKey;
use hex;
use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{
    testutils::Address as _, token, unwrap::UnwrapOptimized, xdr::{
        AccountId, AlphaNum12, Asset, AssetCode12, FromXdr, Limits, PublicKey, ToXdr, Uint256, WriteXdr 
    }, Address, Bytes, BytesN, Env, Symbol
};

extern crate std;
use crate::{Contract, ContractArgs, ContractClient};

fn generate_smol_issuer(env: &Env) -> BytesN<32> {
    let signing_key = SigningKey::generate(&mut rand::thread_rng());
    BytesN::from_array(env, &signing_key.verifying_key().to_bytes())
}

fn create_contract<'a>(
    env: &Env,
    smol_issuer: &BytesN<32>,
    comet_factory: &Address,
    base_asset: &Address,
) -> ContractClient<'a> {
    let contract_id = env.register(
        Contract,
        ContractArgs::__constructor(smol_issuer, comet_factory, base_asset),
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
    let comet_factory = env.register_contract_wasm(None, COMET_FACTORY_WASM);
    env.register_contract_wasm(None, COMET_WASM);
    let base_asset = env.register_stellar_asset_contract_v2(admin.clone());
    let user = Address::generate(&env);

    let base_asset_client = token::StellarAssetClient::new(&env, &base_asset.address());
    let comet_factory_client = CometFactoryClient::new(&env, &comet_factory);

    let wasm_hash_bytes =
        hex::decode("08c6d3b5c9feeef8ec1b30d5194fb296aab5dbdb067bb10e14673bac06388e26").unwrap();
    let wasm_hash = BytesN::from_array(&env, &wasm_hash_bytes.try_into().unwrap());

    comet_factory_client.init(&wasm_hash);

    println!("Base asset: {:?}", base_asset_client.name());

    const STROOP: i128 = 10i128.pow(7);
    const MAX_RATIO: i128 = (STROOP / 3) + 1;

    let base_amount = 100_0000000;
    let base_amount_in = base_amount
        .fixed_mul_floor(MAX_RATIO, STROOP)
        .unwrap_optimized();

    println!("Base amount in: {:?}", base_amount_in);

    base_asset_client.mint(&user, &base_amount_in);
    base_asset_client.mint(&admin, &(base_amount * 2));

    // Create contract using helper function
    let client = create_contract(&env, &smol_issuer, &comet_factory, &base_asset.address());

    // Call the mint function
    let minted_token_address = client.mint(&user, &get_asset_bytes(&env, &client, &smol_issuer));
    let minted_token_client = token::Client::new(&env, &minted_token_address.0);

    // Verify that a token address was returned (should be a valid address)
    assert!(minted_token_address.0.exists());
    println!("Minted token: {:?}", minted_token_client.symbol());

    // Verify that the mint function can be called multiple times without error and that the addresses are different
    let second_mint_address = client.mint(&user, &get_asset_bytes(&env, &client, &smol_issuer));
    let second_mint_client = token::Client::new(&env, &second_mint_address.0);

    assert_ne!(minted_token_address.0, second_mint_address.0);
    println!("Second mint token: {:?}", second_mint_client.symbol());

    let comet_client = CometClient::new(&env, &minted_token_address.1);
    let (amount_out, _) = comet_client.swap_exact_amount_in(
        &base_asset.address(),
        &base_amount_in,
        &minted_token_address.0,
        &0,
        &i128::MAX,
        &user,
    );

    println!("Amount out: {:?}", amount_out);

    assert_eq!(amount_out, 68_701_143_415591);
}

fn get_asset_bytes(env: &Env, client: &ContractClient, smol_issuer: &BytesN<32>) -> Bytes {
    let asset = Asset::CreditAlphanum12(AlphaNum12 {
        asset_code: counter_to_ascii(get_token_counter(&env, &client)),
        issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
            smol_issuer.to_array(),
        ))),
    });
    
    Bytes::from_slice(&env, &asset.to_xdr(Limits::none()).unwrap())
}

fn get_token_counter(env: &Env, client: &ContractClient) -> u64 {
    env.as_contract(&client.address, || {
        let token_counter: u64 = env.storage().instance().get(&Symbol::new(&env, "token_counter")).unwrap();
        token_counter
    })
}

fn counter_to_ascii(counter: u64) -> AssetCode12 {
    // Convert counter to 12-character ASCII bytes array (zero-padded)
    let mut code_bytes = [b'0'; 12]; // Start with all zeros
    let mut pos = 11; // Start from the rightmost position
    let mut num = counter;

    // Convert number to ASCII digits from right to left
    while num > 0 {
        code_bytes[pos] = b'0' + (num % 10) as u8;
        num /= 10;
        pos = pos.saturating_sub(1);
    }

    AssetCode12(code_bytes)
}