use serde::{de, Deserialize, Deserializer, Serializer};

pub mod serde_str {
    use super::*;
    use std::fmt::Display;
    use std::str::FromStr;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod serde_hex_str {
    use super::*;
    use crate::trim_0x;
    use std::fmt::Display;
    use std::fmt::LowerHex;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display + LowerHex,
        S: Serializer,
    {
        serializer.serialize_str(format!("0x{:x}", value).as_str())
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: num_traits::Num,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let s = trim_0x(&s);

        T::from_str_radix(&s, 16)
            .map_err(|_e| de::Error::custom(format!("from_str_radix with 16 err, data: {}", s)))
    }
}

pub mod serde_hex_str_vec {
    use super::*;
    use crate::trim_0x;
    use std::fmt::Display;
    use std::fmt::LowerHex;

    pub fn serialize<T, S>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display + LowerHex,
        S: Serializer,
    {
        serializer.collect_seq(value.iter().map(|v| format!("0x{:x}", v)))
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: num_traits::Num,
        D: Deserializer<'de>,
    {
        let ss = Vec::<String>::deserialize(deserializer)?;

        ss.iter()
            .map(|s| {
                let s = trim_0x(&s);
                Ok(T::from_str_radix(&s, 16).map_err(|_e| {
                    de::Error::custom(format!("from_str_radix with 16 err, data: {}", s))
                })?)
            })
            .collect::<Result<Vec<_>, _>>()
    }
}
