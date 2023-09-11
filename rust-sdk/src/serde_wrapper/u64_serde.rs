use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

pub struct U64SerdeAsString;

impl U64SerdeAsString {
    pub fn serialize<S>(val: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&val.to_string(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        u64::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("u64 from string error: {}", e)))
    }
}

pub struct U64SerdeAsRadix16Prefix0xString;

impl U64SerdeAsRadix16Prefix0xString {
    pub fn serialize<S>(val: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&format!("0x{:x}", val), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_str = String::deserialize(deserializer)?;
        let hex_str = hex_str.trim_start_matches("0x").trim_start_matches("0X");
        u64::from_str_radix(hex_str, 16)
            .map_err(|e| de::Error::custom(format!("u64 from string error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_serialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U64Serde {
            #[serde(with = "U64SerdeAsString")]
            v: u64,
        }

        let obj = U64Serde { v: 33 };
        let json_str = serde_json::to_string(&obj).unwrap();

        assert_eq!(json_str, r##"{"v":"33"}"##)
    }

    #[test]
    fn test_u64_deserialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U64Serde {
            #[serde(with = "U64SerdeAsString")]
            v: u64,
        }

        let json_str = r##"{"v":"44"}"##;
        let obj = serde_json::from_str::<U64Serde>(json_str).unwrap();

        assert_eq!(obj, U64Serde { v: 44 });
    }
}
