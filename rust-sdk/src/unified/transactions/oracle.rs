use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::crypto::public_key_type::PublicKeyType;
use crate::hash::hash2;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::types::{ExternalPriceType, TimestampType};
use crate::U128SerdeAsString;
use crate::U256SerdeAsRadix16Prefix0xString;
use crate::U32SerdeAsString;

// Price definitions:
// An external price is a unit of the collateral asset divided by a unit of synthetic asset.
// An internal price is computed as the ratio between a unit of collateral asset and its resolution,
// divided by the ratio between a unit of synthetic asset and its resolution:
//   (collateral_asset_unit / collateral_resolution) /
//   (synthetic_asset_unit / synthetic_resolution).

// Represents a single signature on an external price with a timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedOraclePrice {
    pub signer_key: PublicKeyType,
    #[serde(with = "U128SerdeAsString")]
    pub external_price: ExternalPriceType,
    #[serde(with = "U32SerdeAsString")]
    pub timestamp: TimestampType,
    #[serde(with = "U256SerdeAsRadix16Prefix0xString")]
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

#[cfg(test)]
mod test {
    use crate::unified::transactions::test::sign_and_verify;
    use crate::unified::transactions::SignedOraclePrice;

    #[test]
    pub fn test_deserialize() {
        let js = r##"
            {
            "signer_key": "0x87e5235c9c3916ef2b0def77111366ecef72914613f52febad308440b6463f83",
            "external_price": "30000000",
            "timestamp": "1651148012",
            "signed_asset_id": "0x425443555344000000000000000000004d616b6572"
            }
        "##;

        let tx = serde_json::from_str::<SignedOraclePrice>(js);
        assert!(tx.is_ok())
    }

    #[test]
    pub fn test_sign_and_verify() {
        let js = r##"
            {
            "signer_key": "0x87e5235c9c3916ef2b0def77111366ecef72914613f52febad308440b6463f83",
            "external_price": "30000000",
            "timestamp": "1651148012",
            "signed_asset_id": "0x425443555344000000000000000000004d616b6572"
            }
        "##;

        let tx = serde_json::from_str::<SignedOraclePrice>(js);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx);
    }
}
