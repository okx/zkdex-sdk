use crate::u8_array_serde::U8Array32SerdeAsStringWith0x;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SignatureOriginal {
    #[serde(rename = "r", with = "U8Array32SerdeAsStringWith0x")]
    pub r: [u8; 32],
    #[serde(rename = "s", with = "U8Array32SerdeAsStringWith0x")]
    pub s: [u8; 32],
}
