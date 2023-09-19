use anyhow::Result;
use serde::{Deserialize, Serialize};
use zkdex_utils::tx::baby_jubjub::JubjubSignature;
use zkdex_utils::tx::packed_public_key::private_key_from_string;
use zkdex_utils::tx::sign::TxSignature;
use zkdex_utils::u256_serde::U256SerdeAsRadix16Prefix0xString;
use zkdex_utils::{I128SerdeAsRadix16Prefix0xString, U64SerdeAsString};
use zkdex_wasm::exchange::{mock_signature, OrderBase};
use zkdex_wasm::perpetual::{limit_order_hash, LimitOrder};
use zkdex_wasm::{AmountType, AssetIdType, CollateralAssetId, LeBytesConvert, PositionIdType};
#[derive(Clone, Debug, Deserialize, Serialize, Default)]

pub struct LimitOrderRequest {
    #[serde(flatten)]
    pub base: crate::common::OrderBase,
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

pub fn sign_limit_order(req: LimitOrderRequest, prvk: &str) -> Result<JubjubSignature> {
    let order = LimitOrder {
        base: OrderBase {
            nonce: req.base.nonce,
            public_key: req.base.public_key,
            expiration_timestamp: req.base.expiration_timestamp,
            signature: mock_signature(),
        },
        amount_synthetic: req.amount_synthetic,
        amount_collateral: req.amount_collateral,
        amount_fee: req.amount_fee,
        asset_id_synthetic: req.asset_id_synthetic,
        is_buying_synthetic: req.is_buying_synthetic,
        asset_id_collateral: req.asset_id_collateral,
        position_id: req.position_id,
    };
    let hash = limit_order_hash(&order);
    let private_key = private_key_from_string(prvk)?;
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::sign_limit_order;
    use crate::{
        transaction::limit_order::LimitOrderRequest,
        utils::{sign_msg, verify_sig},
    };
    use franklin_crypto::{eddsa::PublicKey, jubjub::FixedGenerators};
    use zkdex_utils::tx::{
        packed_public_key::{private_key_from_string, public_key_from_private},
        JUBJUB_PARAMS,
    };
    use zkdex_wasm::{CollateralAssetId, HashType, LeBytesConvert, PublicKeyType};

    #[test]
    pub fn test_sign() {
        let prv_key = "0x05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let pub_key = public_key_from_private(&private_key);
        let expire = 2;
        let pub_key = PublicKeyType::from(pub_key.clone());

        let req = LimitOrderRequest {
            base: crate::common::OrderBase {
                nonce: 1,
                public_key: pub_key.clone(),
                expiration_timestamp: expire,
            },
            amount_synthetic: 1,
            amount_collateral: 1,
            amount_fee: 1,
            asset_id_synthetic: 1,
            asset_id_collateral: CollateralAssetId::default(),
            position_id: 1,
            is_buying_synthetic: true,
        };

        let w_sig = sign_limit_order(req, prv_key);
        assert!(w_sig.is_ok());
    }

    #[test]
    pub fn test_sign2() {
        let hash = HashType::from_str(
            "0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3",
        )
        .unwrap();
        let hash1 = HashType::from_str(
            "0x15a9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3",
        )
        .unwrap();
        println!("{:?}", hash.clone());
        let prv_key = "0x05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let (sig, _pub_key) = sign_msg(&private_key, hash.as_le_bytes());

        let pub_key = PublicKey::from_private(
            &private_key,
            FixedGenerators::SpendingKeyGenerator,
            &JUBJUB_PARAMS,
        );

        assert!(verify_sig(
            sig.signature.clone(),
            &pub_key,
            hash.as_le_bytes()
        ));
        assert!(!verify_sig(sig.signature, &pub_key, hash1.as_le_bytes()));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"
    {
  "nonce": "1",
  "public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
  "expiration_timestamp": "2",
  "amount_synthetic": "3",
  "amount_collateral": "4",
  "amount_fee": "5",
  "asset_id_synthetic": "6",
  "asset_id_collateral": "0xa",
  "position_id": "8",
  "is_buying_synthetic": false
    }
   "#;

        let ret = serde_json::from_str::<LimitOrderRequest>(json);
        assert!(ret.is_ok());
        println!("{:?}", ret.unwrap())
    }
}
