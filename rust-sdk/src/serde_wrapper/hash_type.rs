use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::HashType;

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
        let result = HashType::from_str(&data)
            .map_err(|e| de::Error::custom(format!("decode public key string error: {}", e)))?;
        Ok(result)
    }
}

pub fn hash_type_to_string_with_0xprefix(hash: HashType) -> String {
    let mut be = [0u8; 32];
    hash.to_big_endian(&mut be);
    format!("0x{:x}", primitive_types::H256(be))
}

#[cfg(test)]
mod tests {
    use primitive_types::U256;

    use super::*;

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

    #[test]
    fn test_hash() {
        let hash = HashType::from_str(
            "0x021fcb6a25c866e3b4168cbdeea385e0481074e18ede1cc9596bfa3a582e0ac8",
        )
        .unwrap();
        assert!(
            &hash_type_to_string_with_0xprefix(hash)
                == "0x021fcb6a25c866e3b4168cbdeea385e0481074e18ede1cc9596bfa3a582e0ac8"
        );
        let hash2 = HashType::from_str(
            "0xe9e95824329ab49e22b0ed11f64a64f45e9736c508fc92341c83dc48defc3525",
        )
        .unwrap();
        assert!(
            &hash_type_to_string_with_0xprefix(hash2)
                == "0xe9e95824329ab49e22b0ed11f64a64f45e9736c508fc92341c83dc48defc3525"
        );
    }
}
