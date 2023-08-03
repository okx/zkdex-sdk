pub use js_ok_zksync_crypto::params::BLOCK_NUMBER_BIT_WIDTH;
pub use js_ok_zksync_crypto::params::FR_BIT_WIDTH;
use num_bigint::BigInt;
use once_cell::sync::Lazy;

pub const NO_SYNTHETIC_DELTA_ASSET_ID: Lazy<BigInt> = Lazy::new(|| BigInt::from(-1i32));

pub const POSITION_MAX_SUPPORTED_N_ASSETS: usize = 4;

// TODO: original ASSET_ID_BIT_WIDTH is 120, when invalid asset id is -1, 120 bit is not enough.
//pub const ASSET_ID_BIT_WIDTH: usize = 120;
pub const ASSET_ID_BIT_WIDTH: usize = FR_BIT_WIDTH;
pub const SIGNED_MESSAGE_BIT_WIDTH: usize = 251;
pub const ORDER_ID_BIT_WIDTH: usize = 64;
pub const LIMIT_ORDER_WITH_FEES: u8 = 3;

pub const BALANCE_BIT_WIDTH: usize = 64;

pub const POSITION_TREE_DEPTH: usize = 64;
pub const ORDER_TREE_DEPTH: usize = 64;
pub const FUNDING_INDEX_BIT_WIDTH: usize = 64;

// Actual supported block tx sizes by prover and verifier contract
pub const SUPPORTED_BLOCK_TX_VOLUME: &[usize] = &[1];

pub fn position_tree_depth() -> usize {
    POSITION_TREE_DEPTH
}

pub fn order_tree_depth() -> usize {
    ORDER_TREE_DEPTH
}

//the max position num in one transaction circuit of block circuit
//generally maker+taker+fee
pub const TX_CIRCUIT_POSITION_NUM: usize = 3;

//the max order num in one transaction circuit of block circuit
//generally maker+taker
pub const TX_CIRCUIT_ORDER_NUM: usize = 2;

pub const ACCOUNT_ID_BIT_WIDTH: usize = 64;
/// Timestamp bit width
pub const TIMESTAMP_BIT_WIDTH: usize = 8 * 8;

// TODO: is N_TOTAL_ASSETS same as N_ASSETS_UPPER_BOUND, and same as POSITION_MAX_SUPPORTED_N_ASSETS?
pub const N_TOTAL_ASSETS: usize = 4;
pub const N_SIGNERS: usize = 4;

pub static AMOUNT_UPPER_BOUND: Lazy<BigInt> = Lazy::new(|| BigInt::from(2).pow(64));

//todo move to withdraw tx type file
pub const WITHDRAWAL_SIGN_TYPE: u32 = 6;
pub const WITHDRAWAL_TO_OWNER_KEY_SIGN_TYPE: u32 = 7;
