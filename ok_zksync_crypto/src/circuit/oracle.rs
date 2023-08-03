use crate::params;

use crate::franklin_crypto::bellman::pairing::{
    ff::{Field, PrimeField},
    Engine,
};

use crate::primitives::{GetBits, GetBitsFixed};

pub struct OraclePrice<E: Engine> {
    pub asset_id: E::Fr,
    pub funding_index: E::Fr,
}

/// Representation of one order used in `zkdex_circuit`. id is the tree index
pub struct OraclePrices<E: Engine> {
    pub prices: Vec<OraclePrice<E>>,
    pub timestamp: E::Fr, // TODO: rename this field
}

impl<E: Engine> GetBits for OraclePrices<E> {
    fn get_bits_le(&self) -> Vec<bool> {
        let mut leaf_content = Vec::new();
        leaf_content.extend(
            self.timestamp
                .get_bits_le_fixed(params::TIME_STAMP_BIT_WIDTH),
        );
        assert!(
            params::TIME_STAMP_BIT_WIDTH < E::Fr::CAPACITY as usize,
            "due to algebraic nature of the hash we should not overflow the capacity"
        );

        leaf_content
    }
}

impl<E: Engine> std::default::Default for OraclePrices<E> {
    //default should be changed: since subtree_root_hash is not zero for all zero balances and subaccounts
    fn default() -> Self {
        Self {
            timestamp: E::Fr::zero(),
            prices: Vec::new(),
        }
    }
}
