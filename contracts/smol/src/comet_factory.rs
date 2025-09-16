#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn init(env: soroban_sdk::Env, pool_wasm_hash: soroban_sdk::BytesN<32>);
    fn new_c_pool(
        env: soroban_sdk::Env,
        salt: soroban_sdk::BytesN<32>,
        controller: soroban_sdk::Address,
        tokens: soroban_sdk::Vec<soroban_sdk::Address>,
        weights: soroban_sdk::Vec<i128>,
        balances: soroban_sdk::Vec<i128>,
        swap_fee: i128,
    ) -> soroban_sdk::Address;
    fn is_c_pool(env: soroban_sdk::Env, addr: soroban_sdk::Address) -> bool;
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKeyFactory {
    IsCpool(soroban_sdk::Address),
    WasmHash,
}
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    ErrNotCPool = 1,
    ErrNotController = 5,
    AlreadyInitialized = 7,
}
#[soroban_sdk::contractevent(topics = ["new_pool_event"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NewPoolEvent {
    #[topic]
    pub tag: soroban_sdk::Symbol,
    #[topic]
    pub event: soroban_sdk::Symbol,
    pub caller: soroban_sdk::Address,
    pub pool: soroban_sdk::Address,
}
