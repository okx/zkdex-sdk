use primitive_types::U256;
use std::ops::ShlAssign;

use crate::common::OrderBase;
use crate::constant::SPOT_TRANSFER_ORDER_TYPE;
use crate::hash;
use crate::hash::Hasher;
use crate::tx::public_key_type::PublicKeyType;
use crate::tx::{private_key_from_string, HashType, TxSignature};
use crate::types::{SpotAmountType, SpotAssetIdType, SpotPositionIdType};

use crate::felt::LeBytesConvert;
use crate::zkw::JubjubSignature;
use {
    crate::serde_wrapper::{
        SpotAmountTypeSerdeAsRadix10String, SpotAssetIdTypeSerdeAsRadix16String,
        SpotPositionIdTypeSerdeAsRadix10String,
    },
    crate::TransferBaseSerde,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Transfer {
    #[serde(flatten, with = "TransferBaseSerde")]
    pub base: OrderBase,
    #[serde(rename = "amount", with = "SpotAmountTypeSerdeAsRadix10String")]
    pub amount: SpotAmountType,
    #[serde(rename = "asset_id", with = "SpotAssetIdTypeSerdeAsRadix16String")]
    pub asset_id: SpotAssetIdType,
    #[serde(rename = "receiver_position_id", with = "SpotPositionIdTypeSerdeAsRadix10String")]
    pub receiver_position_id: SpotPositionIdType,
    #[serde(rename = "receiver_public_key")]
    pub receiver_public_key: PublicKeyType,
    #[serde(rename = "sender_position_id", with = "SpotPositionIdTypeSerdeAsRadix10String")]
    pub sender_position_id: SpotPositionIdType,
}

pub fn transfer_hash(transfer: &Transfer) -> HashType {
    let mut hasher = hash::new_hasher();

    hasher.update_single(&SPOT_TRANSFER_ORDER_TYPE);

    hasher.update_single(&(transfer.asset_id.0 as u64));

    let mut packed_message0 = U256([
        transfer.sender_position_id.into(),
        transfer.receiver_position_id.into(),
        transfer.sender_position_id.into(),
        0,
    ]);

    packed_message0.shl_assign(32);
    packed_message0 += U256::from(transfer.base.nonce);

    hasher.update_single(&packed_message0);

    let mut packed_message1 = U256([
        transfer.amount.0 as u64,
        (transfer.amount.0 >> 64) as u64,
        SPOT_TRANSFER_ORDER_TYPE,
        0,
    ]);

    packed_message1.shl_assign(32);
    packed_message1 += U256::from(transfer.base.expiration_timestamp);
    packed_message1.shl_assign(81); // Padding.

    hasher.update_single(&packed_message1);

    hasher.finalize()
}

pub fn sign_transfer(transfer: Transfer, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let hash = transfer_hash(&transfer);
    let private_key = private_key_from_string(private_key).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

#[cfg(test)]
mod test {
    use crate::spot::Transfer;

    #[test]
    pub fn test_deserialize() {
        let json = r#"
        {
        "nonce": "1",
        "sender_public_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
        "expiration_timestamp": "3608164305",
        "amount": "10",
        "asset_id": "0x00001",
        "receiver_position_id": "1",
        "receiver_public_key": "0x0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
        "sender_position_id": "1"
        }
        "#;

        let transfer = serde_json::from_str::<Transfer>(json);
        assert!(transfer.is_ok());
    }
}
