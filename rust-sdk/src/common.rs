use serde::{Deserialize, Serialize};

use crate::crypto::public_key_type::PublicKeyType;
use crate::types::{NonceType, TimestampType};
use crate::U32SerdeAsString;

pub const TRANSFER_ORDER_TYPE: u64 = 4;
pub const CONDITIONAL_TRANSFER_ORDER_TYPE: u64 = 5;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OrderBase {
    #[serde(rename = "nonce", with = "U32SerdeAsString")]
    pub nonce: NonceType,
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
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Signature<'a> {
    pub r: &'a str,
    pub s: &'a str,
}
