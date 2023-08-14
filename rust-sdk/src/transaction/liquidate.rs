use serde::{Deserialize, Serialize};
use crate::transaction::limit_order::LimitOrderRequest;
use crate::transaction::types::{AmountType, PositionIdType};
use crate::U64SerdeAsString;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Liquidate {
    #[serde(rename = "liquidator_order")]
    pub liquidator_order: LimitOrderRequest,
    // liquidator_position_id = liquidator_order.position_id.
    #[serde(rename = "liquidated_position_id", with = "U64SerdeAsString")]
    pub liquidated_position_id: PositionIdType,

    #[serde(rename = "actual_collateral", with = "U64SerdeAsString")]
    pub actual_collateral: AmountType,
    #[serde(rename = "actual_synthetic", with = "U64SerdeAsString")]
    pub actual_synthetic: AmountType,
    #[serde(rename = "actual_liquidator_fee", with = "U64SerdeAsString")]
    pub actual_liquidator_fee: AmountType,
}
