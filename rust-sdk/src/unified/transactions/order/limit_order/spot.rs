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

#[cfg(test)]
mod test {
    use crate::unified::transactions::order::spot::LimitOrder;
    use crate::unified::transactions::test::sign_and_verify;

    #[test]
    fn test_sign_verify() {
        let json = r#"
        {
                "amount_buy": "80",
                "amount_sell": "70",
                "amount_fee": "111",
                "expiration_timestamp": "3396833",
                "nonce": "1654615998",
                "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
                "signature": {
                    "r": "0x9d4ed071faf946d6e22aa9f72016b4dcb07137bfdd976a1482d26c862bc1cd6e",
                    "s": "0x2baed04d00fbf2109d9d53235f6168d6e43b8858fc9b435bb8823e7d3aff335"
                 },
                "asset_buy": "0x22222",
                "asset_sell": "0x1111",
                "position_id": "922337"
        }
        "#;
        let tx = serde_json::from_str::<LimitOrder>(json);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx);
    }
}
