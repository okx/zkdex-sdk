use std::ops::ShlAssign;
use std::str::FromStr;

use num_bigint::BigInt;
use primitive_types::U256;
use serde::{Deserialize, Deserializer, Serializer};
use wasm_bindgen::JsValue;

use crate::common::{
    CONDITIONAL_TRANSFER_ORDER_TYPE,
    TRANSFER_ORDER_TYPE,
};
use crate::common::OrderBase;
use crate::hash::hash2;
use crate::new_public_key::PublicKeyType;
use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;
use crate::sign_musig_without_hash_msg;
use crate::withdraw::{AmountType, CollateralAssetId, HashType, PositionIdType};

#[derive(Debug, Clone,Deserialize)]
pub struct TransferRequest {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "sender_position_id")]
    pub sender_position_id: PositionIdType,
    #[serde(rename = "receiver_public_key")]
    pub receiver_public_key: PublicKeyType,
    #[serde(rename = "receiver_position_id")]
    pub receiver_position_id: PositionIdType,
    #[serde(rename = "amount")]
    pub amount: AmountType,
    #[serde(rename = "asset_id", with="U256SerdeAsRadix16Prefix0xString")]
    pub asset_id: CollateralAssetId,
}

#[derive(Debug, Clone)]
pub struct HashTransferRequest {
    // #[serde(rename = "asset_id", with = "BigIntSerdeAsRadix10String")]
    pub asset_id: BigInt,
    // #[serde(rename = "asset_id_fee", with = "BigIntSerdeAsRadix10String")]
    pub asset_id_fee: BigInt,
    pub sender_vault_id: u64,
    pub receiver_vault_id: u64,
    pub src_fee_vault_id: u64,
    // #[serde(rename = "max_amount_fee", with = "BigIntSerdeAsRadix10String")]
    pub max_amount_fee: BigInt,
    // #[serde(rename = "amount", with = "BigIntSerdeAsRadix10String")]
    pub amount: BigInt,
    pub nonce: u64,
    pub expiration_timestamp: i64,

    // #[serde(rename = "receiver_public_key")]
    pub receiver_public_key: PublicKeyType,
}

// impl From<TransferRequest> for HashTransferRequest {
//     fn from(value: TransferRequest) -> Self {
//         Self {
//             asset_id: value.asset_id,
//             asset_id_fee: BigInt::zero(),
//             sender_vault_id: value.sender_position_id,
//             receiver_vault_id: value.receiver_position_id,
//             src_fee_vault_id: value.sender_position_id,
//             max_amount_fee: BigInt::zero(),
//             amount: value.amount.clone(),
//             nonce: value.base.nonce,
//             expiration_timestamp: value.base.expiration_timestamp.unix_timestamp(),
//             receiver_public_key: value.receiver_public_key.clone(),
//         }
//     }
// }


pub fn sign_transfer(
    transfer: TransferRequest,
    private_key: &[u8],
    condition: isize,
) -> Result<Vec<u8>, JsValue> {
    let hash = transfer_hash(&transfer, 0);
    println!("{:?}", hex::encode(&hash.as_bytes()));
    sign_musig_without_hash_msg(private_key, hash.as_bytes())
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

impl ExchangeTransfer {
    pub fn hash(&self, condition: u64) -> HashType {
        internal_transfer_hash(self, condition)
    }
}

fn internal_transfer_hash(transfer: &ExchangeTransfer, condition: u64) -> HashType {
    let msg = hash2(&transfer.asset_id, &transfer.asset_id_fee);
    let mut msg = hash2(&msg, &transfer.receiver_public_key);

    // Add condition to the signature hash if exists.
    if condition != 0 {
        msg = hash2(&msg, &condition);
    }

    let mut packed_message0 = U256::from(transfer.sender_vault_id);
    packed_message0.shl_assign(64);
    packed_message0 += U256::from(transfer.receiver_vault_id);

    packed_message0.shl_assign(64);
    packed_message0 += U256::from(transfer.src_fee_vault_id);

    packed_message0.shl_assign(32);
    packed_message0 += U256::from(transfer.base.nonce);

    let msg = hash2(&msg, &packed_message0);

    let mut packed_message1 = U256::from(if condition == 0 {
        // Normal Transfer.
        TRANSFER_ORDER_TYPE
    } else {
        // Conditional transfer.
        CONDITIONAL_TRANSFER_ORDER_TYPE
    });
    packed_message1.shl_assign(64);
    packed_message1 += U256::from(transfer.amount);

    packed_message1.shl_assign(64);
    packed_message1 += U256::from(transfer.max_amount_fee);

    packed_message1.shl_assign(32);
    packed_message1 += U256::from(transfer.base.expiration_timestamp);

    packed_message1.shl_assign(81); // Padding.

    hash2(&msg, &packed_message1)
}


pub fn transfer_hash(transfer: &TransferRequest, condition: u64) -> HashType {
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

#[test]
pub fn test_sign() {
    // let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    // let binding = hex::decode(&prv_key).unwrap();
    // let prv_bytes = binding.as_slice();
    // let pubbytes = private_key_to_pubkey_hash(prv_bytes).unwrap();
    // let expire = 1684832800i64;
    // let data = OffsetDateTime::from_unix_timestamp(expire).unwrap();
    // let req = TransferRequest {
    //     base: OrderBase {
    //         nonce: 1,
    //         public_key: Default::default(),
    //         expiration_timestamp: data,
    //     },
    //     sender_position_id: 1,
    //     receiver_public_key: Default::default(),
    //     receiver_position_id: 1,
    //     amount: BigInt::from(1),
    //     asset_id: BigInt::from(1),
    // };
    // let data = serde_json::to_string(&req).unwrap();
    // println!("{:?}", data);
    // let sig = sign_transfer(req, prv_bytes, 0).unwrap();
    // println!("{:?}", sig);
}
