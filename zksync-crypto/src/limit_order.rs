use std::ops::ShlAssign;

use num_bigint::BigInt;
use primitive_types::U256;
use serde::{Deserialize, Deserializer, Serializer};
use wasm_bindgen::JsValue;

use js_types::common::params::LIMIT_ORDER_WITH_FEES;

use crate::common::OrderBase;
use crate::hash::hash2;
pub use crate::serde_wrapper::*;
use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;
use crate::sign_musig_without_hash_msg;
use crate::withdraw::{AmountType, CollateralAssetId, HashType, PositionIdType};

#[derive(Clone, Debug, Deserialize)]
pub struct ExchangeLimitOrderRequest {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "amount_synthetic")]
    pub amount_synthetic: AmountType,
    #[serde(rename = "amount_collateral")]
    pub amount_collateral: AmountType,
    #[serde(rename = "amount_fee")]
    pub amount_fee: AmountType,
    #[serde(rename = "asset_id_synthetic")]
    pub asset_id_synthetic: AmountType,
    #[serde(rename = "asset_id_collateral",with="U256SerdeAsRadix16Prefix0xString")]
    pub asset_id_collateral: CollateralAssetId,
    #[serde(rename = "position_id")]
    pub position_id: PositionIdType,
    #[serde(rename = "is_buying_synthetic")]
    pub is_buying_synthetic: bool,
}

// impl From<ExchangeLimitOrderRequest> for HashLimitOrderRequest {
//     fn from(limit_order: ExchangeLimitOrderRequest) -> Self {
//         let (asset_id_sell, asset_id_buy, amount_sell, amount_buy) =
//             if limit_order.is_buying_synthetic {
//                 (
//                     limit_order.asset_id_collateral.clone(),
//                     limit_order.asset_id_synthetic.clone(),
//                     limit_order.amount_collateral.clone(),
//                     limit_order.amount_synthetic.clone(),
//                 )
//             } else {
//                 (
//                     limit_order.asset_id_synthetic.clone(),
//                     limit_order.asset_id_collateral.clone(),
//                     limit_order.amount_synthetic.clone(),
//                     limit_order.amount_collateral.clone(),
//                 )
//             };
//
//         Self {
//             base: limit_order.base.clone(),
//             amount_fee: limit_order.amount_fee.clone(),
//             asset_id_fee: limit_order.asset_id_collateral.clone(),
//             vault_buy: limit_order.position_id,
//             vault_sell: limit_order.position_id,
//             vault_fee: limit_order.position_id,
//             asset_id_buy,
//             asset_id_sell,
//             amount_sell,
//             amount_buy,
//         }
//     }
// }

#[derive(Clone, Debug)]
pub struct HashLimitOrderRequest {
    base: OrderBase,
    amount_buy: BigInt,
    amount_sell: BigInt,
    amount_fee: AmountType,
    asset_id_buy: BigInt,
    asset_id_sell: BigInt,
    asset_id_fee: BigInt,
    vault_buy: u64,
    vault_sell: u64,
    vault_fee: u64,
}

pub fn sign_limit_order(req: ExchangeLimitOrderRequest, prvk: &[u8]) -> Result<Vec<u8>, JsValue> {
    // let req = HashLimitOrderRequest::from(req);
    let hash = limit_order_hash(&req);
    sign_musig_without_hash_msg(prvk, hash.as_bytes())
}

#[derive(Default)]
pub struct ExchangeLimitOrder {
    pub base: OrderBase,
    pub amount_buy: AmountType,
    pub amount_sell: AmountType,
    pub amount_fee: AmountType,
    pub asset_id_buy: CollateralAssetId,
    pub asset_id_sell: CollateralAssetId,
    pub asset_id_fee: CollateralAssetId,
    pub vault_buy: PositionIdType,
    pub vault_sell: PositionIdType,
    pub vault_fee: PositionIdType,
}

impl ExchangeLimitOrder {
    pub fn hash(&self) -> HashType {
        internal_limit_order_hash(self)
    }
}

