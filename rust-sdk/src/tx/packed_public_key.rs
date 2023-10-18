use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

use anyhow::anyhow;
use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::bellman::bn256::Fr;
use franklin_crypto::eddsa::{PrivateKey, PublicKey, Signature};
use franklin_crypto::jubjub::{edwards, FixedGenerators, JubjubEngine};
use pairing_ce::bn256::{Bn256, FrRepr};
use pairing_ce::ff::{PrimeField, PrimeFieldRepr};
use primitive_types::U256;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error as ThisError;

use crate::trim_0x;
use crate::tx::packed_signature::{get_r_from_xy, get_xy_from_r};
use crate::tx::{u256_to_le, JUBJUB_PARAMS};
use crate::zkw::BabyJubjubPoint;

use super::convert::FeConvert;

pub type PrivateKeyType = PrivateKey<Bn256>;

#[derive(Clone, Debug, PartialEq)]
pub struct PackedPublicKey(pub U256);

impl std::fmt::LowerHex for PackedPublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl std::fmt::UpperHex for PackedPublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl ToString for PackedPublicKey {
    fn to_string(&self) -> String {
        self.format_hex(true)
    }
}

impl TryFrom<&str> for PackedPublicKey {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let key = U256::from_str_radix(value, 16)?;

        Ok(Self(key))
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

impl From<BabyJubjubPoint> for PackedPublicKey {
    fn from(value: BabyJubjubPoint) -> Self {
        let r = get_r_from_xy(&value.x, &value.y);

        PackedPublicKey(r)
    }
}

impl Into<U256> for PackedPublicKey {
    fn into(self) -> U256 {
        self.0
    }
}

impl PackedPublicKey {
    pub fn is_address(&self) -> bool {
        return is_address(&self.0);
    }

    pub fn new_address_public_key(address: &str) -> Self {
        PackedPublicKey::try_from(address).unwrap()
    }

    pub fn format_hex(&self, x_prefix: bool) -> String {
        if x_prefix {
            format!("{:#066x}", self)
        } else {
            format!("{:064x}", self)
        }
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
        serializer.serialize_str(&self.format_hex(true))
    }
}

impl<'de> Deserialize<'de> for PackedPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        PackedPublicKey::try_from(string.as_str()).map_err(Error::custom)
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
        <Bn256 as JubjubEngine>::Fs::from_bytes(hex::decode(trim_0x(s))?.as_slice())?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_pubkey_fmt() {
        let packed_key = PackedPublicKey(U256::one() * 10);

        assert_eq!(format!("{:x}", packed_key), "a");
        assert_eq!(format!("{:X}", packed_key), "A");
        assert_eq!(format!("{:#X}", packed_key), "0xA");
        assert_eq!(format!("{:02x}", packed_key), "0a");
        assert_eq!(
            format!("{:#066x}", packed_key),
            "0x000000000000000000000000000000000000000000000000000000000000000a"
        );

        let test_data = vec![
            packed_key,
            PackedPublicKey(U256::MAX),
            PackedPublicKey(U256::zero()),
        ];

        for d in &test_data {
            let mut be_bytes = [0u8; 32];
            d.0.to_big_endian(&mut be_bytes);
            assert_eq!(d.format_hex(false), hex::encode(be_bytes));
            assert_eq!(d.format_hex(true), format!("0x{}", hex::encode(be_bytes)));
            assert_eq!(d.format_hex(true), d.to_string());
        }
    }

    #[test]
    fn test_packed_pubkey_from() {
        let test_data = vec![
            ("", true),
            ("0", true),
            ("0x0000001", true),
            (
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                false,
            ),
        ];

        for (i, (d, ok)) in test_data.into_iter().enumerate() {
            let result = PackedPublicKey::try_from(d);
            assert_eq!(result.is_ok(), ok, "{}", i);
        }
    }

    #[test]
    fn test_packed_pubkey_serde() {
        for _ in 0..10 {
            let pk = random_packed_pubkey();
            let json = serde_json::to_string(&pk).unwrap();
            let pk2: PackedPublicKey = serde_json::from_str(&json).unwrap();

            assert_eq!(pk, pk2);
        }
    }

    fn random_packed_pubkey() -> PackedPublicKey {
        PackedPublicKey(U256([
            rand::random::<u64>(),
            rand::random::<u64>(),
            rand::random::<u64>(),
            rand::random::<u64>(),
        ]))
    }
}
