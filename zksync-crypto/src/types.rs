use primitive_types::{H256, U256};

pub fn u256_to_h256(u: &U256) -> H256 {
    let mut h = [0u8; 32];
    u.to_little_endian(&mut h[..]);
    H256(h)
}

pub fn h256_to_u256(h: &H256) -> U256 {
    U256::from_little_endian(&h[..])
}