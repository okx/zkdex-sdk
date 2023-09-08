use std::fmt::{Debug, Formatter};
use std::thread::sleep;
use std::time::Duration;

use ef::ff::{PrimeField, PrimeFieldRepr};
use franklin_crypto::alt_babyjubjub::fs::{Fs, FsRepr};
use franklin_crypto::eddsa::{PrivateKey, PublicKey, Seed, Signature};
use franklin_crypto::jubjub::{FixedGenerators, JubjubEngine};
use pairing_ce as ef;
use pairing_ce::bn256::Bn256;
use primitive_types::{H256, U256};
use rand::{Rng, SeedableRng, XorShiftRng};
use time::OffsetDateTime;

use crate::tx::{h256_to_u256, JUBJUB_PARAMS, le_to_u256, u256_to_h256};
use crate::tx::convert::FeConvert;
use crate::tx::packed_public_key::{fr_to_u256, PackedPublicKey, public_key_from_private, public_key_from_private_with_verify};
use crate::tx::packed_signature::{PackedSignature, point_from_xy};
use crate::zkw::{BabyJubjubPoint, JubjubSignature};

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
        let sig_s = le_to_u256(&packed_signature);
        JubjubSignature {
            sig_r,
            sig_s: sig_s.0,
        }
    }
}

impl From<JubjubSignature> for PackedSignature {
    fn from(value: JubjubSignature) -> Self {
        let r = point_from_xy(&value.sig_r.x, &value.sig_r.y);
        let s = u256_to_h256(U256(value.sig_s)).0;
        let mut fspr = FsRepr::default();
        fspr.read_le(&s[..]).unwrap();
        let s = Fs::from_repr(fspr).unwrap();
        PackedSignature { 0: Signature { r: r, s: s } }
    }
}

impl TxSignature {
    pub fn sign_msg(pk: &PrivateKey<Bn256>, msg: &[u8]) -> (TxSignature, PackedPublicKey) {
        let ret = Self::sign_raw(pk, msg);
        let pubkey = ret.pub_key.clone();
        (ret, pubkey)
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

    pub fn verify(&self, pk: &PublicKey<Bn256>, msg: &[u8]) -> bool {
        self.signature.verify(pk, msg)
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
    use std::str::FromStr;
    use crate::felt::LeBytesConvert;

    use crate::tx;
    use crate::tx::packed_public_key::private_key_from_string;
    use crate::tx::{HashType};

    use super::*;

    #[test]
    pub fn test_sign() {
        let key = gen_test_pk();
        let msg = [
            57, 157, 225, 12, 118, 179, 210, 146, 126, 70, 97, 155, 39, 53, 69, 99, 133, 171, 101,
            205, 154, 123, 60, 47, 41, 171, 133, 216, 161, 228, 205, 32,
        ];
        let sig = TxSignature::sign_msg(&key, msg.as_slice());
        let pub_key = PublicKey::from_private(&key, FixedGenerators::SpendingKeyGenerator, &JUBJUB_PARAMS);

        assert!(sig.0.verify(&pub_key, &msg));
        let a1 = sig.0.signature.clone();
        let a2 = PackedSignature::from(<tx::sign::TxSignature as Into<JubjubSignature>>::into(sig.0));
        println!("{:#?}", a1);
        println!("{:#?}", a2);
    }

    #[test]
    pub fn test_verify_from_poc() {
        let hash = HashType::from_str("0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520").unwrap();
        let sig = JubjubSignature::from_str("353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6",
                                            "c80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502");
        let pack_sig = PackedSignature::from(sig);


        let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let pub_key = PublicKey::from_private(&private_key, FixedGenerators::SpendingKeyGenerator, &JUBJUB_PARAMS);
        // assert!(pack_sig.verify(&pub_key,hash.as_bytes()));
        let hash2 = HashType::from_str("0x01817ed5bea1d0082c0fbe02edb06c15f52e2bb98c2b92f36d160ab082f1a520").unwrap();
        assert_eq!(pack_sig.verify(&pub_key, hash.as_le_bytes()), true);
        assert_eq!(pack_sig.verify(&pub_key, hash2.as_le_bytes()), false);


        // assert_eq!(pack_sig.verify(&pubkey.0, hash.as_le_bytes()), true);
        // println!("{:#?}", pubkey);
        // println!("{:#?}", pubkey);
    }

    #[test]
    pub fn test_pubkey() {
        let pubkey = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";

    }
}
