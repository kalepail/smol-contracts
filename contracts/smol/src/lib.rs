#![no_std]

use comet_factory::Client as CometFactoryClient;
use soroban_sdk::{
    contract, contractimpl, token, vec,
    xdr::{ FromXdr },
    Address, Bytes, BytesN, Env, Symbol,
};

mod comet_factory;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(
        env: Env,
        smol_issuer: BytesN<32>,
        comet_factory: Address,
        base_asset: Address,
    ) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "comet_factory"), &comet_factory);

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "smol_issuer"), &smol_issuer);

        // Initialize token counter starting from 0 (will generate 000000000000 first)
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "token_counter"), &0u64);

        // Base asset
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "base_asset"), &base_asset);
    }
    pub fn mint(env: Env, user: Address, asset_bytes: Bytes) -> (Address, Address) {
        user.require_auth();

        let comet_factory: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "comet_factory"))
            .unwrap();

        // Get the smol_issuer address
        let smol_issuer: BytesN<32> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "smol_issuer"))
            .unwrap();

        let base_asset: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "base_asset"))
            .unwrap();

        let mut items = [0u8; 44];
        items[0..12].copy_from_slice(&[0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0]);
        items[12..].copy_from_slice(&smol_issuer.to_array());

        let controller = Address::from_xdr(&env, &Bytes::from_slice(&env, &items)).unwrap();

        controller.require_auth();

        // Get the token_counter number
        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "token_counter"))
            .unwrap();

        let sac_deployer = env.deployer().with_stellar_asset(asset_bytes);
        let sac_address: Address;

        if sac_deployer.deployed_address().executable().is_none() {
            sac_address = sac_deployer.deploy();
        } else {
            sac_address = sac_deployer.deployed_address();
        }

        let sac_client = token::StellarAssetClient::new(&env, &sac_address);

        // Mint 1M tokens to the user (creator)
        let amount = 1_000_000_0000000; // 1M tokens with 7 decimal places

        sac_client.mint(&user, &amount);

        // TODO deploy new AMM contract
        let factory_client = CometFactoryClient::new(&env, &comet_factory);

        let mut salt = [0u8; 32];
        salt[24..].copy_from_slice(&counter.to_be_bytes());

        // Open the new AMM pool
        let comet_address = factory_client.new_c_pool(
            &BytesN::from_array(&env, &salt),
            &controller,
            &vec![&env, sac_address.clone(), base_asset],
            &vec![&env, 80_00000, 20_00000],
            &vec![&env, 99_000_000_0000000, 100_0000000],
            &10,
        );

        // Increment the token_counter
        counter += 1;

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "token_counter"), &counter);

        (sac_address, comet_address)
    }
}

mod test;
