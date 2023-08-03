use crate::merkle_tree::rescue_hasher::BabyRescueHasher;
use franklin_crypto::{alt_babyjubjub::AltJubjubBn256, rescue::bn256::Bn256RescueParams};
use lazy_static::lazy_static;

// pub const ACCOUNT_TREE_DEPTH: usize = 32;
/// Depth of the balance tree for each account.
/// TODO
pub const BALANCE_TREE_DEPTH: usize = 4;

//TODO
pub const ORDER_TREE_DEPTH: usize = 4;

// pub const BALANCE_TREE_DEPTH: usize = 32;
/// Version of transactions.
pub const CURRENT_TX_VERSION: u8 = 1;

/// balance tree_depth.
pub fn balance_tree_depth() -> usize {
    BALANCE_TREE_DEPTH
}

/// Tokens settings

/// Number of supported tokens.
pub fn total_tokens() -> usize {
    2usize.pow((balance_tree_depth() - 1) as u32) - 2
}

//TODO
// pub const PROCESSABLE_TOKENS_DEPTH: u32 = 10;
pub const PROCESSABLE_TOKENS_DEPTH: u32 = 2;
/// Number of tokens that are processed by this release
pub fn number_of_processable_tokens() -> usize {
    let num = 2usize.pow(PROCESSABLE_TOKENS_DEPTH);

    assert!(num <= total_tokens());
    assert!(num.is_power_of_two());

    num
}

pub const ACCOUNT_ID_BIT_WIDTH: usize = 32;

pub const INPUT_DATA_ADDRESS_BYTES_WIDTH: usize = 32;
pub const INPUT_DATA_BLOCK_NUMBER_BYTES_WIDTH: usize = 32;
pub const INPUT_DATA_FEE_ACC_BYTES_WIDTH_WITH_EMPTY_OFFSET: usize = 32;
pub const INPUT_DATA_FEE_ACC_BYTES_WIDTH: usize = 3;
pub const INPUT_DATA_ROOT_BYTES_WIDTH: usize = 32;
pub const INPUT_DATA_EMPTY_BYTES_WIDTH: usize = 64;
pub const INPUT_DATA_ROOT_HASH_BYTES_WIDTH: usize = 32;

pub const LEGACY_TOKEN_BIT_WIDTH: usize = 16;
pub const TOKEN_BIT_WIDTH: usize = 32;
pub const TX_TYPE_BIT_WIDTH: usize = 8;

pub const TX_VERSION_FOR_SIGNATURE_BIT_WIDTH: usize = 8;

/// Account subtree hash width
pub const SUBTREE_HASH_WIDTH: usize = 254; //seems to be equal to Bn256::NUM_BITS could be replaced
pub const SUBTREE_HASH_WIDTH_PADDED: usize = 256;

/// Content hash size
pub const CONTENT_HASH_WIDTH: usize = 256;

/// NFT serial id size
pub const SERIAL_ID_WIDTH: usize = 32;

/// balance bit width
pub const BALANCE_BIT_WIDTH: usize = 64;

pub const NEW_PUBKEY_HASH_WIDTH: usize = FR_ADDRESS_LEN * 8;
pub const ADDRESS_WIDTH: usize = FR_ADDRESS_LEN * 8;
/// Nonce bit width
pub const NONCE_BIT_WIDTH: usize = 32;

/// order fulfil bit width
pub const ORDER_FULFILL_BIT_WIDTH: usize = 1;

/// time stamp  bit width //todo
pub const TIME_STAMP_BIT_WIDTH: usize = 128;

//
pub const LEGACY_CHUNK_BIT_WIDTH: usize = 72;
pub const LEGACY_CHUNK_BYTES: usize = LEGACY_CHUNK_BIT_WIDTH / 8;

pub const CHUNK_BIT_WIDTH: usize = 80;
pub const CHUNK_BYTES: usize = CHUNK_BIT_WIDTH / 8;

pub const MAX_CIRCUIT_MSG_HASH_BITS: usize = 736;

pub const ETH_ADDRESS_BIT_WIDTH: usize = 160;
/// Block number bit width
pub const BLOCK_NUMBER_BIT_WIDTH: usize = 32;

/// Amount bit widths
pub const AMOUNT_EXPONENT_BIT_WIDTH: usize = 5;
pub const AMOUNT_MANTISSA_BIT_WIDTH: usize = 35;

/// Fee bit widths
pub const FEE_EXPONENT_BIT_WIDTH: usize = 5;
pub const FEE_MANTISSA_BIT_WIDTH: usize = 11;

/// Timestamp bit width
pub const TIMESTAMP_BIT_WIDTH: usize = 8 * 8;

pub const PRICE_BIT_WIDTH: usize = 120;

// Signature data
pub const SIGNATURE_S_BIT_WIDTH: usize = 254;
pub const SIGNATURE_S_BIT_WIDTH_PADDED: usize = 256;
pub const SIGNATURE_R_X_BIT_WIDTH: usize = 254;
pub const SIGNATURE_R_Y_BIT_WIDTH: usize = 254;
pub const SIGNATURE_R_BIT_WIDTH_PADDED: usize = 256;

// Fr element encoding
pub const FR_BIT_WIDTH: usize = 254;
pub const FR_BIT_WIDTH_PADDED: usize = 256;
pub const FR_BYTE_WIDTH_PADDED: usize = FR_BIT_WIDTH_PADDED / 8;

