use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

pub struct U32SerdeAsString;

impl U32SerdeAsString {
    pub fn serialize<S>(val: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&val.to_string(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        u32::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("u64 from string error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_serialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U32Serde {
            #[serde(with = "U32SerdeAsString")]
            v: u32,
        }

        let obj = U32Serde { v: 33 };
        let json_str = serde_json::to_string(&obj).unwrap();

        assert_eq!(json_str, r##"{"v":"33"}"##)
    }

    #[test]
    fn test_u32_deserialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U32Serde {
            #[serde(with = "U32SerdeAsString")]
            v: u32,
        }

        let json_str = r##"{"v":"44"}"##;
        let obj = serde_json::from_str::<U32Serde>(json_str).unwrap();

        assert_eq!(obj, U32Serde { v: 44 });
    }
}
