use crate::trim_0x;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub struct I128SerdeAsRadix16Prefix0xString;

impl I128SerdeAsRadix16Prefix0xString {
    pub fn serialize<S>(val: &i128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&format!("0x{:x}", val), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i128, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_str = String::deserialize(deserializer)?;
        let hex_str = trim_0x(&hex_str);
        i128::from_str_radix(hex_str, 16)
            .map_err(|e| de::Error::custom(format!("decode hex string error: {}", e)))
    }
}

pub struct U128SerdeAsString;

impl U128SerdeAsString {
    pub fn serialize<S>(val: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&format!("{}", val), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        u128::from_str_radix(&str, 10)
            .map_err(|e| de::Error::custom(format!("decode u128 string error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_i128_serialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct I128Serde {
            #[serde(with = "I128SerdeAsRadix16Prefix0xString")]
            v: i128,
        }

        let obj = I128Serde { v: 33 };
        let json_str = serde_json::to_string(&obj).unwrap();

        assert_eq!(json_str, r##"{"v":"0x21"}"##)
    }

    #[test]
    fn test_i128_deserialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct I128Serde {
            #[serde(with = "I128SerdeAsRadix16Prefix0xString")]
            v: i128,
        }

        let json_str = r##"{"v":"0x21"}"##;
        let obj = serde_json::from_str::<I128Serde>(json_str).unwrap();

        assert_eq!(obj, I128Serde { v: 33 });
    }
}
