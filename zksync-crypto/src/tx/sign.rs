use crate::tx::convert::FeConvert;
use crate::tx::packed_public_key::{
    fr_to_u256, public_key_from_private, public_key_from_private_with_verify, PackedPublicKey,
};
use crate::tx::packed_signature::PackedSignature;
use crate::tx::{h256_to_u256, JUBJUB_PARAMS};
use franklin_crypto::alt_babyjubjub::fs::Fs;
use franklin_crypto::eddsa::{PrivateKey, Seed};
use franklin_crypto::jubjub::FixedGenerators;
use pairing_ce::bn256::Bn256;
use primitive_types::H256;
use rand::{Rng, SeedableRng, XorShiftRng};
use std::fmt::{Debug, Formatter};
use std::thread::sleep;
use std::time::Duration;
use pairing_ce as ef;
use ef::ff::{PrimeField, PrimeFieldRepr};
use time::OffsetDateTime;
use zkwasm_rust_sdk::{BabyJubjubPoint, JubjubSignature};

/// zkSync transaction signature.
///
/// Represents a MuSig Rescue signature for the message.
#[derive(Clone)]
pub struct TxSignature {
    pub pub_key: PackedPublicKey,
    pub signature: PackedSignature,
}

impl Debug for TxSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TxSignature")
            .field("pub_key", &self.pub_key)
            .field("signature", &self.signature)
            .finish()
    }
}
impl Into<JubjubSignature> for TxSignature {
    fn into(self) -> JubjubSignature {
        let (x, y) = self.signature.0.r.into_xy();
        let sig_r = BabyJubjubPoint {
            x: fr_to_u256(&x).unwrap(),
            y: fr_to_u256(&y).unwrap(),
        };
        let mut packed_signature = [0u8; 32];
        let s_bar = packed_signature.as_mut();
        self.signature.0.s.into_repr().write_le(s_bar).unwrap();
        let sig_s = h256_to_u256(H256(packed_signature));
        JubjubSignature {
            sig_r,
            sig_s: sig_s.0,
        }
    }
}

impl TxSignature {
    pub fn sign_msg(pk: &PrivateKey<Bn256>, msg: &[u8]) -> (JubjubSignature, PackedPublicKey) {
        let ret = Self::sign_raw(pk, msg);
        let pubkey = ret.pub_key.clone();
        (ret.into(), pubkey)
    }
    pub fn sign_raw(pk: &PrivateKey<Bn256>, hash_msg: &[u8]) -> Self {
        let seed = Seed::deterministic_seed(pk, &hash_msg);
        let signature = pk.sign_raw_message(
            &hash_msg,
            &seed,
            FixedGenerators::SpendingKeyGenerator,
            &JUBJUB_PARAMS,
            hash_msg.len(), /* usize */
        );
        Self {
            pub_key: public_key_from_private_with_verify(pk, hash_msg, &signature),
            signature: PackedSignature(signature),
        }
    }
}

pub fn gen_test_pk() -> PrivateKey<Bn256> {
    let ss = "0x057afe7e950189b17eedfd749f5537a88eb3ed4981467636a115e5c3efcce0f4";
    PrivateKey::<Bn256>(Fs::from_bytes(&*hex::decode(&ss[2..]).unwrap()).unwrap())
}
pub fn gen_random_key() -> PrivateKey<Bn256> {
    sleep(Duration::from_millis(100));
    let ts_nanos = OffsetDateTime::now_utc().unix_timestamp_nanos();
    let mut rng = XorShiftRng::from_seed([
        (ts_nanos as u32 & u32::MAX),
        ((ts_nanos >> 32) as u32) & u32::MAX,
        ((ts_nanos >> 64) as u32) & u32::MAX,
        ((ts_nanos >> 96) as u32) & u32::MAX,
    ]);
    PrivateKey(rng.gen())
}
pub fn gen_couple() -> (PrivateKey<Bn256>, PackedPublicKey) {
    let key = gen_random_key();
    let pubkey = public_key_from_private(&key);
    (key, pubkey)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_sign() {
        let key = gen_test_pk();
        let msg = [
            57, 157, 225, 12, 118, 179, 210, 146, 126, 70, 97, 155, 39, 53, 69, 99, 133, 171, 101,
            205, 154, 123, 60, 47, 41, 171, 133, 216, 161, 228, 205, 32,
        ];
        let sig = TxSignature::sign_msg(&key, msg.as_slice());
        println!("s: {:?}", sig.0.sig_s);
        println!("x: {:?}", sig.0.sig_r.x.0);
        println!("y: {:?}", sig.0.sig_r.y.0);
        let _pk = sig.1.clone();

        let pubkey: BabyJubjubPoint = sig.1.into();
        println!("pub_x:{:?}", pubkey.x.0);
        println!("pub_y:{:?}", pubkey.y.0);
        let msg = h256_to_u256(H256(msg)).0;
        println!("msg:{:?}", msg);
    }
}
