#![no_std]

use comet_factory::Client as CometFactoryClient;
use soroban_sdk::{contract, contractimpl, token, vec, Address, Bytes, Env, Symbol};

mod comet_factory;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, comet_factory: Address, base_asset: Address) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "comet_factory"), &comet_factory);

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

        let base_asset: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "base_asset"))
            .unwrap();

        let sac_deployer = env.deployer().with_stellar_asset(asset_bytes.clone());
        let sac_address: Address;

        if sac_deployer.deployed_address().executable().is_none() {
            sac_address = sac_deployer.deploy();
        } else {
            sac_address = sac_deployer.deployed_address();
        }

        let sac_client = token::StellarAssetClient::new(&env, &sac_address);

        sac_client.admin().require_auth();

        // Mint 1M tokens to the user (creator)
        let amount = 1_000_000_0000000; // 1M tokens with 7 decimal places

        sac_client.mint(&user, &amount);

        // TODO deploy new AMM contract
        let factory_client = CometFactoryClient::new(&env, &comet_factory);

        // Open the new AMM pool
        let comet_address = factory_client.new_c_pool(
            &env.crypto().sha256(&asset_bytes).to_bytes(),
            &sac_client.admin(),
            &vec![&env, sac_address.clone(), base_asset],
            &vec![&env, 80_00000, 20_00000],
            &vec![&env, 99_000_000_0000000, 100_0000000],
            &10,
        );

        (sac_address, comet_address)
    }
}

mod test;
