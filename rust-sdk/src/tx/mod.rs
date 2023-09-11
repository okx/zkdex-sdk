pub mod convert;
pub mod packed_public_key;
pub mod packed_signature;
pub mod public_key_type;
pub mod sign;

pub use crate::*;
use franklin_crypto::{alt_babyjubjub::AltJubjubBn256, rescue::bn256::Bn256RescueParams};
use lazy_static::lazy_static;
use primitive_types::{H256, U256};

lazy_static! {
    pub static ref JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub static ref RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

pub fn h256_to_u256(h: H256) -> U256 {
    U256::from_little_endian(&h[..])
}

// TODO:
pub fn u256_to_h256(u: U256) -> H256 {
    let mut h = [0u8; 32];
    u.to_little_endian(&mut h[..]);
    H256(h)
}

pub fn le_to_u256(h: &[u8; 32]) -> U256 {
    U256::from_little_endian(&h[..])
}

pub fn u256_to_le(u: &U256) -> [u8; 32] {
    let mut h = [0u8; 32];
    u.to_little_endian(&mut h[..]);
    h
}
