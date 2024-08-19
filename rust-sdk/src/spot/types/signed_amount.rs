use crate::spot::AmountType;

#[derive(Debug, Default, Clone, PartialEq)]
#[repr(C)]
pub struct SignedAmountType {
    pub amount: AmountType,
    pub is_neg: bool,
}

#[cfg(feature = "notwasm")]
pub use native_env::*;

#[cfg(feature = "notwasm")]
mod native_env {
    use super::*;
    use num_traits::Zero;
    use serde::de::Error;
    use serde::{Deserializer, Serializer};
    use std::fmt::Display;

    impl serde::Serialize for SignedAmountType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&format!("{}", self), serializer)
        }
    }

    impl<'de> serde::Deserialize<'de> for SignedAmountType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let str = String::deserialize(deserializer)?;
            let mut str = str.trim_start();

            let is_neg;
            if str.starts_with("-") {
                is_neg = true;
                str = &str[1..];
            } else {
                is_neg = false;
            }

            let num = u64::from_str_radix(str, 10).map_err(|e| {
                D::Error::custom(format!("SignedAmountType from string error: {}", e))
            })?;
            Ok(Self {
                is_neg,
                amount: num,
            })
        }
    }

    impl From<i64> for SignedAmountType {
        fn from(value: i64) -> Self {
            Self {
                is_neg: value.is_negative(),
                amount: (value.abs() as u64).into(),
            }
        }
    }

    impl From<u64> for SignedAmountType {
        fn from(value: u64) -> Self {
            Self {
                is_neg: false,
                amount: value.into(),
            }
        }
    }

    impl Display for SignedAmountType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let str = if !self.is_neg || self.amount.is_zero() {
                self.amount.to_string()
            } else {
                format!("-{}", self.amount)
            };
            write!(f, "{}", str)
        }
    }
}
