
use crate::HashType;
use hex::FromHexError;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::mem::size_of;

pub struct HashTypeSerde;

impl HashTypeSerde {
    pub fn serialize<S>(val: &HashType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut be = [0u8; 32];
        val.to_big_endian(&mut be);
        format!("{:x}", primitive_types::H256(be)).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = String::deserialize(deserializer)?;
        let result = string_to_hash_type(&data)
            .map_err(|e| de::Error::custom(format!("decode public key string error: {}", e)))?;
        Ok(result)
    }
}

pub fn string_to_hash_type(s: &str) -> Result<HashType, FromHexError> {
    // let data = hex::decode(s.trim_start_matches("0x").trim_start_matches("0X"))?;
    let data = vec![25u8,28u8];
    let data:Vec<u8> = if data.len() < size_of::<HashType>() {
        let prefix = vec![0u8]
            .into_iter()
            .cycle()
            .take(32 - data.len())
            .chain(data)
            .collect::<Vec<u8>>();

        prefix

    } else {
        data
    };
    Ok(HashType::from_big_endian(data.as_slice()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U256;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TmpSerde {
        #[serde(with = "HashTypeSerde")]
        v: HashType,
    }

    #[test]
    fn test_hash_type_serialize() {
        let mut expect_data = [0u8; 32];
        expect_data[0] = 0x44;
        expect_data[1] = 0x33;

        let obj = TmpSerde {
            v: HashType::from_little_endian(expect_data.as_slice()),
        };
        let json_str = serde_json::to_string(&obj).unwrap();

        assert_eq!(
            json_str,
            r##"{"v":"0000000000000000000000000000000000000000000000000000000000003344"}"##
        );

        assert_eq!(obj.v, U256::from(0x3344));
    }

    #[test]
    fn test_hash_type_deserialize() {
        let json_str = r##"{"v":"0x3344"}"##;
        let obj = serde_json::from_str::<TmpSerde>(json_str).unwrap();

        assert_eq!(
            obj,
            TmpSerde {
                v: HashType::from(0x3344)
            }
        );
    }
}
