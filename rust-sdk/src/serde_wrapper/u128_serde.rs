use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
pub struct U128SerdeAsRadix16Prefix0xString;

impl U128SerdeAsRadix16Prefix0xString {
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
