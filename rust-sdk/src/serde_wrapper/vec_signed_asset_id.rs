use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::transaction::types::SignedAssetId;

pub struct VecSignedAssetIdSerdeAsRadix16String;

impl VecSignedAssetIdSerdeAsRadix16String {
    pub fn serialize<S>(val: &Vec<SignedAssetId>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = val
            .iter()
            .map(|id| format!("0x{:x}", id))
            .collect::<Vec<_>>();

        data.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<SignedAssetId>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_str = Vec::<String>::deserialize(deserializer)?;
        hex_str
            .iter()
            .map(|s| {
                SignedAssetId::from_str_radix(
                    s.trim_start_matches("0x").trim_start_matches("0X"),
                    16,
                )
                .map_err(|e| de::Error::custom(format!("deserialize asset id {} error: {}", s, e)))
            })
            .collect::<Result<Vec<_>, _>>()
    }
}
