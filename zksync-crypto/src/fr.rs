use franklin_crypto::bellman::{Field, PrimeField, PrimeFieldRepr};
use franklin_crypto::bellman::plonk::transparent_engine::Fr;

pub fn hash_to_fr(mut hash: [u8; 32]) -> Fr {
    // TODO: this is a comment from zksync, so what's the final solution?
    // temporary solution, this nullifies top bits to be encoded into field element correctly
    hash[0] &= 0x1f;

    let mut repr = Fr::zero().into_repr();
    repr.read_be(&hash[..]).expect("pack hash as field element");

    Fr::from_repr(repr).unwrap()
}
