use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::hash::hash2;
use crate::tx::public_key_type::PublicKeyType;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::types::{AssetIdType, ExternalPriceType, PriceType, TimestampType};
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OraclePrice {
    pub asset_id: AssetIdType,
    // # 32.32 fixed point.
    pub price: PriceType,
}

// An array of oracle prices.
#[cfg_attr(feature = "notwasm", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OraclePrices {
    pub data: Vec<OraclePrice>,
}

// Price definitions:
// An external price is a unit of the collateral asset divided by a unit of synthetic asset.
// An internal price is computed as the ratio between a unit of collateral asset and its resolution,
// divided by the ratio between a unit of synthetic asset and its resolution:
//   (collateral_asset_unit / collateral_resolution) /
//   (synthetic_asset_unit / synthetic_resolution).

// Represents a single signature on an external price with a timestamp.
#[derive(Debug, Clone, PartialEq,Serialize,Deserialize)]
pub struct SignedOraclePrice {
    pub signer_key: PublicKeyType,
    pub external_price: ExternalPriceType,
    pub timestamp: TimestampType,
    pub signed_asset_id: U256,
}

impl HashTrait for SignedOraclePrice {
    fn hash(&self) -> U256 {
        let external_price: u128 = self.external_price;

        // y=sig.external_price * TIMESTAMP_BOUND + sig.timestamp
        hash2(
            &self.signed_asset_id,
            &U256([
                self.timestamp as u64 | ((external_price & (u32::MAX as u128)) << 32) as u64,
                (external_price >> 32) as u64,
                (external_price >> (32 + 64)) as u64,
                0,
            ]),
        )
    }
}

impl SignTrait for SignedOraclePrice {}

// Represents a single Oracle Price of an asset in internal representation and
// signatures on that price. The price is a median of all prices in the signatures.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetOraclePrice {
    pub asset_id: AssetIdType,
    pub price: PriceType,
    // Oracle signatures, sorted by signer_key.
    pub signed_prices: Vec<SignedOraclePrice>,
}
