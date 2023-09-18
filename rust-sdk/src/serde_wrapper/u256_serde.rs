use primitive_types::U256;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub struct U256SerdeAsRadix16Prefix0xString;

impl U256SerdeAsRadix16Prefix0xString {
    pub fn serialize<S>(val: &U256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&format!("0x{:064x}", val), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_str = String::deserialize(deserializer)?;
        U256::from_str_radix(&hex_str, 16)
            .map_err(|e| de::Error::custom(format!("u256 from string error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_u256_serialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U256Serde {
            #[serde(with = "U256SerdeAsRadix16Prefix0xString")]
            v: U256,
        }
        let obj = U256Serde { v: U256::from(33) };
        let json_str = serde_json::to_string(&obj).unwrap();

        assert_eq!(
            json_str,
            r##"{"v":"0x0000000000000000000000000000000000000000000000000000000000000021"}"##
        )
    }

    #[test]
    fn test_u256_deserialize_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U256Serde {
            #[serde(with = "U256SerdeAsRadix16Prefix0xString")]
            v: U256,
        }
        let hex_str = r##"{"v":"0x21"}"##;
        let obj = serde_json::from_str::<U256Serde>(hex_str).unwrap();
        assert_eq!(obj, U256Serde { v: U256::from(33) })
    }
}
