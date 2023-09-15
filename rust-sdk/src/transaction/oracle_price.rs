use crate::serde_wrapper::U128SerdeAsRadix16Prefix0xString;

use primitive_types::U256;
use serde::{Deserialize, Serialize};
use zkdex_utils::tx::packed_public_key::private_key_from_string;
use std::ops::ShlAssign;
use zkdex_utils::tx::sign::TxSignature;
use zkdex_utils::I64SerdeAsString;
use zkdex_wasm::perpetual::signed_oracle_price_hash;
use zkdex_wasm::{
    AssetIdType, HashType, LeBytesConvert, PriceType, PublicKeyType, SignedAssetId, TimestampType,
};
use zkwasm_rust_sdk::JubjubSignature;

use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;

use anyhow::Result;

// Represents a single signature on an external price with a timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedOraclePriceRequest {
    #[serde(rename = "signer_key")]
    pub signer_key: PublicKeyType,
    #[serde(rename = "external_price", with = "U128SerdeAsRadix16Prefix0xString")]
    pub external_price: PriceType,
    #[serde(rename = "timestamp", with = "I64SerdeAsString")]
    pub timestamp: TimestampType,
    #[serde(rename = "signed_asset_id", with = "U256SerdeAsRadix16Prefix0xString")]
    pub signed_asset_id: SignedAssetId,
}

impl Default for SignedOraclePriceRequest {
    fn default() -> Self {
        Self {
            signer_key: PublicKeyType::default(),
            external_price: PriceType::default(),
            timestamp: TimestampType::default(),
            signed_asset_id: SignedAssetId::default(),
        }
    }
}

pub struct TimeBounds {
    pub min_time: TimestampType,
    pub max_time: TimestampType,
}

pub fn sign_signed_oracle_price(
    price: zkdex_wasm::perpetual::SignedOraclePrice,
    prvk: &str,
) -> Result<JubjubSignature> {
    let hash = signed_oracle_price_hash(&price);
    let private_key = private_key_from_string(prvk)?;
    let (signature, public_key) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(signature)
}

#[test]
fn test_deserialize() {
    let json = r#"
    {
    "signer_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
    "external_price": "100",
    "timestamp": "2",
    "signed_asset_id": "0xa"
    }
    "#;

    let ret = serde_json::from_str::<SignedOraclePriceRequest>(json);
    assert!(ret.is_ok());
    println!("{:?}", ret);
}

// #[test]
// fn test_oracle() {
//     let pri = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
//     let mut data = SignedOraclePrice::default();
//     data.external_price = 100000;
//     data.timestamp = 18778987;
//     data.signed_asset_id = SignedAssetId::from(100);
//     let pri_key = private_key_from_string(pri).unwrap();
//     let sig = sign_signed_oracle_price(data, pri).unwrap();
//     let json = serde_json::to_string(&sig).unwrap();
//     println!("{:#?}", json);
//     println!("{:#?}", sig);
// }
