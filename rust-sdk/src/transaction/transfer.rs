use crate::hash::Hasher;
use anyhow::Result;
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use std::ops::ShlAssign;

use crate::common::OrderBase;
use crate::common::{CONDITIONAL_TRANSFER_ORDER_TYPE, TRANSFER_ORDER_TYPE};
use crate::felt::LeBytesConvert;
use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;
use crate::transaction::types::{AmountType, CollateralAssetId, HashType, PositionIdType};
use crate::tx::packed_public_key::private_key_from_string;
use crate::tx::public_key_type::PublicKeyType;
use crate::tx::TxSignature;
use crate::zkw::JubjubSignature;
use crate::{hash, U64SerdeAsString};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transfer {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "sender_position_id", with = "U64SerdeAsString")]
    pub sender_position_id: PositionIdType,
    #[serde(rename = "receiver_public_key")]
    pub receiver_public_key: PublicKeyType,
    #[serde(rename = "receiver_position_id", with = "U64SerdeAsString")]
    pub receiver_position_id: PositionIdType,
    #[serde(rename = "amount", with = "U64SerdeAsString")]
    pub amount: AmountType,
    #[serde(rename = "asset_id", with = "U256SerdeAsRadix16Prefix0xString")]
    pub asset_id: CollateralAssetId,
}

pub fn sign_transfer(transfer: Transfer, private_key: &str) -> Result<JubjubSignature> {
    let hash = transfer_hash(&transfer, 0);
    let private_key = private_key_from_string(private_key).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

#[derive(Default)]
pub struct ExchangeTransfer {
    pub base: OrderBase,
    // sender_public_key = base.public_key.
    pub sender_vault_id: PositionIdType,
    pub receiver_public_key: PublicKeyType,
    pub receiver_vault_id: PositionIdType,
    pub amount: AmountType,
    pub asset_id: CollateralAssetId,
    pub src_fee_vault_id: PositionIdType,
    pub asset_id_fee: AmountType,
    pub max_amount_fee: AmountType,
}

pub fn transfer_hash(transfer: &Transfer, condition: u64) -> HashType {
    let mut exchange_transfer = ExchangeTransfer::default();
    exchange_transfer.base = transfer.base.clone();
    exchange_transfer.sender_vault_id = transfer.sender_position_id;
    exchange_transfer.receiver_public_key = transfer.receiver_public_key.clone();
    exchange_transfer.receiver_vault_id = transfer.receiver_position_id;
    exchange_transfer.amount = transfer.amount;
    exchange_transfer.asset_id = transfer.asset_id;
    exchange_transfer.src_fee_vault_id = transfer.sender_position_id;
    exchange_transfer.asset_id_fee = 0;
    exchange_transfer.max_amount_fee = 0;

    return exchange_transfer.hash(condition);
}

pub fn transfer_hash_internal(transfer: &ExchangeTransfer, condition: u64) -> HashType {
    let mut hasher = hash::new_hasher();
    // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=transfer.asset_id, y=transfer.asset_id_fee);
    // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=msg, y=transfer.receiver_public_key);

    hasher.update_single(&transfer.asset_id);
    hasher.update_single(&transfer.asset_id_fee);

    // Add condition to the signature hash if exists.
    if condition != 0 {
        // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=msg, y=condition);
        hasher.update_single(&condition);
    }

    // let packed_message0 = transfer.sender_vault_id;
    // let packed_message0 = packed_message0 * VAULT_ID_UPPER_BOUND + transfer.receiver_vault_id;
    // let packed_message0 = packed_message0 * VAULT_ID_UPPER_BOUND + transfer.src_fee_vault_id;
    let mut packed_message0 = U256([
        transfer.src_fee_vault_id,
        transfer.receiver_vault_id,
        transfer.sender_vault_id,
        0,
    ]);
    // let packed_message0 = packed_message0 * NONCE_UPPER_BOUND + transfer.base.nonce;
    packed_message0.shl_assign(32);
    packed_message0 += U256::from(transfer.base.nonce);

    // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=msg, y=packed_message0);
    hasher.update_single(&packed_message0);

    // if (condition == 0) {
    //     // Normal Transfer.
    //     tempvar packed_message1 = TRANSFER_ORDER_TYPE;
    // } else {
    //     // Conditional transfer.
    //     tempvar packed_message1 = CONDITIONAL_TRANSFER_ORDER_TYPE;
    // }
    // let packed_message1 = packed_message1 * AMOUNT_UPPER_BOUND + transfer.amount;
    // let packed_message1 = packed_message1 * AMOUNT_UPPER_BOUND + transfer.max_amount_fee;
    let mut packed_message1 = U256([
        transfer.max_amount_fee,
        transfer.amount,
        if condition == 0 {
            // Normal Transfer.
            TRANSFER_ORDER_TYPE
        } else {
            // Conditional transfer.
            CONDITIONAL_TRANSFER_ORDER_TYPE
        },
        0,
    ]);

    // let packed_message1 = (
    //     packed_message1 * EXPIRATION_TIMESTAMP_UPPER_BOUND + transfer.base.expiration_timestamp
    // );
    packed_message1.shl_assign(32);
    packed_message1 += U256::from(transfer.base.expiration_timestamp);
    // let packed_message1 = packed_message1 * (2 ** 81);  // Padding.
    packed_message1.shl_assign(81); // Padding.

    // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=msg, y=packed_message1);
    hasher.update_single(&packed_message1);

    hasher.finalize()
}

impl ExchangeTransfer {
    pub fn hash(&self, condition: u64) -> HashType {
        transfer_hash_internal(self, condition)
    }
}

#[cfg(test)]
mod test {
    use crate::common::OrderBase;
    use crate::tx::public_key_type::PublicKeyType;
    use crate::tx::transfer::sign_transfer;
    use crate::tx::{private_key_from_string, public_key_from_private, Transfer};

    #[test]
    pub fn test_sign_transfer() {
        let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let pub_key = public_key_from_private(&private_key);
        let expire = 1684832800u32;
        let pub_key = PublicKeyType::from(pub_key.clone());
        let req = Transfer {
            base: OrderBase {
                nonce: 1,
                public_key: pub_key.clone(),
                expiration_timestamp: expire,
            },
            sender_position_id: 0,
            receiver_public_key: Default::default(),
            amount: 1,
            receiver_position_id: 0,
            asset_id: Default::default(),
        };

        let w = sign_transfer(req, prv_key).unwrap();
        println!("{:?}", w);
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
        "nonce": "1",
        "public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "expiration_timestamp": "11111111",
        "sender_position_id": "1",
        "receiver_public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "receiver_position_id": "1",
        "amount": "1",
        "asset_id": "0xa8"
    }"#;

        let ret = serde_json::from_str::<Transfer>(json);
        assert!(ret.is_ok());
        println!("{:?}", ret.unwrap());
    }
}
