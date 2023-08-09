use std::fmt::{Display, Formatter};
use std::ops::ShlAssign;
use std::str::FromStr;

use primitive_types::U256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use wasm_bindgen::JsValue;

use crate::common::OrderBase;
use crate::hash::{hash2, ToHashable};
use crate::new_public_key::PublicKeyType;
pub use crate::serde_wrapper::*;
use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;
use crate::transaction::types::{AmountType, CollateralAssetId, HashType, PositionIdType};
use crate::tx::packed_public_key::{private_key_from_string, public_key_from_private};
use crate::tx::TxSignature;
use crate::zkw::{BabyJubjubPoint, JubjubSignature};

const LIMIT_ORDER_WITH_FEES: u64 = 3;
const TRANSFER_ORDER_TYPE: u64 = 4;
const CONDITIONAL_TRANSFER_ORDER_TYPE: u64 = 5;

#[derive(Clone, Debug, Deserialize, Serialize,Default)]
pub struct LimitOrderRequest {
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
    #[serde(
        rename = "asset_id_collateral",
        with = "U256SerdeAsRadix16Prefix0xString"
    )]
    pub asset_id_collateral: CollateralAssetId,
    #[serde(rename = "position_id")]
    pub position_id: PositionIdType,
    #[serde(rename = "is_buying_synthetic")]
    pub is_buying_synthetic: bool,
}

pub fn sign_limit_order(
    mut req: LimitOrderRequest,
    prvk: &str,
) -> Result<JubjubSignature, JsValue> {
    let hash = limit_order_hash(&req);
    let private_key = private_key_from_string(prvk).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_bytes());
    Ok(sig)
}

#[derive(Default)]
pub struct LimitOrder {
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

impl LimitOrder {
    pub fn hash(&self) -> HashType {
        internal_limit_order_hash(self)
    }
}

fn internal_limit_order_hash(limit_order: &LimitOrder) -> HashType {
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

pub fn limit_order_hash(limit_order: &LimitOrderRequest) -> HashType {
    let mut exchange_limit_order: LimitOrder = Default::default();
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
    let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    let private_key = private_key_from_string(prv_key).unwrap();
    let pub_key = public_key_from_private(&private_key);
    let expire = 2;
    let pub_key = PublicKeyType::from(pub_key.clone());
    println!("{}",serde_json::to_string(&pub_key.clone()).unwrap());

    let req = LimitOrderRequest {
        base: OrderBase {
            nonce: 1,
            public_key: pub_key,
            expiration_timestamp: expire,
        },
        amount_synthetic: 3,
        amount_collateral: 4,
        amount_fee: 5,
        asset_id_synthetic: 6,
        asset_id_collateral: CollateralAssetId::from(7),
        position_id: 8,
        is_buying_synthetic: false,
    };

    let w = sign_limit_order(req, prv_key).unwrap();
    println!("{:?}", w);
}


#[test]
pub fn test_sign2() {
    let hash = HashType::from_str("0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3").unwrap();
    println!("{:?}",hash.clone());
    let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    let private_key = private_key_from_string(prv_key).unwrap();
    let (sig, pub_key) = TxSignature::sign_msg(&private_key, hash.as_bytes());
    println!("{:#?}", sig);
    println!("{:#?}", pub_key)
    // if sig.sig_r.x == 0 && sig.sig_r.y == 1 {
    //     println!("verify success")
    // }
    // BabyJubjubPoint{ x: U256::from_str('), y: Default::default() }
}