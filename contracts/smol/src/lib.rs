#![no_std]

use soroban_sdk::{
    contract, contractimpl, token,
    xdr::{
        AccountId, AlphaNum12, Asset, AssetCode12, FromXdr, Limits, PublicKey, Uint256, WriteXdr,
    },
    Address, Bytes, BytesN, Env, Symbol,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, smol_issuer: BytesN<32>, comet_factory: Address) {
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
    }
    pub fn mint(env: Env, user: Address) -> Address {
        user.require_auth();

        // Get the smol_issuer address
        let smol_issuer: BytesN<32> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "smol_issuer"))
            .unwrap();

        let mut items = [0u8; 44];
        items[0..12].copy_from_slice(&[0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0]);
        items[12..].copy_from_slice(&smol_issuer.to_array());

        Address::from_xdr(&env, &Bytes::from_slice(&env, &items))
            .unwrap()
            .require_auth();

        // Get the token_counter number
        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "token_counter"))
            .unwrap();

        let asset = Asset::CreditAlphanum12(AlphaNum12 {
            asset_code: counter_to_ascii(counter),
            issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                smol_issuer.to_array(),
            ))),
        });

        let asset_bytes = Bytes::from_slice(&env, &asset.to_xdr(Limits::none()).unwrap());

        let sac_address = env.deployer().with_stellar_asset(asset_bytes).deploy();

        // Initialize the SAC token with the smol_issuer as admin
        let sac_client = token::StellarAssetClient::new(&env, &sac_address);

        sac_client.set_admin(&env.current_contract_address());

        // Mint 1M tokens to the user (creator)
        let amount = 1_000_000_0000000i128; // 1M tokens with 7 decimal places

        sac_client.mint(&user, &amount);

        // Increment the token_counter
        counter += 1;

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "token_counter"), &counter);

        sac_address
    }
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

mod test;
