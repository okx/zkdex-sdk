use num_bigint::BigInt;
use num_traits::Num;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

pub struct BigIntSerdeAsRadix10String;

impl BigIntSerdeAsRadix10String {
    pub fn serialize<S>(val: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&val.to_str_radix(10), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
    where
        D: Deserializer<'de>,
    {
        BigInt::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("BigInt from string error: {}", e)))
    }
}

pub struct BigIntSerdeAsRadix16Prefix0xString;

impl BigIntSerdeAsRadix16Prefix0xString {
    pub fn serialize<S>(val: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&bigint_to_prefixed_hex_string(val), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_str = String::deserialize(deserializer)?;
        hex_string_to_bigint(&hex_str)
    }
}

pub struct VecBigIntSerdeAsRadix16Prefix0xString;

impl VecBigIntSerdeAsRadix16Prefix0xString {
    pub fn serialize<S>(val: &Vec<BigInt>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = val
            .iter()
            .map(bigint_to_prefixed_hex_string)
            .collect::<Vec<_>>();

        data.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BigInt>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_str = Vec::<String>::deserialize(deserializer)?;
        hex_str
            .iter()
            .map(|s| hex_string_to_bigint::<D::Error>(s.as_str()))
            .collect::<Result<Vec<_>, _>>()
    }
}

fn bigint_to_prefixed_hex_string(v: &BigInt) -> String {
    let s = v.to_str_radix(16);
    if s.starts_with('-') {
        "-0x".chars().chain(s.chars().skip(1)).collect::<String>()
    } else {
        "0x".chars().chain(s.chars()).collect::<String>()
    }
}

fn hex_string_to_bigint<E: de::Error>(s: &str) -> Result<BigInt, E> {
    let num = BigInt::from_str_radix(
        s.trim_start_matches("0x")
            .trim_start_matches("0X")
            .trim_start_matches("-0x")
            .trim_start_matches("-0X"),
        16,
    )
    .map_err(|e| de::Error::custom(format!("BigInt from string error: {}", e)))?;
    if s.starts_with('-') {
        Ok(-num)
    } else {
        Ok(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_serialize_radix10_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct BigIntSerde {
            #[serde(with = "BigIntSerdeAsRadix10String")]
            v: BigInt,
        }

        let obj = BigIntSerde {
            v: BigInt::from(33),
        };
        let json_str = serde_json::to_string(&obj).unwrap();
        assert_eq!(json_str, r##"{"v":"33"}"##);

        let obj = BigIntSerde {
            v: BigInt::from(-33),
        };
        let json_str = serde_json::to_string(&obj).unwrap();
        assert_eq!(json_str, r##"{"v":"-33"}"##);
    }

    #[test]
    fn test_bigint_deserialize_radix10_string() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct BigIntSerde {
            #[serde(with = "BigIntSerdeAsRadix10String")]
            v: BigInt,
        }

        let json_str = r##"{"v":"33"}"##;
        let obj = serde_json::from_str::<BigIntSerde>(json_str).unwrap();
        assert_eq!(
            obj,
            BigIntSerde {
                v: BigInt::from(33),
            }
        );

        let json_str = r##"{"v":"-33"}"##;
        let obj = serde_json::from_str::<BigIntSerde>(json_str).unwrap();
        assert_eq!(
            obj,
            BigIntSerde {
                v: BigInt::from(-33),
            }
        );
    }

    #[test]
    fn test_bigint_serde_as_radix16_prefix0x_string_serialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct BigInt0xSerde {
            #[serde(with = "BigIntSerdeAsRadix16Prefix0xString")]
            v: BigInt,
        }

        let json_str = serde_json::to_string(&BigInt0xSerde {
            v: BigInt::from(0xabc),
        })
        .unwrap();
        assert_eq!(json_str, format!(r##"{{"v":"0x{:x}"}}"##, 0xabc));

        let json_str = serde_json::to_string(&BigInt0xSerde {
            v: BigInt::from(-0xabc),
        })
        .unwrap();
        assert_eq!(json_str, format!(r##"{{"v":"-0x{:x}"}}"##, 0xabc));
    }

    #[test]
    fn test_bigint_serde_as_radix16_prefix0x_string_deserialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct BigInt0xSerde {
            #[serde(with = "BigIntSerdeAsRadix16Prefix0xString")]
            v: BigInt,
        }

        let json_str = r##"{"v":"0xabc"}"##;
        let obj = serde_json::from_str::<BigInt0xSerde>(json_str).unwrap();
        assert_eq!(
            obj,
            BigInt0xSerde {
                v: BigInt::from(0xabc)
            }
        );

        let json_str = r##"{"v":"-0xabc"}"##;
        let obj = serde_json::from_str::<BigInt0xSerde>(json_str).unwrap();
        assert_eq!(
            obj,
            BigInt0xSerde {
                v: BigInt::from(-0xabc)
            }
        );
    }
}
