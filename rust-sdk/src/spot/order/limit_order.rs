use crate::common::OrderBase;
use crate::constant::SPOT_SETTLEMENT_ORDER_TYPE;
use crate::felt::LeBytesConvert;
use crate::hash::Hasher;
use crate::tx::{private_key_from_string, TxSignature};
use crate::types::amount::AmountType;
use crate::types::asset_id::AssetIdType;
use crate::types::position_id::PositionIdType;
use crate::zkw::JubjubSignature;
use crate::{hash, HashType};
use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct LimitOrder {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "amount_buy")]
    pub amount_buy: AmountType,
    #[serde(rename = "amount_sell")]
    pub amount_sell: AmountType,
    #[serde(rename = "amount_fee")]
    pub amount_fee: AmountType,
    #[serde(rename = "asset_buy")]
    pub asset_buy: AssetIdType,
    #[serde(rename = "asset_sell")]
    pub asset_sell: AssetIdType,
    #[serde(rename = "position_id")]
    pub position_id: PositionIdType,
}

impl LimitOrder {
    pub fn hash(&self) -> HashType {
        let mut hasher = hash::new_hasher();

        hasher.update_single(&SPOT_SETTLEMENT_ORDER_TYPE);

        hasher.update_single(&(self.asset_sell.0 as u64));
        hasher.update_single(&(self.asset_buy.0 as u64));

        hasher.update_single(self.amount_sell.as_ref());
        hasher.update_single(self.amount_buy.as_ref());
        hasher.update_single(self.amount_fee.as_ref());

        let packed_message1 = U256([
            (self.base.expiration_timestamp as u64) << 32 | self.base.nonce as u64,
            self.position_id.0 as u64,
            0,
            SPOT_SETTLEMENT_ORDER_TYPE,
        ]) << 49;

        hasher.update_single(&packed_message1);

        hasher.finalize()
    }
}

pub fn sign_limit_order(
    withdrawal: &LimitOrder,
    private_key: &str,
) -> anyhow::Result<JubjubSignature> {
    let hash = withdrawal.hash();
    let private_key = private_key_from_string(private_key).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

#[cfg(test)]
mod test {
    use crate::spot::LimitOrder;

    #[test]
    pub fn test_deserialize() {
        let json = r#"{
            "nonce": "0",
            "expiration_timestamp": "0",
            "public_key": "0x00000000",
            "amount_buy": "0",
            "amount_sell": "0",
            "amount_fee": "0",
            "asset_buy":"0x01",
            "asset_sell":"0x02",
            "position_id":"1"

                }"#;
        let limit_order = serde_json::from_str::<LimitOrder>(json);
        assert!(limit_order.is_ok())
    }
}
