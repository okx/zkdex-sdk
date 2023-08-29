use serde::{Deserialize, Serialize};
use crate::transaction::limit_order::LimitOrderRequest;
use crate::transaction::types::{AmountType, PositionIdType};
use crate::U64SerdeAsString;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Liquidate {
    #[serde(rename = "liquidator_order")]
    pub liquidator_order: LimitOrderRequest,

    #[serde(rename = "liquidated_position_id", with = "U64SerdeAsString")]
    pub liquidated_position_id: PositionIdType,

    #[serde(rename = "actual_collateral", with = "U64SerdeAsString")]
    pub actual_collateral: AmountType,
    #[serde(rename = "actual_synthetic", with = "U64SerdeAsString")]
    pub actual_synthetic: AmountType,
    #[serde(rename = "actual_liquidator_fee", with = "U64SerdeAsString")]
    pub actual_liquidator_fee: AmountType,
}


#[test]
fn test_deserialize() {
    let json = r#"
    {
  "liquidator_order": {
    "nonce": "0",
    "public_key": "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
    "expiration_timestamp": "0",
    "amount_synthetic": "1",
    "amount_collateral": "2",
    "amount_fee": "3",
    "asset_id_synthetic": "4",
    "asset_id_collateral": "5",
    "position_id": "6",
    "is_buying_synthetic": false
  },
  "liquidated_position_id": "7",
  "actual_collateral": "8",
  "actual_synthetic": "9",
  "actual_liquidator_fee": "10"
}
    "#;

    let ret = serde_json::from_str::<Liquidate>(json);
    assert!(ret.is_ok());
}