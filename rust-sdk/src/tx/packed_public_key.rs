use super::convert::FeConvert;
use crate::tx::packed_signature::get_xy_from_r;
use crate::tx::{h256_to_u256, u256_to_le, JUBJUB_PARAMS};
use anyhow::{anyhow, Error};
use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::bellman::bn256::Fr;
use franklin_crypto::eddsa::{PrivateKey, PublicKey, Signature};
use franklin_crypto::jubjub::{edwards, FixedGenerators, JubjubEngine};
use pairing_ce as ef;
use pairing_ce::bn256::{Bn256, FrRepr};
use pairing_ce::ff::{PrimeField, PrimeFieldRepr};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

use crate::zkw::BabyJubjubPoint;
use crate::Engine;
use primitive_types::{H256, U256};
use rand::Rng;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sha2::digest::generic_array::typenum::U25;
use thiserror::{Error as ThisError, Error};

pub type PrivateKeyType = PrivateKey<Bn256>;

#[derive(Clone, Debug)]
pub struct PackedPublicKey(pub U256);

impl ToString for PackedPublicKey {
    fn to_string(&self) -> String {
        let packed_point = self.serialize_packed().unwrap();
        "0x".to_owned() + &hex::encode(packed_point)
    }
}

impl TryFrom<String> for PackedPublicKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let key_str = value.trim_start_matches("0x").trim_start_matches("0X");
        let bytes_vec = if key_str.len() % 2 != 0 {
            // TODO: is there have a more efficient implement?
            hex::decode(String::from("0") + key_str)?
        } else {
            hex::decode(key_str)?
        };

        let mut bytes_array = [0u8; 32];
        let bytes = if bytes_vec.len() < 32 {
            bytes_array[32 - bytes_vec.len()..32].copy_from_slice(&bytes_vec);
            bytes_array.as_slice()
        } else if bytes_vec.len() == 32 {
            bytes_vec.as_slice()
        } else {
            panic!("invalid public key length");
        };
        let packed_point = bytes.to_vec();
        Ok(PackedPublicKey::deserialize_packed(&packed_point)?)
    }
}

impl Into<BabyJubjubPoint> for PackedPublicKey {
    fn into(self) -> BabyJubjubPoint {
        if self.is_address() {
            BabyJubjubPoint {
                x: Default::default(),
                y: self.0.clone(),
            }
        } else {
            let r = &self.0;
            let (x, y) = get_xy_from_r(r);
            let x = fr_to_u256(&x).unwrap();
            let y = fr_to_u256(&y).unwrap();
            BabyJubjubPoint { x, y }
        }
    }
}
impl PackedPublicKey {
    pub fn is_address(&self) -> bool {
        return is_address(&self.0);
    }
    pub fn new_address_public_key(address: String) -> Self {
        PackedPublicKey::try_from(address).unwrap()
    }
    pub fn serialize_packed(&self) -> std::io::Result<Vec<u8>> {
        let mut packed_point = [0; 32];
        self.0.to_big_endian(&mut packed_point);
        Ok(packed_point.to_vec())
    }

    pub fn deserialize_packed(bytes: &[u8]) -> Result<Self, DeserializeError> {
        if bytes.len() != 32 {
            return Err(DeserializeError::IncorrectPublicKeyLength);
        }
        Ok(PackedPublicKey(U256::from_big_endian(&bytes)))
    }
}
pub fn is_address(data: &U256) -> bool {
    let data = u256_to_le(data);
    let suffix_slice: &[u8] = &data[20..];
    return if suffix_slice == [0; 12] && &data != &[0; 32] {
        true
    } else {
        false
    };
}

#[derive(Debug, ThisError)]
pub enum DeserializeError {
    #[error("Public key size mismatch")]
    IncorrectPublicKeyLength,
    #[error("Failed to restore point: {0}")]
    RestoreCurvePoint(std::io::Error),
}

impl Serialize for PackedPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let packed_point = self.serialize_packed().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&hex::encode(packed_point))
    }
}

impl<'de> Deserialize<'de> for PackedPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        PackedPublicKey::try_from(string).map_err(Error::custom)
    }
}

pub fn public_key_from_private_with_verify(
    pk: &PrivateKey<Bn256>,
    msg: &[u8],
    sig: &Signature<Bn256>,
) -> PackedPublicKey {
    let pubkey = PublicKey::from_private(pk, FixedGenerators::SpendingKeyGenerator, &JUBJUB_PARAMS);
    let aaa = pubkey.clone();
    let point = pubkey.0;
    let mut packed_point = [0u8; 32];
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let a = aaa.verify_for_raw_message(msg, sig, p_g, &JUBJUB_PARAMS, msg.len());
    assert_eq!(a, true);
    point.write(packed_point.as_mut()).unwrap();
    PackedPublicKey(U256::from_little_endian(&packed_point))
}

pub fn public_key_from_private(pk: &PrivateKey<Bn256>) -> PackedPublicKey {
    let pubkey = PublicKey::from_private(pk, FixedGenerators::SpendingKeyGenerator, &JUBJUB_PARAMS);
    let point = pubkey.0;
    let mut packed_point = [0u8; 32];
    point.write(packed_point.as_mut()).unwrap();
    PackedPublicKey(U256::from_little_endian(&packed_point))
}

pub fn fr_to_u256(fr: &Fr) -> Result<U256, anyhow::Error> {
    let repr = fr.into_repr();
    let mut buf = [0u8; 32];
    repr.write_le(&mut buf[..])
        .map_err(|e| anyhow!(e.to_string()))?;
    Ok(U256::from_little_endian(&buf))
}

pub fn u256_to_fr(u: &U256) -> Result<Fr, anyhow::Error> {
    let mut s_repr = FrRepr::default();
    s_repr
        .read_le(&u256_to_le(u)[..])
        .map_err(|e| anyhow!(e.to_string()))?;
    let s = Fr::from_repr(s_repr)?;
    Ok(s)
}

pub fn convert_to_pubkey(x: &U256, y: &U256) -> Result<PublicKey<Bn256>, anyhow::Error> {
    let x = u256_to_fr(x)?;
    let y = u256_to_fr(y)?;
    let point = edwards::Point::from_xy(x, y, &JUBJUB_PARAMS as &AltJubjubBn256).ok_or(anyhow!(
        String::from("could not decode public key by x and y")
    ))?;
    Ok(PublicKey(point))
}

impl From<(U256, U256)> for PackedPublicKey {
    fn from(value: (U256, U256)) -> Self {
        if value.0 == U256::zero() && is_address(&value.1) {
            return PackedPublicKey(value.1);
        }
        let pubkey = convert_to_pubkey(&value.0, &value.1).unwrap();
        let point = pubkey.0;
        let mut packed_point = [0u8; 32];
        point.write(packed_point.as_mut()).unwrap();
        PackedPublicKey(U256::from_little_endian(&packed_point))
    }
}

pub fn new_private_key() -> PrivateKey<Bn256> {
    let mut rng = rand::thread_rng();
    PrivateKey::<Bn256>(rng.gen())
}

pub fn private_key_to_string(pk: &PrivateKeyType) -> String {
    pk.0.to_hex()
}

pub fn private_key_from_string(s: &str) -> Result<PrivateKeyType, anyhow::Error> {
    Ok(PrivateKey::<Bn256>(
        <Bn256 as JubjubEngine>::Fs::from_bytes(
            hex::decode(s.trim_start_matches("0x").trim_start_matches("0X"))?.as_slice(),
        )?,
    ))
}
