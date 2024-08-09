pub use liquidate::*;
pub use oracle::*;
pub use perpetual_trade::*;
pub use spot_trade::*;
pub use transfer::*;
pub use withdrawal::*;

pub(crate) mod hash_trait;
mod liquidate;
mod oracle;
pub mod order;
mod perpetual_trade;
pub(crate) mod sign_trait;
mod spot_trade;
mod transfer;
mod withdrawal;

#[cfg(test)]
mod test {
    use crate::hash_type::hash_type_to_string_with_0xprefix;
    use crate::tx::packed_public_key::private_key_from_string;
    use crate::unified::transactions::hash_trait::HashTrait;
    use crate::unified::transactions::sign_trait::SignTrait;
    use crate::verify_jubjub_signature;

    pub const PRI_KEY: &str = "0x01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
    pub const PUB_KEY: &str = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
    pub fn sign_and_verify(tx: impl SignTrait + HashTrait) {
        let pri_key = private_key_from_string(PRI_KEY).unwrap();
        let sig = tx.sign(&pri_key);

        let msg = tx.hash();
        let verify =
            verify_jubjub_signature(sig.into(), PUB_KEY, &hash_type_to_string_with_0xprefix(msg));
        assert!(verify.is_ok());
        assert!(verify.unwrap());
    }
}
