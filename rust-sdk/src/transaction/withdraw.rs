

use serde::{Deserialize, Serialize};
use zkdex_utils::tx::packed_public_key::private_key_from_string;
use zkdex_utils::tx::sign::TxSignature;
use zkdex_utils::U64SerdeAsString;

use crate::common::OrderBase;
use anyhow::Result;
use zkdex_wasm::perpetual::{withdrawal_hash, Withdrawal};
use zkdex_wasm::PublicKeyType;
use zkdex_wasm::{CollateralAssetId, LeBytesConvert};
use zkdex_utils::tx::baby_jubjub::JubjubSignature;

pub type AmountType = u64;
pub type PositionIdType = u64;
use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithdrawRequest {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "position_id", with = "U64SerdeAsString")]
    pub position_id: PositionIdType,
    #[serde(rename = "amount", with = "U64SerdeAsString")]
    pub amount: AmountType,
    #[serde(rename = "eth_address")]
    pub owner_key: PublicKeyType,
    #[serde(rename = "asset_id", with = "U256SerdeAsRadix16Prefix0xString")]
    pub asset_id: CollateralAssetId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Withdraw {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "position_id")]
    pub position_id: PositionIdType,
    #[serde(rename = "amount")]
    pub amount: AmountType,
    #[serde(rename = "eth_address")] //TODO:: it`s string type in gateway api
    pub owner_key: PublicKeyType,
}

pub fn sign_withdraw(
    withdrawal: Withdrawal,
    asset_id_collateral: &CollateralAssetId,
    prvk: &str,
) -> Result<JubjubSignature> {
    let hash = withdrawal_hash(&withdrawal, asset_id_collateral);
    let private_key = private_key_from_string(prvk).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig)
}

#[test]
pub fn test_withdraw() {
    // let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    // let private_key = private_key_from_string(prv_key).unwrap();
    // let binding = hex::decode(&prv_key).unwrap();
    // let prv_bytes = binding.as_slice();
    // let pub_key = privkey_to_pubkey_internal(prv_bytes).unwrap();
    // let pub_key = public_key_from_private(&private_key);
    // let expire = 1684832800i64;
    // let pub_key = PublicKeyType::from(pub_key.clone());
    // let req = Withdraw {
    //     base: OrderBase {
    //         nonce: 1,
    //         public_key: pub_key.clone(),
    //         expiration_timestamp: expire,
    //     },
    //     position_id: 1,
    //     amount: 1,
    //     owner_key: pub_key.clone(),
    // };

    // println!("{:#?}", serde_json::to_string(&req).unwrap());
    //c1434d28
    // let w = sign_withdraw(req, &CollateralAssetId::from(10), prv_key).unwrap();
    // println!("{:#?}", w);
}

#[test]
pub fn test_deserialize() {
    // let json = r#"{
    //     "nonce":"1",
    //     "public_key":"0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
    //     "expiration_timestamp":"1684832800",
    //     "position_id":"2",
    //     "amount":"3",
    //     "eth_address":"0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
    //     "asset_id": "0x1a"
    // }"#;

    // let withdraw = serde_json::from_str::<WithdrawRequest>(json);
    // assert!(withdraw.is_ok());
}
