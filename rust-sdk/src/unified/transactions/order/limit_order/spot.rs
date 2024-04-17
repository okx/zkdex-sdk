use crate::common::OrderBase;
use crate::constant::UNIFIED_SPOT_LIMIT_ORDER_TYPE;
use crate::hash::new_hasher;
use crate::hash::Hasher;
use crate::serde_utils::serde_str;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::types::{AmountType, AssetIdType, PositionIdType};
use primitive_types::U256;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LimitOrder {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(with = "serde_str")]
    pub amount_buy: AmountType,
    #[serde(with = "serde_str")]
    pub amount_sell: AmountType,
    #[serde(with = "serde_str")]
    pub amount_fee: AmountType,
    pub asset_buy: AssetIdType,
    pub asset_sell: AssetIdType,
    pub position_id: PositionIdType,
}

impl HashTrait for LimitOrder {
    fn hash(&self) -> U256 {
        let mut hasher = new_hasher();

        hasher.update_single(&UNIFIED_SPOT_LIMIT_ORDER_TYPE);

        let mut packed = Into::<u32>::into(self.asset_buy) as u64;
        packed |= (Into::<u32>::into(self.asset_sell) as u64) << 32;

        hasher.update_single(&packed);

        hasher.update_single(&self.amount_sell);
        hasher.update_single(&self.amount_buy);
        hasher.update_single(&self.amount_fee);

        let packed_message = U256([
            (self.base.expiration_timestamp as u64) << 32 | self.base.nonce as u64,
            self.position_id.0 as u64,
            0,
            0,
        ]);

        hasher.update_single(&packed_message);

        hasher.finalize()
    }
}

impl SignTrait for LimitOrder {}
