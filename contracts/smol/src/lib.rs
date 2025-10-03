#![no_std]

use crate::comet::{Args as CometArgs, FeeRecipient, FeeRule};
use soroban_sdk::{contract, contractimpl, token, vec, Address, Bytes, BytesN, Env, Symbol, Vec};

mod comet;

#[contract]
pub struct Contract;

// NOTE currently as-is admin must the be asset_bytes issuer address.
// Perhaps not ideal but does ensure not anyone can mint through this contract, only the admin.

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, admin: Address, comet_wasm: BytesN<32>, base_asset: Address) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "admin"), &admin);

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "comet_wasm"), &comet_wasm);

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "base_asset"), &base_asset);
    }
    pub fn update(
        env: Env,
        new_admin: Option<Address>,
        new_comet_wasm: Option<BytesN<32>>,
        new_base_asset: Option<Address>,
    ) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .unwrap();

        admin.require_auth();

        if let Some(new_admin) = new_admin {
            env.storage()
                .instance()
                .set(&Symbol::new(&env, "admin"), &new_admin);
        }

        if let Some(new_comet_wasm) = new_comet_wasm {
            env.storage()
                .instance()
                .set(&Symbol::new(&env, "comet_wasm"), &new_comet_wasm);
        }

        if let Some(new_base_asset) = new_base_asset {
            env.storage()
                .instance()
                .set(&Symbol::new(&env, "base_asset"), &new_base_asset);
        }
    }
    pub fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .unwrap();

        admin.require_auth();

        env.deployer().update_current_contract_wasm(wasm_hash);
    }
    pub fn swap_them_in(
        env: Env,
        user: Address,
        comet_addresses: Vec<Address>,
        tokens_out: Vec<Address>,
        token_amount_in: i128,
        fee_recipients: Option<Vec<FeeRecipient>>,
    ) {
        user.require_auth();

        let base_asset: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "base_asset"))
            .unwrap();

        for (comet_address_ref, token_out_ref) in
            itertools::izip!(comet_addresses.iter(), tokens_out.iter())
        {
            let comet_client = comet::Client::new(&env, &comet_address_ref);

            comet_client.swap_exact_amount_in(
                &base_asset,
                &token_amount_in,
                &token_out_ref,
                &0,
                &i128::MAX,
                &user,
                &fee_recipients,
            );
        }
    }
    pub fn coin_them(
        env: Env,
        user: Address,
        asset_bytes: Vec<Bytes>,
        salts: Vec<BytesN<32>>,
        fee_rules: Vec<Option<FeeRule>>,
    ) -> Vec<(Address, Address)> {
        user.require_auth();

        let admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .unwrap();

        admin.require_auth();

        let mut results = Vec::new(&env);

        for (asset_bytes_ref, salt_ref, fee_rule_ref) in
            itertools::izip!(asset_bytes.iter(), salts.iter(), fee_rules.iter())
        {
            results.push_front(coin_it(
                &env,
                &user,
                &asset_bytes_ref,
                &salt_ref,
                &fee_rule_ref,
            ));
        }

        results
    }
    pub fn coin_it(
        env: Env,
        user: Address,
        asset_bytes: Bytes,
        salt: BytesN<32>,
        fee_rule: Option<FeeRule>,
    ) -> (Address, Address) {
        user.require_auth();

        let admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .unwrap();

        admin.require_auth();

        // TODO consider reworking maybe the asset_bytes or storing the issuer so it's easier to authorize without causing issues

        coin_it(&env, &user, &asset_bytes, &salt, &fee_rule)
    }
}

fn coin_it(
    env: &Env,
    user: &Address,
    asset_bytes: &Bytes,
    salt: &BytesN<32>,
    fee_rule: &Option<FeeRule>,
) -> (Address, Address) {
    let comet_wasm: BytesN<32> = env
        .storage()
        .instance()
        .get(&Symbol::new(&env, "comet_wasm"))
        .unwrap();

    let base_asset: Address = env
        .storage()
        .instance()
        .get(&Symbol::new(&env, "base_asset"))
        .unwrap();

    // TODO I think I want a stronger guarantee that coin_it mints are official. Maybe this should be guarded by the admin signature.

    let sac_deployer = env.deployer().with_stellar_asset(asset_bytes.clone());
    let sac_address: Address;

    if sac_deployer.deployed_address().executable().is_none() {
        sac_address = sac_deployer.deploy();
    } else {
        sac_address = sac_deployer.deployed_address();
    }

    let sac_client = token::StellarAssetClient::new(&env, &sac_address);

    // sac_client.admin().require_auth();

    // Mint tokens to the user (creator)
    let coin_amount = 10_000_000_0000000;

    sac_client.mint(&user, &coin_amount);

    // Pay the mint fee to the admin
    let base_client = token::Client::new(&env, &base_asset);
    let base_amount = 100_0000000;

    base_client.transfer(&user, &sac_client.admin(), &base_amount);

    // Deploy the new AMM pool
    let comet_address = env
        .deployer()
        .with_current_contract(salt.clone())
        .deploy_v2(
            comet_wasm,
            CometArgs::__constructor(
                &sac_client.admin(),
                &vec![&env, sac_address.clone(), base_asset.clone()],
                &vec![&env, 50_00000, 50_00000],
                &vec![&env, 990_000_000_0000000, base_amount],
                &5_00000,        // min fee (5%)
                &95_00000,       // max fee (95%)
                &base_asset,     // tracked token
                &base_amount,    // low util balance
                &70_000_0000000, // high util balance
                &fee_rule,
            ),
        );

    (sac_address, comet_address)
}

mod test;
