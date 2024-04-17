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
