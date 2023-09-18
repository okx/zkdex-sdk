use crate::signature_serde::SignatureOriginal;
use crate::trim_0x;
use hex::FromHex;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub struct U8Array32SerdeAsStringWith0x;

impl U8Array32SerdeAsStringWith0x {
    const ARRAY_LEN: usize = 32;
    const HEX_STR_MAX_LEN: usize = Self::ARRAY_LEN * 2;

    pub fn serialize<S>(val: &[u8; Self::ARRAY_LEN], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&format!("0x{}", hex::encode(val.as_slice())), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; Self::ARRAY_LEN], D::Error>
    where
        D: Deserializer<'de>,
    {
        let array_str = String::deserialize(deserializer)?;
        let array_str = trim_0x(&array_str);

        assert!(array_str.len() <= Self::HEX_STR_MAX_LEN);
        if array_str.len() < Self::HEX_STR_MAX_LEN {
            let hex = IntoIterator::into_iter(['0'])
                .cycle()
                .take(Self::HEX_STR_MAX_LEN - array_str.len())
                .chain(array_str.chars())
                .collect::<String>();
            // let hex = ['0']
            //     .into_iter()
            //     .cycle()
            //     .take(Self::HEX_STR_MAX_LEN - array_str.len())
            //     // .chain(array_str.chars())
            //     .collect::<String>();
            Self::decode::<D>(&hex)
        } else {
            Self::decode::<D>(array_str)
        }
    }

    fn decode<'de, D>(hex: &str) -> Result<[u8; Self::ARRAY_LEN], D::Error>
    where
        D: Deserializer<'de>,
    {
        <[u8; Self::ARRAY_LEN]>::from_hex(hex)
            .map_err(|e| de::Error::custom(format!("decode hex string error: {}", e)))
    }
}

// serde of [u8;64] as signature value.
pub struct U8Array64SignatureSerde;

impl U8Array64SignatureSerde {
    pub fn serialize<S>(val: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sign = SignatureOriginal::default();
        sign.r.copy_from_slice(&val[..32]);
        sign.s.copy_from_slice(&val[32..]);

        SignatureOriginal::serialize(&sign, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let sign = SignatureOriginal::deserialize(deserializer)?;

        let mut result = [0; 64];
        result[..32].copy_from_slice(&sign.r);
        result[32..].copy_from_slice(&sign.s);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8array32_serde_as_string_with_0x_serialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U8Array32Serde {
            #[serde(with = "U8Array32SerdeAsStringWith0x")]
            v: [u8; 32],
        }

        let array = [22u8; 32];
        let json_str = serde_json::to_string(&U8Array32Serde { v: array }).unwrap();
        assert_eq!(
            json_str,
            format!(r##"{{"v":"0x{}"}}"##, hex::encode(array.as_slice()))
        );
    }

    #[test]
    fn test_u8array32_serde_as_string_with_0x_deserialize() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U8Array32Serde {
            #[serde(with = "U8Array32SerdeAsStringWith0x")]
            v: [u8; 32],
        }

        let array = [22u8; 32];
        let json_str = format!(r##"{{"v":"0x{}"}}"##, hex::encode(array.as_slice()));
        let obj = serde_json::from_str::<U8Array32Serde>(&json_str).unwrap();
        assert_eq!(obj, U8Array32Serde { v: array });
    }

    #[test]
    fn test_u8array32_serde_as_string_less_than_32_bytes() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct U8Array32Serde {
            #[serde(with = "U8Array32SerdeAsStringWith0x")]
            v: [u8; 32],
        }

        let mut expect = [22u8; 32];
        expect[0] = 0;
        expect[1] = 0;

        let array = [22u8; 30];
        let json_str = format!(r##"{{"v":"{}"}}"##, hex::encode(array.as_slice()));
        let obj = serde_json::from_str::<U8Array32Serde>(&json_str).unwrap();
        assert_eq!(obj, U8Array32Serde { v: expect });
    }

    #[test]
    fn test_hex() {
        let a = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 8, 8, 8, 8, 8, 8, 9, 255, 6, 7, 6, 4, 3,
            1, 0, 3, 4,
        ];
        let s = hex::encode(a.as_slice());
        println!("{s}");
    }
}