pub const LEAF_DATA_BIT_WIDTH: usize =
    NONCE_BIT_WIDTH + NEW_PUBKEY_HASH_WIDTH + FR_BIT_WIDTH_PADDED + ETH_ADDRESS_BIT_WIDTH;

/// Priority op should be executed for this number of eth blocks.
pub const PRIORITY_EXPIRATION: u64 = 35000; // TODO: Check that in the future this constant cannot cause unexpected behavior (ZKS-520).
pub const FR_ADDRESS_LEN: usize = 20;

pub const PAD_MSG_BEFORE_HASH_BITS_LEN: usize = 736;

/// Size of the data that is signed for withdraw tx
pub const SIGNED_WITHDRAW_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + TX_VERSION_FOR_SIGNATURE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + 2 * ADDRESS_WIDTH
    + TOKEN_BIT_WIDTH
    + BALANCE_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for withdraw tx, without timestamps, and with 2-byte token representation
pub const OLD1_SIGNED_WITHDRAW_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + 2 * ADDRESS_WIDTH
    + LEGACY_TOKEN_BIT_WIDTH
    + BALANCE_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH;

/// Size of the data that is signed for withdraw tx, with timestamps, but with 2-byte token representation
pub const OLD2_SIGNED_WITHDRAW_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + 2 * ADDRESS_WIDTH
    + LEGACY_TOKEN_BIT_WIDTH
    + BALANCE_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for transfer tx
pub const SIGNED_TRANSFER_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + TX_VERSION_FOR_SIGNATURE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + 2 * ADDRESS_WIDTH
    + TOKEN_BIT_WIDTH
    + AMOUNT_EXPONENT_BIT_WIDTH
    + AMOUNT_MANTISSA_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for transfer tx, without timestamps, and with 2-byte token representation
pub const OLD1_SIGNED_TRANSFER_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + 2 * ADDRESS_WIDTH
    + LEGACY_TOKEN_BIT_WIDTH
    + AMOUNT_EXPONENT_BIT_WIDTH
    + AMOUNT_MANTISSA_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH;

/// Size of the data that is signed for transfer tx, with timestamps, but with 2-byte token representation
pub const OLD2_SIGNED_TRANSFER_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + 2 * ADDRESS_WIDTH
    + LEGACY_TOKEN_BIT_WIDTH
    + AMOUNT_EXPONENT_BIT_WIDTH
    + AMOUNT_MANTISSA_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for forced exit tx
pub const SIGNED_FORCED_EXIT_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + TX_VERSION_FOR_SIGNATURE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + ADDRESS_WIDTH
    + TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for forced exit tx with 2-byte token representation
pub const OLD_SIGNED_FORCED_EXIT_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + ADDRESS_WIDTH
    + LEGACY_TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for mint nft tx
pub const SIGNED_MINT_NFT_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + TX_VERSION_FOR_SIGNATURE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + ADDRESS_WIDTH
    + CONTENT_HASH_WIDTH
    + ADDRESS_WIDTH
    + TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH;

/// Size of the data that is signed for withdraw nft tx
pub const SIGNED_WITHDRAW_NFT_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + TX_VERSION_FOR_SIGNATURE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + ADDRESS_WIDTH
    + ADDRESS_WIDTH
    + TOKEN_BIT_WIDTH
    + TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for change pubkey tx
pub const SIGNED_CHANGE_PUBKEY_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + TX_VERSION_FOR_SIGNATURE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + ADDRESS_WIDTH
    + NEW_PUBKEY_HASH_WIDTH
    + TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Size of the data that is signed for change pubkey tx, without timestamps, and with 2-byte token representation
pub const OLD1_SIGNED_CHANGE_PUBKEY_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + ADDRESS_WIDTH
    + NEW_PUBKEY_HASH_WIDTH
    + LEGACY_TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH;

/// Size of the data that is signed for change pubkey tx, with timestamps, but with 2-byte token representation
pub const OLD2_SIGNED_CHANGE_PUBKEY_BIT_WIDTH: usize = TX_TYPE_BIT_WIDTH
    + ACCOUNT_ID_BIT_WIDTH
    + ADDRESS_WIDTH
    + NEW_PUBKEY_HASH_WIDTH
    + LEGACY_TOKEN_BIT_WIDTH
    + FEE_EXPONENT_BIT_WIDTH
    + FEE_MANTISSA_BIT_WIDTH
    + NONCE_BIT_WIDTH
    + 2 * TIMESTAMP_BIT_WIDTH;

/// Number of inputs in the basic circuit that is aggregated by recursive circuit
pub const RECURSIVE_CIRCUIT_NUM_INPUTS: usize = 1;
/// Depth of the tree which contains different verification keys for basic circuit
pub const RECURSIVE_CIRCUIT_VK_TREE_DEPTH: usize = 3;

/// Major version of the ZkSync
pub const ZKSYNC_VERSION: &str = "contracts-6";

lazy_static! {
    pub static ref JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub static ref RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
    pub static ref RESCUE_HASHER: BabyRescueHasher = BabyRescueHasher::default();
    /////  Special address for the account used in the nft logic
    // pub static ref NFT_STORAGE_ACCOUNT_ADDRESS: Address =
    //     Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_it_works() {}
}
