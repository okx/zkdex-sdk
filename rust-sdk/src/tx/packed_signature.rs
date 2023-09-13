use std::fmt::{Debug, Formatter};

use franklin_crypto::alt_babyjubjub::{AltJubjubBn256, FixedGenerators};
use franklin_crypto::bellman::PrimeField;
use franklin_crypto::eddsa::{PublicKey, Signature};
use franklin_crypto::jubjub::edwards::Point;
use franklin_crypto::jubjub::{edwards, Unknown};
use pairing_ce::bn256::{Bn256, Fr};
use primitive_types::U256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::tx::packed_public_key::u256_to_fr;
use crate::tx::{le_to_u256, u256_to_le, JUBJUB_PARAMS};
use crate::zkw::{BabyJubjubPoint, JubjubSignature};
use crate::U256SerdeAsRadix16Prefix0xString;

pub struct SignatureSerde;

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Signature length should be 64 bytes")]
    IncorrectSignatureLength,
    #[error("Failed to restore R point from R_bar: {0}")]
    RestoreRPoint(std::io::Error),
    #[error("Cannot read S scalar: {0}")]
    ReadS(std::io::Error),

    #[error("Cannot read S scalar: {0}")]
    Unknown(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SignatureOriginal {
    #[serde(rename = "r", with = "U256SerdeAsRadix16Prefix0xString")]
    pub r: U256,
    #[serde(rename = "s", with = "U256SerdeAsRadix16Prefix0xString")]
    pub s: U256,
}

#[derive(Clone)]
pub struct PackedSignature(pub Signature<Bn256>);
impl Debug for PackedSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.0.r.into_xy();
        let mut msg = format!("x:{:?},", x);
        msg.push_str(&format!("y:{:?},", y));
        msg.push_str(&format!("s:{:?}", self.0.s));
        f.write_str(&msg)
    }
}

impl PackedSignature {
    pub fn verify(&self, pk: &PublicKey<Bn256>, msg: &[u8]) -> bool {
        let p_g = FixedGenerators::SpendingKeyGenerator;
        pk.verify_for_raw_message(msg, &self.0, p_g, &JUBJUB_PARAMS, msg.len())
    }
}

impl Serialize for JubjubSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut r = [0u8; 32];
        let r_point = point_from_xy(&self.sig_r.x, &self.sig_r.y);
        r_point.write(r.as_mut()).unwrap();

        let r = le_to_u256(&r);
        let s = U256(self.sig_s);
        let sign = SignatureOriginal { r, s };

        SignatureOriginal::serialize(&sign, serializer)
    }
}

impl SignatureSerde {
    pub fn serialize<S>(val: &JubjubSignature, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut r = [0u8; 32];
        let r_point = point_from_xy(&val.sig_r.x, &val.sig_r.y);
        r_point.write(r.as_mut()).unwrap();

        let r = le_to_u256(&r);
        let s = U256(val.sig_s);
        let sign = SignatureOriginal { r, s };

        SignatureOriginal::serialize(&sign, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<JubjubSignature, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sign = SignatureOriginal::deserialize(deserializer)?;

        Ok(signature_from_rs(&sign.r, &sign.s))
    }
}

pub fn signature_from_rs(r: &U256, s: &U256) -> JubjubSignature {
    let (x, y) = get_xy_from_r(r);

    let x_repr = x.into_repr();
    let y_repr = y.into_repr();

    JubjubSignature {
        sig_r: BabyJubjubPoint {
            x: U256(x_repr.0),
            y: U256(y_repr.0),
        },
        sig_s: s.0.clone(),
    }
}

pub fn get_xy_from_r(r_bar: &U256) -> (Fr, Fr) {
    let r_bar = u256_to_le(&r_bar);
    let r: Point<Bn256, Unknown> =
        edwards::Point::read(r_bar.as_slice(), &JUBJUB_PARAMS as &AltJubjubBn256)
            .map_err(DeserializeError::RestoreRPoint)
            .unwrap();
    r.into_xy()
}
pub fn get_r_from_xy(x: &U256, y: &U256) -> U256 {
    let point = point_from_xy(x, y);
    let mut packed_point = [0u8; 32];
    point.write(packed_point.as_mut()).unwrap();
    le_to_u256(&packed_point)
}

pub(crate) fn point_from_xy(x: &U256, y: &U256) -> Point<Bn256, Unknown> {
    let x = u256_to_fr(x).unwrap();
    let y = u256_to_fr(y).unwrap();

    Point::from_xy(x, y, &JUBJUB_PARAMS as &AltJubjubBn256).unwrap()
}

impl JubjubSignature {
    pub fn from_str(r: &str, s: &str) -> Self {
        let r_str = r.trim_start_matches("0x").trim_start_matches("0X");
        let r = U256::from_str_radix(r_str, 16).unwrap();

        let s_str = s.trim_start_matches("0x").trim_start_matches("0X");
        let s = U256::from_str_radix(s_str, 16).unwrap();
        signature_from_rs(&r, &s)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::convert::TryInto;

    use crate::tx::packed_public_key::fr_to_u256;

    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct OrderBase {
        pub nonce: u64,
        #[serde(rename = "signature", with = "SignatureSerde")]
        pub signature: JubjubSignature,
    }
    pub const SIGNATURE_1_0_R: &str =
        "5f00b6c207a8235426f6df1b3eab83a228bc711908b9536f51f34cae820e7c25";
    pub const SIGNATURE_1_0_S: &str =
        "f3fd87e986f383ea42342ed293f90351baece370d03fc082caccbfed419c0705";
    #[test]
    pub fn test_serialize_deserialize() {
        let r: [u8; 32] = hex::decode(&String::from(SIGNATURE_1_0_R))
            .unwrap()
            .try_into()
            .unwrap();
        let s: [u8; 32] = hex::decode(SIGNATURE_1_0_S).unwrap().try_into().unwrap();
        let (x, y) = get_xy_from_r(&le_to_u256(&r));
        let x = fr_to_u256(&x).unwrap();
        let y = fr_to_u256(&y).unwrap();

        let base = OrderBase {
            nonce: 1,
            signature: JubjubSignature {
                sig_r: BabyJubjubPoint { x, y },
                sig_s: le_to_u256(&s).0,
            },
        };
        let data = serde_json::to_vec(&base).unwrap();
        println!("{:?}", data);
        let base2: OrderBase = serde_json::from_slice(data.as_slice()).unwrap();
        assert_eq!(base.signature.sig_s, base2.signature.sig_s);
        assert_eq!(base.signature.sig_r.x, base2.signature.sig_r.x);
        assert_eq!(base.signature.sig_r.y, base2.signature.sig_r.y);
    }
}
