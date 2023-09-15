use std::fmt::Display;
use std::ops::ShlAssign;
use std::str::FromStr;

use franklin_crypto::eddsa::PublicKey;
use franklin_crypto::jubjub::FixedGenerators;
use primitive_types::U256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use wasm_bindgen::JsValue;
use zkdex_utils::tx::packed_public_key::private_key_from_string;
use zkdex_utils::tx::sign::TxSignature;
use zkdex_utils::{I128SerdeAsRadix16Prefix0xString, U64SerdeAsString};

pub use crate::serde_wrapper::*;



use anyhow::Result;
use zkdex_wasm::{AmountType, AssetIdType, CollateralAssetId, LeBytesConvert, PositionIdType};
use zkwasm_rust_sdk::JubjubSignature;

const LIMIT_ORDER_WITH_FEES: u64 = 3;
const TRANSFER_ORDER_TYPE: u64 = 4;
const CONDITIONAL_TRANSFER_ORDER_TYPE: u64 = 5;

use crate::common::OrderBase;
use zkdex_utils::u256_serde::U256SerdeAsRadix16Prefix0xString;
use zkdex_wasm::perpetual::{limit_order_hash, LimitOrder};
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct LimitOrderRequest {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "amount_synthetic", with = "U64SerdeAsString")]
    pub amount_synthetic: AmountType,
    #[serde(rename = "amount_collateral", with = "U64SerdeAsString")]
    pub amount_collateral: AmountType,
    #[serde(rename = "amount_fee", with = "U64SerdeAsString")]
    pub amount_fee: AmountType,
    #[serde(
        rename = "asset_id_synthetic",
        with = "I128SerdeAsRadix16Prefix0xString"
    )]
    pub asset_id_synthetic: AssetIdType,
    #[serde(
        rename = "asset_id_collateral",
        with = "U256SerdeAsRadix16Prefix0xString"
    )]
    pub asset_id_collateral: CollateralAssetId,
    #[serde(rename = "position_id", with = "U64SerdeAsString")]
    pub position_id: PositionIdType,
    #[serde(rename = "is_buying_synthetic")]
    pub is_buying_synthetic: bool,
}

pub fn sign_limit_order(mut req: LimitOrder, prvk: &str) -> Result<JubjubSignature> {
    let hash = limit_order_hash(&req);
    let private_key = private_key_from_string(prvk)?;
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig)
}

// #[test]
// pub fn test_sign() {
//     let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
//     let private_key = private_key_from_string(prv_key).unwrap();
//     let pub_key = public_key_from_private(&private_key);
//     let expire = 2;
//     let pub_key = PublicKeyType::from(pub_key.clone());
//     println!("{}", serde_json::to_string(&pub_key.clone()).unwrap());

//     let req = LimitOrderRequest {
//         base: OrderBase {
//             nonce: 1,
//             public_key: pub_key,
//             expiration_timestamp: expire,
//         },
//         amount_synthetic: 3,
//         amount_collateral: 4,
//         amount_fee: 5,
//         asset_id_synthetic: 6,
//         asset_id_collateral: CollateralAssetId::from(7),
//         position_id: 8,
//         is_buying_synthetic: false,
//     };

//     let w = sign_limit_order(req, prv_key).unwrap();
//     println!("{:?}", w);
// }

// #[test]
// pub fn test_sign2() {
//     let hash =
//         HashType::from_str("0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3")
//             .unwrap();
//     let hash1 =
//         HashType::from_str("0x15a9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3")
//             .unwrap();
//     println!("{:?}", hash.clone());
//     let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
//     let private_key = private_key_from_string(prv_key).unwrap();
//     let (sig, pub_key) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());

//     let pub_key = PublicKey::from_private(
//         &private_key,
//         FixedGenerators::SpendingKeyGenerator,
//         &JUBJUB_PARAMS,
//     );
//     assert!(sig.verify(&pub_key, hash.as_le_bytes()));
//     assert!(!sig.verify(&pub_key, hash1.as_le_bytes()));
// }

// #[test]
// fn test_deserialize() {
//     let json = r#"
//     {
//   "nonce": "1",
//   "public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
//   "expiration_timestamp": "2",
//   "amount_synthetic": "3",
//   "amount_collateral": "4",
//   "amount_fee": "5",
//   "asset_id_synthetic": "6",
//   "asset_id_collateral": "0xa",
//   "position_id": "8",
//   "is_buying_synthetic": false
//     }
//    "#;

//     let ret = serde_json::from_str::<LimitOrderRequest>(json);
//     assert!(ret.is_ok());
//     println!("{:?}", ret.unwrap())
// }
