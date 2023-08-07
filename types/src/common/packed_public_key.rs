use std::fmt::Debug;

use lazy_static::lazy_static;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use js_ok_zksync_crypto::{
    franklin_crypto::{
        alt_babyjubjub::{edwards, AltJubjubBn256},
        eddsa::PublicKey,
    },
    params::JUBJUB_PARAMS,
    Engine,
};

lazy_static! {
    // default point
    pub static ref DEFAULT_POINT: PublicKeyType = PublicKeyType::default();
}

#[derive(Clone)]
pub struct PublicKeyType(pub PublicKey<Engine>);

impl Debug for PublicKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let hex_pk = hex::encode(&self.serialize_packed().unwrap());
        write!(f, "{{{}}}", hex_pk)
    }
}

impl PublicKeyType {
    pub fn serialize_packed(&self) -> std::io::Result<Vec<u8>> {
        let mut packed_point = [0u8; 32];
        (self.0).0.write(packed_point.as_mut())?;
        Ok(packed_point.to_vec())
    }

    pub fn deserialize_packed(bytes: &[u8]) -> Result<Self, DeserializeError> {
        if bytes.len() != 32 {
            return Err(DeserializeError::IncorrectPublicKeyLength);
        }
        Ok(PublicKeyType(PublicKey::<Engine>(
            edwards::Point::read(&*bytes, &JUBJUB_PARAMS as &AltJubjubBn256)
                .map_err(DeserializeError::RestoreCurvePoint)?,
        )))
    }

    pub fn deserialize_str(s: &str) -> Result<Self, de::value::Error> {
        let key_str = s.trim_start_matches("0x").trim_start_matches("0X");
        let bytes_vec = if key_str.len() % 2 != 0 {
            // TODO: is there have a more efficient implement?
            hex::decode(String::from("0") + key_str).map_err(de::Error::custom)?
        } else {
            hex::decode(key_str).map_err(de::Error::custom)?
        };

        let mut bytes_array = [0u8; 32];
        let bytes = if bytes_vec.len() < 32 {
            bytes_array[(32 - bytes_vec.len())..].copy_from_slice(&bytes_vec);
            bytes_array.as_slice()
        } else if bytes_vec.len() == 32 {
            bytes_vec.as_slice()
        } else {
            return Err(de::Error::custom(format!(
                "too long public key string: '{}'",
                s
            )));
        };

        Self::deserialize_packed(bytes).map_err(de::Error::custom)
    }

    // pub fn verify_musig_rescue(&self, signature: &SignatureType, hash: &[u8]) -> bool {
    //     return self.0.verify_musig_rescue(
    //         &hash,
    //         &signature.0 .0,
    //         FixedGenerators::SpendingKeyGenerator,
    //         &RESCUE_PARAMS,
    //         &JUBJUB_PARAMS,
    //     );
    // }

    pub fn empty(&self) -> bool {
        return self.eq(&PublicKeyType::default());
    }
}

impl PartialEq for PublicKeyType {
    fn eq(&self, other: &PublicKeyType) -> bool {
        self.serialize_packed().unwrap() == other.serialize_packed().unwrap()
    }
}

impl Default for PublicKeyType {
    fn default() -> Self {
        let bytes: &[u8] = &[0u8; 32];
        PublicKeyType(PublicKey::<Engine>(
            edwards::Point::read(&*bytes, &JUBJUB_PARAMS as &AltJubjubBn256).unwrap(),
        ))
    }
}

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Public key size mismatch")]
    IncorrectPublicKeyLength,
    #[error("Failed to restore point: {0}")]
    RestoreCurvePoint(std::io::Error),
}

impl Serialize for PublicKeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let packed_point = self.serialize_packed().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&hex::encode(packed_point))
    }
}

impl<'de> Deserialize<'de> for PublicKeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let string = String::deserialize(deserializer)?;

        Self::deserialize_str(&string).map_err(Error::custom)
    }
}
