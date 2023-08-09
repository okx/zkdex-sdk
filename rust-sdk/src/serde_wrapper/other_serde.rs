use bigdecimal::BigDecimal;
use num::{bigint::ToBigInt, rational::Ratio, BigUint};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

use crate::convert::*;

/// Trait for specifying prefix for bytes to hex serialization
pub trait Prefix {
    fn prefix() -> &'static str;
}

/// "0x" hex prefix
pub struct ZeroxPrefix;
impl Prefix for ZeroxPrefix {
    fn prefix() -> &'static str {
        "0x"
    }
}

/// Used to annotate `Vec<u8>` fields that you want to serialize like hex-encoded string with prefix
/// Use this struct in annotation like that `[serde(with = "BytesToHexSerde::<T>"]`
/// where T is concrete prefix type (e.g. `SyncBlockPrefix`)
pub struct BytesToHexSerde<P> {
    _marker: std::marker::PhantomData<P>,
}

impl<P: Prefix> BytesToHexSerde<P> {
    pub fn serialize<S>(value: impl AsRef<[u8]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // First, serialize to hexadecimal string.
        let hex_value = format!("{}{}", P::prefix(), hex::encode(value));

        // Then, serialize it using `Serialize` trait implementation for `String`.
        String::serialize(&hex_value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized_string = String::deserialize(deserializer)?;

        if let Some(deserialized_string) = deserialized_string.strip_prefix(P::prefix()) {
            hex::decode(&deserialized_string).map_err(de::Error::custom)
        } else {
            Err(de::Error::custom(format!(
                "string value missing prefix: {:?}",
                P::prefix()
            )))
        }
    }
}

pub type ZeroPrefixHexSerde = BytesToHexSerde<ZeroxPrefix>;

/// Used to serialize BigUint as radix 10 string.
#[derive(Clone, Debug)]
pub struct BigUintSerdeAsRadix10Str;

impl BigUintSerdeAsRadix10Str {
    pub fn serialize<S>(val: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let big_dec = BigDecimal::from(val.to_bigint().unwrap());
        BigDecimal::serialize(&big_dec, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        BigDecimal::deserialize(deserializer).and_then(|bigdecimal| {
            let big_int = bigdecimal
                .to_bigint()
                .ok_or_else(|| Error::custom("Expected integer value"))?;
            big_int
                .to_biguint()
                .ok_or_else(|| Error::custom("Expected positive value"))
        })
    }
}

/// Used to serialize BigUint as radix 10 string.
#[derive(Clone, Debug)]
pub struct BigUintPairSerdeAsRadix10Str;

impl BigUintPairSerdeAsRadix10Str {
    pub fn serialize<S>(pair: &(BigUint, BigUint), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <(BigDecimal, BigDecimal)>::serialize(
            &(
                BigDecimal::from(pair.0.to_bigint().unwrap()),
                BigDecimal::from(pair.1.to_bigint().unwrap()),
            ),
            serializer,
        )
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(BigUint, BigUint), D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let convert = |bigdecimal: BigDecimal| {
            bigdecimal
                .to_bigint()
                .ok_or_else(|| Error::custom("Expected integer value"))?
                .to_biguint()
                .ok_or_else(|| Error::custom("Expected positive value"))
        };

        <(BigDecimal, BigDecimal)>::deserialize(deserializer)
            .and_then(|(a, b)| Ok((convert(a)?, convert(b)?)))
    }
}

#[derive(Clone, Debug)]
pub struct UnsignedRatioSerializeAsDecimal;
impl UnsignedRatioSerializeAsDecimal {
    pub fn serialize<S>(value: &Ratio<BigUint>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        BigDecimal::serialize(&ratio_to_big_decimal(value, 18), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ratio<BigUint>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, deserialize a string value. It is expected to be a
        // hexadecimal representation of `BigDecimal`.
        let big_decimal_string = BigDecimal::deserialize(deserializer)?;

        big_decimal_to_ratio(&big_decimal_string).map_err(de::Error::custom)
    }

    pub fn deserialize_from_str_with_dot(input: &str) -> Result<Ratio<BigUint>, anyhow::Error> {
        big_decimal_to_ratio(&BigDecimal::from_str(input)?)
    }

    pub fn serialize_to_str_with_dot(num: &Ratio<BigUint>, precision: usize) -> String {
        ratio_to_big_decimal(num, precision)
            .to_string()
            .trim_end_matches('0')
            .to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BigUintSerdeWrapper(#[serde(with = "BigUintSerdeAsRadix10Str")] pub BigUint);

impl From<BigUint> for BigUintSerdeWrapper {
    fn from(uint: BigUint) -> BigUintSerdeWrapper {
        BigUintSerdeWrapper(uint)
    }
}

/// Used to annotate `Option<Vec<u8>>` fields that you want to serialize like hex-encoded string with prefix
/// Use this struct in annotation like that `[serde(with = "OptionBytesToHexSerde::<T>"]`
/// where T is concrete prefix type (e.g. `SyncBlockPrefix`)
pub struct OptionBytesToHexSerde<P> {
    _marker: std::marker::PhantomData<P>,
}

impl<P: Prefix> OptionBytesToHexSerde<P> {
    pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // First, serialize to hexadecimal string.
        let hex_value = value
            .as_ref()
            .map(|val| format!("{}{}", P::prefix(), hex::encode(val)));

        // Then, serialize it using `Serialize` trait implementation for `String`.
        Option::serialize(&hex_value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, deserialize a string value. It is expected to be a
        // hexadecimal representation of `Vec<u8>`.
        let optional_deserialized_string: Option<String> = Option::deserialize(deserializer)?;

        optional_deserialized_string
            .map(|s| {
                if let Some(hex_str) = s.strip_prefix(P::prefix()) {
                    hex::decode(hex_str).map_err(de::Error::custom)
                } else {
                    Err(de::Error::custom(format!(
                        "string value missing prefix: {:?}",
                        P::prefix()
                    )))
                }
            })
            .transpose()
    }
}

/// "sync-bl:" hex prefix
pub struct SyncBlockPrefix;
impl Prefix for SyncBlockPrefix {
    fn prefix() -> &'static str {
        "sync-bl:"
    }
}
