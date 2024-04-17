mod asset_id;
pub mod chain_id;
mod macros;
mod position_id;
mod signed_amount;

pub use asset_id::*;
pub use position_id::*;
pub use signed_amount::*;

// TODO: reconsider the types
pub type AmountType = u64;
//pub type IndexType = i64;
pub type TimestampType = u32;
pub type PriceType = u64;
pub type ExternalPriceType = u128;
//pub type ResolutionType = u64;
//pub type RiskFactorType = u64;
//pub type FundingRateType = u64;
//pub type OraclePriceQuorumType = u64;
