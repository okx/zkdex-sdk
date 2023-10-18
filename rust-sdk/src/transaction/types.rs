use primitive_types::U256;

use crate::zkw::JubjubSignature;

pub type AssetIdType = i128;
pub type CollateralAssetId = U256;
pub type SignedAssetId = U256;
pub type PositionIdType = u64;
pub type OrderIdType = u64;
pub type HashType = U256;
pub type IndexType = i128;
pub type ResolutionType = u64;
pub type RiskFactorType = u64;
pub type OraclePriceQuorumType = u64;
pub type TimestampType = u32;
pub type TreeHeightType = u32;
pub type FundingRateType = i128;
pub type PriceType = u128;
pub type AmountType = u64;
pub type BalanceType = i64;
pub type ConditionType = u64;
pub type NonceType = u64;
pub type SignatureType = JubjubSignature;
