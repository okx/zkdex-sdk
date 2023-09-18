use serde::{Deserialize, Serialize};
use zkdex_utils::{I64SerdeAsString, U64SerdeAsString};
use zkdex_wasm::{PublicKeyType, TimestampType};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OrderBase {
    #[serde(rename = "nonce", with = "U64SerdeAsString")]
    pub nonce: u64,
    #[serde(rename = "public_key")]
    pub public_key: PublicKeyType,
    #[serde(rename = "expiration_timestamp", with = "I64SerdeAsString")]
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