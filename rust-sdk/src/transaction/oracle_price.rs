// Price definitions:
// An external price is a unit of the collateral asset divided by a unit of synthetic asset.
// An internal price is computed as the ratio between a unit of collateral asset and its resolution,
// divided by the ratio between a unit of synthetic asset and its resolution:
//   (collateral_asset_unit / collateral_resolution) /
//   (synthetic_asset_unit / synthetic_resolution).
use crate::i128_serde::U128SerdeAsString;
use crate::U32SerdeAsString;
use primitive_types::U256;
use serde::Deserialize;
use std::ops::ShlAssign;

use crate::felt::LeBytesConvert;
use crate::hash::hash2;
use crate::transaction::types::{AssetIdType, HashType, PriceType, SignedAssetId, TimestampType};
use crate::tx::packed_public_key::private_key_from_string;
use crate::tx::public_key_type::PublicKeyType;
use crate::tx::{Serialize, TxSignature};
use crate::zkw::JubjubSignature;
use crate::U256SerdeAsRadix16Prefix0xString;
use anyhow::Result;

// Represents a single signature on an external price with a timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedOraclePrice {
    #[serde(rename = "signer_key")]
    pub signer_key: PublicKeyType,
    #[serde(rename = "external_price", with = "U128SerdeAsString")]
    pub external_price: PriceType,
    #[serde(rename = "timestamp", with = "U32SerdeAsString")]
    pub timestamp: TimestampType,
    #[serde(rename = "signed_asset_id", with = "U256SerdeAsRadix16Prefix0xString")]
    pub signed_asset_id: SignedAssetId,
}

impl Default for SignedOraclePrice {
    fn default() -> Self {
        Self {
            signer_key: PublicKeyType::default(),
            external_price: PriceType::default(),
            timestamp: TimestampType::default(),
            signed_asset_id: SignedAssetId::default(),
        }
    }
}

// Represents a single Oracle Price of an asset in internal representation and
// signatures on that price. The price is a median of all prices in the signatures.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetOraclePrice {
    pub asset_id: AssetIdType,
    pub price: PriceType,
    // Oracle signatures, sorted by signer_key.
    pub signed_prices: Vec<SignedOraclePrice>,
}

pub struct TimeBounds {
    pub min_time: TimestampType,
    pub max_time: TimestampType,
}

// const TIMESTAMP_BOUND = 2 ** 32;
pub const TIMESTAMP_BOUND: i64 = 1 << 32;

pub fn signed_oracle_price_hash(price: &SignedOraclePrice) -> HashType {
    let mut y = U256::from(price.external_price);
    y.shl_assign(32);
    y += U256::from(price.timestamp);
    hash2(&price.signed_asset_id, &y)
}

pub fn sign_signed_oracle_price(price: SignedOraclePrice, prvk: &str) -> Result<JubjubSignature> {
    let hash = signed_oracle_price_hash(&price);
    let private_key = private_key_from_string(prvk)?;
    let (signature, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(signature.into())
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

    let ret = serde_json::from_str::<SignedOraclePrice>(json);
    assert!(ret.is_ok());
    println!("{:?}", ret);
}

#[test]
fn test_oracle() {
    let pri = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    let mut data = SignedOraclePrice::default();
    data.external_price = 100000;
    data.timestamp = 18778987;
    data.signed_asset_id = SignedAssetId::from(100);
    let sig = sign_signed_oracle_price(data, pri).unwrap();
    let json = serde_json::to_string(&sig).unwrap();
    println!("{:#?}", json);
    println!("{:#?}", sig);
}
