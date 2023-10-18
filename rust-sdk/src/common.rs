use serde::{Deserialize, Serialize};

use crate::tx::public_key_type::PublicKeyType;
use crate::U32SerdeAsString;
use crate::U64SerdeAsString;

pub const TRANSFER_ORDER_TYPE: u64 = 4;
pub const CONDITIONAL_TRANSFER_ORDER_TYPE: u64 = 5;

pub type TimestampType = u32;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OrderBase {
    #[serde(rename = "nonce", with = "U64SerdeAsString")]
    pub nonce: u64,
    #[serde(rename = "public_key")]
    pub public_key: PublicKeyType,
    #[serde(rename = "expiration_timestamp", with = "U32SerdeAsString")]
    pub expiration_timestamp: TimestampType,
}

impl Default for OrderBase {
    fn default() -> Self {
        Self {
            nonce: 0,
            public_key: Default::default(),
            expiration_timestamp: 0,
            // signature: None,
        }
    }
}

// #[derive(Clone, Debug)]
// pub struct OffsetDateTimeSerdeAsTimeStampStr;
//
// impl OffsetDateTimeSerdeAsTimeStampStr {
//     pub fn serialize<S>(val: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let ts_str = format!("{}", val.unix_timestamp());
//         String::serialize(&ts_str, serializer)
//     }
//
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let ts_str = String::deserialize(deserializer)?;
//         let ts = i64::from_str(&ts_str)
//             .map_err(|e| de::Error::custom(format!("string to i64 error: {}", e)))?;
//         OffsetDateTime::from_unix_timestamp(ts).map_err(|e| {
//             de::Error::custom(format!("timestamp {} to OffsetDateTime error: {}", ts, e))
//         })
//     }
// }
//
// pub struct BigIntSerdeAsRadix10String;
//
// impl BigIntSerdeAsRadix10String {
//     pub fn serialize<S>(val: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         String::serialize(&val.to_str_radix(10), serializer)
//     }
//
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         BigInt::from_str(&String::deserialize(deserializer)?)
//             .map_err(|e| de::Error::custom(format!("BigInt from string error: {}", e)))
//     }
// }
//
// pub struct BigIntSerdeAsRadix16Prefix0xString;
//
// impl BigIntSerdeAsRadix16Prefix0xString {
//     pub fn serialize<S>(val: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         String::serialize(&bigint_to_prefixed_hex_string(val), serializer)
//     }
//
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let hex_str = String::deserialize(deserializer)?;
//         hex_string_to_bigint(&hex_str)
//     }
// }
// fn bigint_to_prefixed_hex_string(v: &BigInt) -> String {
//     let s = v.to_str_radix(16);
//     if s.starts_with('-') {
//         "-0x".chars().chain(s.chars().skip(1)).collect::<String>()
//     } else {
//         "0x".chars().chain(s.chars()).collect::<String>()
//     }
// }
//
// fn hex_string_to_bigint<E: de::Error>(s: &str) -> Result<BigInt, E> {
//     let num = BigInt::from_str_radix(
//         s.trim_start_matches("0x")
//             .trim_start_matches("0X")
//             .trim_start_matches("-0x")
//             .trim_start_matches("-0X"),
//         16,
//     )
//     .map_err(|e| de::Error::custom(format!("BigInt from string error: {}", e)))?;
//     if s.starts_with('-') {
//         Ok(-num)
//     } else {
//         Ok(num)
//     }
// }

// pub struct U64SerdeAsString;
//
// impl U64SerdeAsString {
//     pub fn serialize<S>(val: &u64, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         String::serialize(&val.to_string(), serializer)
//     }
//
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         u64::from_str(&String::deserialize(deserializer)?)
//             .map_err(|e| de::Error::custom(format!("u64 from string error: {}", e)))
//     }
// }

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type")]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum Transaction {
//     #[serde(rename = "TRANSFER")]
//     Transfer(Box<TransferRequest>),
//     #[serde(rename = "WITHDRAWAL_TO_ADDRESS")]
//     Withdrawal(Box<WithdrawRequest>),
//     #[serde(rename = "DELEVERAGE")]
//     Deleverage(Box<Deleverage>),
//     // ConditionalTransfer(Box<ConditionalTransfer>),
// }
