use crate::crypto::packed_public_key::PrivateKeyType;
use crate::crypto::sign::TxSignature;
use crate::serde_wrapper::U64SerdeStr;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::order::perpetual::LimitOrder;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::types::{AmountType, PositionIdType};
use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Liquidate {
    pub liquidator_order: LimitOrder,
    // liquidator_position_id = liquidator_order.position_id.
    pub liquidated_position_id: PositionIdType,
    #[serde(with = "U64SerdeStr")]
    pub actual_collateral: AmountType,
    #[serde(with = "U64SerdeStr")]
    pub actual_synthetic: AmountType,
    #[serde(with = "U64SerdeStr")]
    pub actual_liquidator_fee: AmountType,
}

impl HashTrait for Liquidate {
    fn hash(&self) -> U256 {
        self.liquidator_order.hash()
    }
}

impl SignTrait for Liquidate {
    fn sign(&self, private_key: &PrivateKeyType) -> TxSignature {
        self.liquidator_order.sign(private_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::unified::transactions::test::sign_and_verify;

    #[test]
    fn test_tx_serde() {
        let js = r##"
{
    "actual_collateral":"7758176404715800194",
    "actual_liquidator_fee":"8791662011684601223",
    "actual_synthetic":"15308084094301570617",
    "liquidated_position_id":"1541968236",
    "liquidated_type":"PERP_CROSS",
    "liquidator_order":{
        "amount_collateral":"8187132600743567510",
        "amount_fee":"11081939229867047606",
        "amount_synthetic":"16558026091473266411",
        "asset_id_collateral":"0x57d05d1",
        "asset_id_synthetic":"0x2",
        "expiration_timestamp":"1430804514",
        "is_buying_synthetic":false,
        "type":"PERP_CROSS",
        "nonce":"3900315155",
        "position_id":"11534",
        "public_key":"0x5db665983e23607de57d6dc068797336bfdcb954238044688bec922ca296d3e"
    }
}
        "##;
        let tx = serde_json::from_str::<Liquidate>(js);
        assert!(tx.is_ok());
    }

    #[test]
    pub fn test_sign_and_verify() {
        let js = r##"
    {
    "actual_collateral":"7758176404715800194",
    "actual_liquidator_fee":"8791662011684601223",
    "actual_synthetic":"15308084094301570617",
    "liquidated_position_id":"1541968236",
    "liquidated_type":"PERP_CROSS",
    "liquidator_order":{
        "amount_collateral":"8187132600743567510",
        "amount_fee":"11081939229867047606",
        "amount_synthetic":"16558026091473266411",
        "asset_id_collateral":"0x57d05d1",
        "asset_id_synthetic":"0x2",
        "expiration_timestamp":"1430804514",
        "is_buying_synthetic":false,
        "type":"PERP_CROSS",
        "nonce":"3900315155",
        "position_id":"11534",
        "public_key":"0x5db665983e23607de57d6dc068797336bfdcb954238044688bec922ca296d3e"
        }
    }
        "##;
        let tx = serde_json::from_str::<Liquidate>(js);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx);
    }
}
