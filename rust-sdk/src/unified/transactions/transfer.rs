use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::common::OrderBase;
use crate::constant::UNIFIED_TRANSFER_ORDER_TYPE;
use crate::hash::new_hasher;
use crate::hash::Hasher;
use crate::serde_wrapper::{serde_utils::serde_str, TransferBaseSerde};
use crate::tx::public_key_type::PublicKeyType;
use crate::types::AmountType;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::types::{AssetIdType, PositionIdType};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Transfer {
    #[serde(flatten, with = "TransferBaseSerde")]
    pub base: OrderBase,
    #[serde(with = "serde_str")]
    pub amount: AmountType,
    pub asset_id: AssetIdType,
    pub synthetic_id: AssetIdType,
    pub sender_position_id: PositionIdType,
    pub receiver_position_id: PositionIdType,
    pub receiver_public_key: PublicKeyType,
}

impl HashTrait for Transfer {
    fn hash(&self) -> U256 {
        let transfer = self;
        let mut hasher = new_hasher();

        hasher.update_single(&UNIFIED_TRANSFER_ORDER_TYPE);

        let packed_message0 = U256([
            transfer.asset_id.into(),
            transfer.synthetic_id.into(),
            transfer.sender_position_id.into(),
            transfer.receiver_position_id.into(),
        ]);

        hasher.update_single(&packed_message0);

        hasher.update_single(&self.receiver_public_key);

        let packed_message1 = U256([
            (self.base.expiration_timestamp as u64) << 32 | self.base.nonce as u64,
            transfer.amount,
            0,
            0,
        ]);

        hasher.update_single(&packed_message1);

        hasher.finalize()
    }
}

impl SignTrait for Transfer {}

#[cfg(test)]
mod tests {
    use crate::unified::transactions::{sign_and_verify, Transfer};

    #[test]
    fn test_transfer_serde() {
        let js = r##"
        {
            "amount": "7758176404715800194",
            "asset_id": "0x1234",
            "synthetic_id" : "0x0",
            "expiration_timestamp": "2404381470",
            "nonce": "2195908194",
            "receiver_position_id": "609106",
            "receiver_public_key": "0x259f432e6f4590b9a164106cf6a659eb4862b21fb97d43588561712e8e5216b",
            "sender_position_id": "93098",
            "sender_public_key": "0x28e4d45cd0538ffa6fdc09e70f0fea4e56c47fda87a2a969c22b4fdfe997f60"
        }
        "##;
        let tx = serde_json::from_str::<Transfer>(js);
        assert!(tx.is_ok());
    }

    #[test]
    fn test_sign_and_verify() {
        let js = r##"
        {
            "amount": "7758176404715800194",
            "asset_id": "0x1234",
            "synthetic_id" : "0x0",
            "expiration_timestamp": "2404381470",
            "nonce": "2195908194",
            "receiver_position_id": "609106",
            "receiver_public_key": "0x259f432e6f4590b9a164106cf6a659eb4862b21fb97d43588561712e8e5216b",
            "sender_position_id": "93098",
            "sender_public_key": "0x28e4d45cd0538ffa6fdc09e70f0fea4e56c47fda87a2a969c22b4fdfe997f60"
        }
        "##;
        let tx = serde_json::from_str::<Transfer>(js);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx);
    }
}
