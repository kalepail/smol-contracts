#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn init(
        env: soroban_sdk::Env,
        controller: soroban_sdk::Address,
        tokens: soroban_sdk::Vec<soroban_sdk::Address>,
        weights: soroban_sdk::Vec<i128>,
        balances: soroban_sdk::Vec<i128>,
        swap_fee: i128,
    );
    fn gulp(env: soroban_sdk::Env, t: soroban_sdk::Address);
    fn join_pool(
        env: soroban_sdk::Env,
        pool_amount_out: i128,
        max_amounts_in: soroban_sdk::Vec<i128>,
        user: soroban_sdk::Address,
    );
    fn exit_pool(
        env: soroban_sdk::Env,
        pool_amount_in: i128,
        min_amounts_out: soroban_sdk::Vec<i128>,
        user: soroban_sdk::Address,
    );
    fn swap_exact_amount_in(
        env: soroban_sdk::Env,
        token_in: soroban_sdk::Address,
        token_amount_in: i128,
        token_out: soroban_sdk::Address,
        min_amount_out: i128,
        max_price: i128,
        user: soroban_sdk::Address,
    ) -> (i128, i128);
    fn swap_exact_amount_out(
        env: soroban_sdk::Env,
        token_in: soroban_sdk::Address,
        max_amount_in: i128,
        token_out: soroban_sdk::Address,
        token_amount_out: i128,
        max_price: i128,
        user: soroban_sdk::Address,
    ) -> (i128, i128);
    fn dep_tokn_amt_in_get_lp_tokns_out(
        env: soroban_sdk::Env,
        token_in: soroban_sdk::Address,
        token_amount_in: i128,
        min_pool_amount_out: i128,
        user: soroban_sdk::Address,
    ) -> i128;
    fn dep_lp_tokn_amt_out_get_tokn_in(
        env: soroban_sdk::Env,
        token_in: soroban_sdk::Address,
        pool_amount_out: i128,
        max_amount_in: i128,
        user: soroban_sdk::Address,
    ) -> i128;
    fn wdr_tokn_amt_in_get_lp_tokns_out(
        env: soroban_sdk::Env,
        token_out: soroban_sdk::Address,
        pool_amount_in: i128,
        min_amount_out: i128,
        user: soroban_sdk::Address,
    ) -> i128;
    fn wdr_tokn_amt_out_get_lp_tokns_in(
        env: soroban_sdk::Env,
        token_out: soroban_sdk::Address,
        token_amount_out: i128,
        max_pool_amount_in: i128,
        user: soroban_sdk::Address,
    ) -> i128;
    fn set_controller(env: soroban_sdk::Env, manager: soroban_sdk::Address);
    fn set_freeze_status(env: soroban_sdk::Env, val: bool);
    fn get_total_supply(env: soroban_sdk::Env) -> i128;
    fn get_controller(env: soroban_sdk::Env) -> soroban_sdk::Address;
    fn get_tokens(env: soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Address>;
    fn get_balance(env: soroban_sdk::Env, token: soroban_sdk::Address) -> i128;
    fn get_normalized_weight(env: soroban_sdk::Env, token: soroban_sdk::Address) -> i128;
    fn get_spot_price(
        env: soroban_sdk::Env,
        token_in: soroban_sdk::Address,
        token_out: soroban_sdk::Address,
    ) -> i128;
    fn get_swap_fee(env: soroban_sdk::Env) -> i128;
    fn get_spot_price_sans_fee(
        env: soroban_sdk::Env,
        token_in: soroban_sdk::Address,
        token_out: soroban_sdk::Address,
    ) -> i128;
    fn allowance(
        env: soroban_sdk::Env,
        from: soroban_sdk::Address,
        spender: soroban_sdk::Address,
    ) -> i128;
    fn approve(
        env: soroban_sdk::Env,
        from: soroban_sdk::Address,
        spender: soroban_sdk::Address,
        amount: i128,
        expiration_ledger: u32,
    );
    fn balance(env: soroban_sdk::Env, id: soroban_sdk::Address) -> i128;
    fn transfer(
        env: soroban_sdk::Env,
        from: soroban_sdk::Address,
        to: soroban_sdk::MuxedAddress,
        amount: i128,
    );
    fn transfer_from(
        env: soroban_sdk::Env,
        spender: soroban_sdk::Address,
        from: soroban_sdk::Address,
        to: soroban_sdk::Address,
        amount: i128,
    );
    fn burn(env: soroban_sdk::Env, from: soroban_sdk::Address, amount: i128);
    fn burn_from(
        env: soroban_sdk::Env,
        spender: soroban_sdk::Address,
        from: soroban_sdk::Address,
        amount: i128,
    );
    fn decimals(env: soroban_sdk::Env) -> u32;
    fn name(env: soroban_sdk::Env) -> soroban_sdk::String;
    fn symbol(env: soroban_sdk::Env) -> soroban_sdk::String;
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Record {
    pub balance: i128,
    pub index: u32,
    pub scalar: i128,
    pub weight: i128,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceDataKey {
    pub from: soroban_sdk::Address,
    pub spender: soroban_sdk::Address,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: soroban_sdk::String,
    pub symbol: soroban_sdk::String,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKey {
    Factory,
    Controller,
    SwapFee,
    AllTokenVec,
    AllRecordData,
    TokenShare,
    TotalShares,
    PublicSwap,
    Finalize,
    Freeze,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataKeyToken {
    Allowance(AllowanceDataKey),
    Balance(soroban_sdk::Address),
    Nonce(soroban_sdk::Address),
    State(soroban_sdk::Address),
    Admin,
}
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    ErrFinalized = 1,
    ErrNegative = 2,
    ErrMinFee = 3,
    ErrMaxFee = 4,
    ErrNotController = 5,
    ErrInvalidVectorLen = 6,
    AlreadyInitialized = 7,
    ErrIsBound = 8,
    ErrNotBound = 9,
    ErrMaxTokens = 10,
    ErrMinWeight = 11,
    ErrMaxWeight = 12,
    ErrMinBalance = 13,
    ErrFreezeOnlyWithdrawals = 14,
    ErrMinTokens = 15,
    ErrSwapFee = 16,
    ErrMaxInRatio = 17,
    ErrMathApprox = 18,
    ErrLimitIn = 19,
    ErrLimitOut = 20,
    ErrMaxOutRatio = 21,
    ErrBadLimitPrice = 22,
    ErrLimitPrice = 23,
    ErrTotalWeight = 24,
    ErrTokenAmountIsNegative = 25,
    ErrNotAuthorizedByAdmin = 26,
    ErrInsufficientAllowance = 27,
    ErrDeauthorized = 28,
    ErrInsufficientBalance = 29,
    ErrAddOverflow = 30,
    ErrSubUnderflow = 31,
    ErrDivInternal = 32,
    ErrMulOverflow = 33,
    ErrCPowBaseTooLow = 34,
    ErrCPowBaseTooHigh = 35,
    ErrInvalidExpirationLedger = 36,
    ErrNegativeOrZero = 37,
    ErrTokenInvalid = 38,
}
#[soroban_sdk::contractevent(topics = ["swap_event"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwapEvent {
    #[topic]
    pub tag: soroban_sdk::Symbol,
    #[topic]
    pub event: soroban_sdk::Symbol,
    pub caller: soroban_sdk::Address,
    pub token_in: soroban_sdk::Address,
    pub token_out: soroban_sdk::Address,
    pub token_amount_in: i128,
    pub token_amount_out: i128,
}
#[soroban_sdk::contractevent(topics = ["join_event"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct JoinEvent {
    #[topic]
    pub tag: soroban_sdk::Symbol,
    #[topic]
    pub event: soroban_sdk::Symbol,
    pub caller: soroban_sdk::Address,
    pub token_in: soroban_sdk::Address,
    pub token_amount_in: i128,
}
#[soroban_sdk::contractevent(topics = ["exit_event"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExitEvent {
    #[topic]
    pub tag: soroban_sdk::Symbol,
    #[topic]
    pub event: soroban_sdk::Symbol,
    pub caller: soroban_sdk::Address,
    pub token_out: soroban_sdk::Address,
    pub token_amount_out: i128,
}
#[soroban_sdk::contractevent(topics = ["deposit_event"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DepositEvent {
    #[topic]
    pub tag: soroban_sdk::Symbol,
    #[topic]
    pub event: soroban_sdk::Symbol,
    pub caller: soroban_sdk::Address,
    pub token_in: soroban_sdk::Address,
    pub token_amount_in: i128,
}
#[soroban_sdk::contractevent(topics = ["withdraw_event"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WithdrawEvent {
    #[topic]
    pub tag: soroban_sdk::Symbol,
    #[topic]
    pub event: soroban_sdk::Symbol,
    pub caller: soroban_sdk::Address,
    pub token_out: soroban_sdk::Address,
    pub token_amount_out: i128,
    pub pool_amount_in: i128,
}
#[soroban_sdk::contractevent(topics = ["approve"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Approve {
    #[topic]
    pub from: soroban_sdk::Address,
    #[topic]
    pub spender: soroban_sdk::Address,
    pub amount: i128,
    pub expiration_ledger: u32,
}
#[soroban_sdk::contractevent(topics = ["transfer"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TransferWithAmountOnly {
    #[topic]
    pub from: soroban_sdk::Address,
    #[topic]
    pub to: soroban_sdk::Address,
    pub amount: i128,
}
#[soroban_sdk::contractevent(topics = ["transfer"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Transfer {
    #[topic]
    pub from: soroban_sdk::Address,
    #[topic]
    pub to: soroban_sdk::Address,
    pub to_muxed_id: Option<u64>,
    pub amount: i128,
}
#[soroban_sdk::contractevent(topics = ["burn"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Burn {
    #[topic]
    pub from: soroban_sdk::Address,
    pub amount: i128,
}
#[soroban_sdk::contractevent(topics = ["mint"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MintWithAmountOnly {
    #[topic]
    pub to: soroban_sdk::Address,
    pub amount: i128,
}
#[soroban_sdk::contractevent(topics = ["mint"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mint {
    #[topic]
    pub to: soroban_sdk::Address,
    pub to_muxed_id: Option<u64>,
    pub amount: i128,
}
#[soroban_sdk::contractevent(topics = ["clawback"], export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clawback {
    #[topic]
    pub from: soroban_sdk::Address,
    pub amount: i128,
}