fn internal_limit_order_hash(limit_order: &ExchangeLimitOrder) -> HashType {
    let msg = hash2(&limit_order.asset_id_sell, &limit_order.asset_id_buy);

    let msg = hash2(&msg, &limit_order.asset_id_fee);

    let mut packed_message0 = U256::from(limit_order.amount_sell);
    // let packed_message0 = packed_message0 * AMOUNT_UPPER_BOUND + limit_order.amount_buy;
    packed_message0.shl_assign(64);
    packed_message0 += U256::from(limit_order.amount_buy);

    // let packed_message0 = packed_message0 * AMOUNT_UPPER_BOUND + limit_order.amount_fee;
    packed_message0.shl_assign(64);
    packed_message0 += U256::from(limit_order.amount_fee);

    // let packed_message0 = packed_message0 * NONCE_UPPER_BOUND + limit_order.base.nonce;
    packed_message0.shl_assign(32);
    packed_message0 += U256::from(limit_order.base.nonce);

    let msg = hash2(&msg, &packed_message0);

    let mut packed_message1 = U256::from(LIMIT_ORDER_WITH_FEES);
    // let packed_message1 = packed_message1 * VAULT_ID_UPPER_BOUND + limit_order.vault_fee;
    packed_message1.shl_assign(64);
    packed_message1 += U256::from(limit_order.vault_fee);

    // let packed_message1 = packed_message1 * VAULT_ID_UPPER_BOUND + limit_order.vault_sell;
    packed_message1.shl_assign(64);
    packed_message1 += U256::from(limit_order.vault_sell);

    // let packed_message1 = packed_message1 * VAULT_ID_UPPER_BOUND + limit_order.vault_buy;
    packed_message1.shl_assign(64);
    packed_message1 += U256::from(limit_order.vault_buy);

    // let packed_message1 = packed_message1 * EXPIRATION_TIMESTAMP_UPPER_BOUND + limit_order.base.expiration_timestamp;
    packed_message1.shl_assign(32);
    packed_message1 += U256::from(limit_order.base.expiration_timestamp);

    // let packed_message1 = packed_message1 * (2 ** 17);  // Padding.
    let packed_message1 = packed_message1 << 17; // Padding.

    hash2(&msg, &packed_message1)
}


pub fn limit_order_hash(limit_order: &ExchangeLimitOrderRequest) -> HashType {
    let mut exchange_limit_order: ExchangeLimitOrder = Default::default();
    exchange_limit_order.base = limit_order.base.clone();
    exchange_limit_order.amount_fee = limit_order.amount_fee;
    exchange_limit_order.asset_id_fee = limit_order.asset_id_collateral;
    exchange_limit_order.vault_buy = limit_order.position_id;
    exchange_limit_order.vault_sell = limit_order.position_id;
    exchange_limit_order.vault_fee = limit_order.position_id;

    if limit_order.is_buying_synthetic {
        exchange_limit_order.asset_id_sell = limit_order.asset_id_collateral;
        exchange_limit_order.asset_id_buy = U256::from(limit_order.asset_id_synthetic);
        exchange_limit_order.amount_sell = limit_order.amount_collateral;
        exchange_limit_order.amount_buy = limit_order.amount_synthetic;
    } else {
        exchange_limit_order.asset_id_sell = U256::from(limit_order.asset_id_synthetic);
        exchange_limit_order.asset_id_buy = limit_order.asset_id_collateral;
        exchange_limit_order.amount_sell = limit_order.amount_synthetic;
        exchange_limit_order.amount_buy = limit_order.amount_collateral;
    }

    exchange_limit_order.hash()
}


#[test]
pub fn test_sign() {
    // let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    // let binding = hex::decode(&prv_key).unwrap();
    // let prv_bytes = binding.as_slice();
    // let pub_key = privkey_to_pubkey_internal(prv_bytes).unwrap();
    // let pub_key = PublicKeyType(pub_key);
    // let expire = 1684832800i64;
    // let data = OffsetDateTime::from_unix_timestamp(expire).unwrap();
    // let req = ExchangeLimitOrderRequest {
    //     base: OrderBase {
    //         nonce: 1,
    //         public_key: pub_key,
    //         expiration_timestamp: expire,
    //     },
    //     amount_buy: BigInt::from(1),
    //     amount_sell: BigInt::from(1),
    //     amount_fee: BigInt::from(1),
    //     asset_id_buy: BigInt::from(1),
    //     asset_id_sell: BigInt::from(1),
    //     asset_id_fee: BigInt::from(1),
    //     vault_buy: 1,
    //     vault_sell: 1,
    //     vault_fee: 1,
    // };
    // let req = ExchangeLimitOrderRequest {
    //     base: OrderBase {
    //         nonce: 1,
    //         public_key: pub_key,
    //         expiration_timestamp: 1,
    //     },
    //     amount_synthetic: 1,
    //     amount_collateral: 1,
    //     amount_fee: 1,
    //     asset_id_synthetic: 1,
    //     asset_id_collateral: CollateralAssetId::one(),
    //     position_id: 1,
    //     is_buying_synthetic: false,
    // };
    //
    // let json_data = serde_json::to_string(&req).unwrap();
    // println!("{:?}", json_data);
    // let signature = sign_limit_order(req, prv_bytes).unwrap();
    // println!("{:?}", signature);
}
