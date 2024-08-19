use crate::common::OrderBase;
use crate::crypto::public_key_type::PublicKeyType;
use crate::serde_wrapper::U32SerdeAsString;
use crate::types::{NonceType, TimestampType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct TransferBaseSerde;

impl TransferBaseSerde {
    pub fn serialize<S>(val: &OrderBase, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base = TransferOrderBaseForSerde::from(val);
        base.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OrderBase, D::Error>
    where
        D: Deserializer<'de>,
    {
        let base_for_serde = TransferOrderBaseForSerde::deserialize(deserializer)?;
        Ok(OrderBase::from(base_for_serde))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransferOrderBaseForSerde {
    #[serde(rename = "nonce", with = "U32SerdeAsString")]
    pub nonce: NonceType,
    #[serde(rename = "sender_public_key")]
    pub public_key: PublicKeyType,
    #[serde(rename = "expiration_timestamp", with = "U32SerdeAsString")]
    pub expiration_timestamp: TimestampType,
}

impl From<&OrderBase> for TransferOrderBaseForSerde {
    fn from(value: &OrderBase) -> Self {
        Self {
            nonce: value.nonce,
            public_key: value.public_key.clone(),
            expiration_timestamp: value.expiration_timestamp,
        }
    }
}

impl From<TransferOrderBaseForSerde> for OrderBase {
    fn from(value: TransferOrderBaseForSerde) -> Self {
        // NOTE: the construct of field by field here could also make sure
        // that the code here will be fixed when fields of OrderBase is changed.
        Self {
            nonce: value.nonce,
            public_key: value.public_key,
            expiration_timestamp: value.expiration_timestamp,
        }
    }
}
