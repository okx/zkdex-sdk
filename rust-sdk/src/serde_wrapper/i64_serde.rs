use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

pub struct I64SerdeAsString;

impl I64SerdeAsString {
    pub fn serialize<S>(val: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&val.to_string(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("i64 from string error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64_serialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct I64Serde {
            #[serde(with = "I64SerdeAsString")]
            v: i64,
        }

        let obj = I64Serde { v: 33 };
        let json_str = serde_json::to_string(&obj).unwrap();

        assert_eq!(json_str, r##"{"v":"33"}"##)
    }

    #[test]
    fn test_i64_deserialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct I64Serde {
            #[serde(with = "I64SerdeAsString")]
            v: i64,
        }

        let json_str = r##"{"v":"44"}"##;
        let obj = serde_json::from_str::<I64Serde>(json_str).unwrap();

        assert_eq!(obj, I64Serde { v: 44 });
    }
}
