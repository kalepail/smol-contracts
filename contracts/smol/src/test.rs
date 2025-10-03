#![cfg(test)]

// GCEDG23LK46PHGXIY63E3ELQGBX6VHQ4EWLYT7FMLOOCIS3ZY2ITHDXB
// SA3FD3NYLOV3JPXQYC66LLGIEBYQ4NSSG4RUNMMB53BVF75XCKNPZR3O

mod comet {
    soroban_sdk::contractimport!(file = "../../ext/comet.wasm");
}

use comet::{Client as CometClient, WASM as COMET_WASM};

use std::println;

use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    token,
    unwrap::UnwrapOptimized,
    xdr::{AccountId, AlphaNum12, Asset, AssetCode12, Limits, PublicKey, ToXdr, Uint256, WriteXdr},
    Address, Bytes, BytesN, Env,
};

extern crate std;
use crate::{Contract, ContractArgs, ContractClient};

#[test]
fn test() {
    let env = Env::default();
    let admin = Address::from_str(
        &env,
        "GCEDG23LK46PHGXIY63E3ELQGBX6VHQ4EWLYT7FMLOOCIS3ZY2ITHDXB",
    );
    let admin_bytes = admin.clone().to_xdr(&env);
    let mut admin_array = [0u8; 32];
    admin_bytes.slice(12..).copy_into_slice(&mut admin_array);

    let smol_issuer = BytesN::from_array(&env, &admin_array);

    let asset_bytes = get_asset_bytes(&env, 1, &smol_issuer);

    println!("Asset bytes: {:?}", asset_bytes);
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

    // println!("Admin array: {:?}", admin_array);

    let smol_issuer = BytesN::from_array(&env, &admin_array);
    let comet_wasm = env.deployer().upload_contract_wasm(COMET_WASM);
    let base_asset = env.register_stellar_asset_contract_v2(admin.clone());
    let user = Address::generate(&env);

    let base_asset_client = token::StellarAssetClient::new(&env, &base_asset.address());

    const STROOP: i128 = 10i128.pow(7);
    const MAX_RATIO: i128 = (STROOP / 3) + 1;

    let base_amount = 100_0000000;
    let base_amount_in = base_amount
        .fixed_mul_floor(MAX_RATIO, STROOP)
        .unwrap_optimized();

    println!("Base amount in: {:?}", base_amount_in);
    println!("Base asset: {:?}", base_asset_client.name());

    base_asset_client.mint(&user, &i128::MAX);
    base_asset_client.mint(&admin, &(base_amount * 2));

    // Create contract using helper function
    let client = create_contract(&env, &admin, &comet_wasm, &base_asset.address());

    // Call the mint function
    let minted_token_address = client.coin_it(
        &user,
        &get_asset_bytes(&env, 0, &smol_issuer),
        &BytesN::random(&env),
        &None,
    );
    let minted_token_client = token::Client::new(&env, &minted_token_address.0);

    // Verify that a token address was returned (should be a valid address)
    assert!(minted_token_address.0.exists());
    // println!("Minted token: {:?}", minted_token_client.symbol());

    // Verify that the mint function can be called multiple times without error and that the addresses are different
    let second_mint_address = client.coin_it(
        &user,
        &get_asset_bytes(&env, 1, &smol_issuer),
        &BytesN::random(&env),
        &None,
    );
    let second_mint_client = token::Client::new(&env, &second_mint_address.0);

    assert_ne!(minted_token_address.0, second_mint_address.0);
    // println!("Second mint token: {:?}", second_mint_client.symbol());

    let comet_client = CometClient::new(&env, &minted_token_address.1);

    let mut first_out: i128 = 0;
    let mut fiftieth_out: i128 = 0;

    for i in 1..=100 {
        let token_amount_in = comet_client
            .get_balance(&base_asset.address())
            .fixed_mul_floor(MAX_RATIO, STROOP)
            .unwrap_optimized();

        let amount_in = token_amount_in.min(1250_0000000);
        let (amount_out, _) = comet_client.swap_exact_amount_in(
            &base_asset.address(),
            &amount_in, // Max $0.5 of KALE
            &minted_token_address.0,
            &0,
            &i128::MAX,
            &user,
            &None,
        );

        if i == 1 {
            println!("1st TEST out: {:?}, amount_in: {:?}", amount_out, amount_in);
            first_out = amount_out; // First output
        }

        if i == 50 {
            println!(
                "50th TEST out: {:?}, amount_in: {:?}",
                amount_out, amount_in
            );
            fiftieth_out = amount_out; // Fifty output
        }

        // println!("{:?}", env.logs().all())
    }

    println!(
        "Total TEST left: {:?}",
        comet_client.get_balance(&minted_token_address.0)
    );
    println!(
        "Total KALE in: {:?}",
        comet_client.get_balance(&base_asset.address())
    );

    let (amount_out, _) = comet_client.swap_exact_amount_in(
        &minted_token_address.0,
        &first_out,
        // &fiftieth_out,
        // &10_000_000_0000000,
        &base_asset.address(),
        &0,
        &i128::MAX,
        &user,
        &None,
    );

    // println!("{:?}", env.logs().all());

    println!("Amount KALE out: {:?}", amount_out);

    // assert_eq!(amount_out, 68_701_143_415591);
}

fn get_asset_bytes(env: &Env, counter: u64, smol_issuer: &BytesN<32>) -> Bytes {
    let asset = Asset::CreditAlphanum12(AlphaNum12 {
        asset_code: counter_to_ascii(counter),
        issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
            smol_issuer.to_array(),
        ))),
    });

    Bytes::from_slice(&env, &asset.to_xdr(Limits::none()).unwrap())
}

fn create_contract<'a>(
    env: &Env,
    admin: &Address,
    comet_wasm: &BytesN<32>,
    base_asset: &Address,
) -> ContractClient<'a> {
    let contract_id = env.register(
        Contract,
        ContractArgs::__constructor(admin, comet_wasm, base_asset),
    );
    ContractClient::new(env, &contract_id)
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
