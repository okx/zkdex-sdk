use primitive_types::U256;

use serde::{Deserialize, Serialize};

use crate::common::OrderBase;
use crate::constant::UNIFIED_PERPETUAL_LIMIT_ORDER_TYPE;
use crate::hash::new_hasher;
use crate::hash::Hasher;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::types::{AmountType, AssetIdType, PositionIdType};
use crate::U64SerdeStr;

#[derive(Serialize, Deserialize)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum MarginType {
    #[serde(rename = "PERP_CROSS")]
    #[default]
    Cross,
    #[serde(rename = "PERP_ISO")]
    Isolated,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct LimitOrder {
    #[serde(flatten)]
    pub base: OrderBase,
    pub position_id: PositionIdType,
    pub asset_id_synthetic: AssetIdType,
    pub asset_id_collateral: AssetIdType,
    #[serde(with = "U64SerdeStr")]
    pub amount_synthetic: AmountType,
    #[serde(with = "U64SerdeStr")]
    pub amount_collateral: AmountType,
    #[serde(with = "U64SerdeStr")]
    pub amount_fee: AmountType,
    pub is_buying_synthetic: bool,
    #[serde(rename = "type")]
    pub margin_type: MarginType,
}

impl HashTrait for LimitOrder {
    fn hash(&self) -> U256 {
        let mut hasher = new_hasher();

        hasher.update_single(&UNIFIED_PERPETUAL_LIMIT_ORDER_TYPE);

        let mut packed = Into::<u32>::into(self.asset_id_collateral) as u64;
        packed |= (Into::<u32>::into(self.asset_id_synthetic) as u64) << 32;

        hasher.update_single(&packed);

        hasher.update_single(&self.amount_collateral);
        hasher.update_single(&self.amount_synthetic);
        hasher.update_single(&self.amount_fee);

        let packed_message = U256([
            (self.base.expiration_timestamp as u64) << 32 | self.base.nonce as u64,
            self.position_id.0 as u64,
            (self.margin_type as u64) | (self.is_buying_synthetic as u64) << 32,
            0,
        ]);

        hasher.update_single(&packed_message);

        hasher.finalize()
    }
}

impl SignTrait for LimitOrder {}

#[cfg(test)]
mod test {
    use crate::unified::transactions::order::perpetual::LimitOrder;
    use crate::unified::transactions::test::sign_and_verify;

    #[test]
    fn test_sign_verify() {
        let json = r#"
        {
        "type":"PERP_CROSS",
        "amount_collateral":"15334874",
        "amount_fee":"1767749",
        "amount_synthetic":"15460142",
        "asset_id_collateral":"0x57d05d",
        "asset_id_synthetic":"0x2",
        "expiration_timestamp":"3608164305",
        "is_buying_synthetic":true,
        "nonce":"1210484339",
        "order_type":"LIMIT_ORDER_WITH_FEES",
        "position_id":"4805234",
        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7"
        }
        "#;
        let tx = serde_json::from_str::<LimitOrder>(json);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx);
    }
}
