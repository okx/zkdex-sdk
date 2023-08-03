use crate::params;

use crate::franklin_crypto::bellman::pairing::{
    ff::{Field, PrimeField},
    Engine,
};

use crate::primitives::{GetBits, GetBitsFixed};

/// Representation of one order used in `zkdex_circuit`. id is the tree index
#[derive(Clone, Debug)]
pub struct CircuitOrder<E: Engine> {
    pub is_fulfill: E::Fr,
    //todo other order info?
    // pub pub_key: E::Fr,
}

impl<E: Engine> GetBits for CircuitOrder<E> {
    fn get_bits_le(&self) -> Vec<bool> {
        let mut leaf_content = Vec::new();
        leaf_content.extend(
            self.is_fulfill
                .get_bits_le_fixed(params::ORDER_FULFILL_BIT_WIDTH),
        );
        assert!(
            params::ORDER_FULFILL_BIT_WIDTH < E::Fr::CAPACITY as usize,
            "due to algebraic nature of the hash we should not overflow the capacity"
        );

        leaf_content
    }
}

impl<E: Engine> Default for CircuitOrder<E> {
    //default should be changed: since subtree_root_hash is not zero for all zero balances and subaccounts
    fn default() -> Self {
        Self {
            is_fulfill: E::Fr::zero(),
        }
    }
}
