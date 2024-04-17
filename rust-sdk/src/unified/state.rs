use crate::unified::*;

#[cfg(feature = "notwasm")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "notwasm", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SharedState {
    #[cfg_attr(feature = "notwasm", serde(with = "serde_hex_str"))]
    pub positions_root: U256,
    #[cfg_attr(feature = "notwasm", serde(with = "serde_hex_str"))]
    pub orders_root: U256,
    pub global_funding_indices: FundingIndicesInfo,
    pub oracle_prices: OraclePrices,
    #[cfg_attr(feature = "notwasm", serde(with = "serde_str"))]
    pub system_time: TimestampType,
}
