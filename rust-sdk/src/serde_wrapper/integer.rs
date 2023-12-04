use crate::types::{
    AmountType, AssetIdType, FundingRateType, NonceType, OraclePriceQuorumType, PositionIdType,
    PriceType, ResolutionType, RiskFactorType,
};
use crate::types::{SpotAmountType, SpotAssetIdType, SpotPositionIdType, TimestampType};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{marker::PhantomData, num::ParseIntError};

pub type FundingRateTypeSerdeAsRadix10String = SerdeAsString<10, FundingRateType>;
pub type ResolutionTypeSerdeAsRadix16String = SerdeAsString<16, ResolutionType>;
pub type PositionIdTypeSerdeAsRadix10String = SerdeAsString<10, PositionIdType>;
pub type SpotPositionIdTypeSerdeAsRadix10String = SerdeAsString<10, SpotPositionIdType>;
pub type OraclePriceQuorumTypeSerdeAsRadix16String = SerdeAsString<16, OraclePriceQuorumType>;
pub type RiskFactorTypeSerdeAsRadix10String = SerdeAsString<10, RiskFactorType>;
pub type TimestampTypeSerdeAsRadix10String = SerdeAsString<10, TimestampType>;
pub type PriceTypeSerdeAsRadix10String = SerdeAsString<10, PriceType>;
pub type AssetIdTypeSerdeAsRadix16String = SerdeAsString<16, AssetIdType>;
pub type SpotAssetIdTypeSerdeAsRadix16String = SerdeAsString<16, SpotAssetIdType>;
pub type AmountTypeSerdeAsRadix10String = SerdeAsString<10, AmountType>;
pub type SpotAmountTypeSerdeAsRadix10String = SerdeAsString<10, SpotAmountType>;
pub type NonceTypeSerdeAsRadix10String = SerdeAsString<10, NonceType>;

pub struct SerdeAsString<const R: u32, T: SerdeRadix>(PhantomData<T>);

pub trait SerdeRadix: Sized {
    fn to_string(&self, radix: u32) -> String;
    fn from_str(s: &str, radix: u32) -> Result<Self, ParseIntError>;
}

impl<const R: u32, T: SerdeRadix> SerdeAsString<R, T> {
    pub fn serialize<S>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&val.to_string(R), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::from_str(&String::deserialize(deserializer)?, R)
            .map_err(|e| de::Error::custom(format!("FundingRateType from string error: {}", e)))
    }
}

macro_rules! impl_serde_radix {
    ($($ty:ty),+) => {
        $(
            impl SerdeRadix for $ty {
                fn to_string(&self, radix: u32) -> String {
                    if radix == 10 {
                        format!("{}", self)
                    } else if radix == 16 {
                        format!("0x{:x}", self)
                    } else {
                        panic!("unsupported radix")
                    }
                }
                fn from_str(s: &str, radix: u32) -> Result<Self, ParseIntError> {
                    <$ty>::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), radix)
                }
            }
        )+
    };
}

impl_serde_radix!(i32, u32, i64, u64, i128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TmpSerde {
        #[serde(with = "FundingRateTypeSerdeAsRadix10String")]
        v: FundingRateType,
    }

    #[test]
    fn test_fundingratetype_serialize_string() {
        let obj = TmpSerde { v: 33 };
        let json_str = serde_json::to_string(&obj).unwrap();

        assert_eq!(json_str, r##"{"v":"33"}"##)
    }

    #[test]
    fn test_u64_deserialize_string() {
        let json_str = r##"{"v":"44"}"##;
        let obj = serde_json::from_str::<TmpSerde>(json_str).unwrap();

        assert_eq!(obj, TmpSerde { v: 44 });
    }
}
