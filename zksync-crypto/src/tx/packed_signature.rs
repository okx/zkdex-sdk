use crate::tx::packed_public_key::h256_to_fr;
use crate::tx::{h256_to_u256, u256_to_h256, JUBJUB_PARAMS};
use crate::U8Array32SerdeAsStringWith0x;
use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::eddsa::Signature;
use franklin_crypto::jubjub::edwards::Point;
use franklin_crypto::jubjub::{edwards, Unknown};
use pairing_ce::bn256::{Bn256, Fr};
use pairing_ce as ef;
use primitive_types::{H256, U256};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use thiserror::Error;
use crate::zkw::{BabyJubjubPoint, JubjubSignature};

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
    #[serde(rename = "r", with = "U8Array32SerdeAsStringWith0x")]
    pub r: [u8; 32],
    #[serde(rename = "s", with = "U8Array32SerdeAsStringWith0x")]
    pub s: [u8; 32],
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

impl SignatureSerde {
    pub fn serialize<S>(val: &JubjubSignature, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut r = [0u8; 32];
        let r_point = point_from_xy(&val.sig_r.x, &val.sig_r.y);
        r_point.write(r.as_mut()).unwrap();

        let s = u256_to_h256(U256(val.sig_s)).0;
        let sign = SignatureOriginal { r, s };

        SignatureOriginal::serialize(&sign, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<JubjubSignature, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sign = SignatureOriginal::deserialize(deserializer)?;

        let (x, y) = get_xy_from_r(sign.r);
        use ef::ff::PrimeField;
        let x_repr = x.into_repr();
        let y_repr = y.into_repr();

        Ok(JubjubSignature {
            sig_r: BabyJubjubPoint {
                x: U256(x_repr.0),
                y: U256(y_repr.0),
            },
            sig_s: h256_to_u256(H256(sign.s)).0,
        })
    }
}
pub fn get_xy_from_r(r_bar: [u8; 32]) -> (Fr, Fr) {
    let r: Point<Bn256, Unknown> =
        edwards::Point::read(r_bar.as_slice(), &JUBJUB_PARAMS as &AltJubjubBn256)
            .map_err(DeserializeError::RestoreRPoint)
            .unwrap();
    r.into_xy()
}
pub fn get_r_from_xy(x: &U256, y: &U256) -> [u8; 32] {
    let point = point_from_xy(x, y);
    let mut packed_point = [0u8; 32];
    point.write(packed_point.as_mut()).unwrap();
    packed_point
}

fn point_from_xy(x: &U256, y: &U256) -> Point<Bn256, Unknown> {
    let x = h256_to_fr(u256_to_h256(x.clone())).unwrap();
    let y = h256_to_fr(u256_to_h256(y.clone())).unwrap();

    Point::from_xy(x, y, &JUBJUB_PARAMS as &AltJubjubBn256).unwrap()
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use super::*;
    use crate::tx::packed_public_key::fr_to_u256;
    use serde::{Deserialize, Serialize};

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
        let (x, y) = get_xy_from_r(r);
        let x = fr_to_u256(&x).unwrap();
        let y = fr_to_u256(&y).unwrap();

        let base = OrderBase {
            nonce: 1,
            signature: JubjubSignature {
                sig_r: BabyJubjubPoint { x, y },
                sig_s: h256_to_u256(H256(s)).0,
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
