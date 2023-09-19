use anyhow::Result;

use serde::{Deserialize, Serialize};
use zkdex_utils::tx::packed_public_key::private_key_from_string;
use zkdex_utils::tx::sign::TxSignature;
use zkdex_utils::U64SerdeAsString;
use zkdex_wasm::exchange::{mock_signature, OrderBase};
use zkdex_wasm::{CollateralAssetId, LeBytesConvert, PositionIdType, PublicKeyType};

use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;

use zkdex_utils::tx::baby_jubjub::JubjubSignature;

use zkdex_wasm::perpetual::transfer_hash;

use super::withdraw::AmountType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransferRequest {
    #[serde(flatten)]
    pub base: crate::common::OrderBase,
    #[serde(rename = "sender_position_id", with = "U64SerdeAsString")]
    pub sender_position_id: PositionIdType,
    #[serde(rename = "receiver_public_key")]
    pub receiver_public_key: PublicKeyType,
    #[serde(rename = "receiver_position_id", with = "U64SerdeAsString")]
    pub receiver_position_id: PositionIdType,
    #[serde(rename = "amount", with = "U64SerdeAsString")]
    pub amount: AmountType,
    #[serde(rename = "asset_id", with = "U256SerdeAsRadix16Prefix0xString")]
    pub asset_id: CollateralAssetId,
}

pub fn sign_transfer(req: TransferRequest, private_key: &str) -> Result<JubjubSignature> {
    let transfer = zkdex_wasm::perpetual::transactions::Transfer {
        base: OrderBase {
            nonce: req.base.nonce,
            public_key: req.base.public_key,
            expiration_timestamp: req.base.expiration_timestamp,
            signature: mock_signature(),
        },
        sender_position_id: req.sender_position_id,
        receiver_public_key: req.receiver_public_key,
        receiver_position_id: req.receiver_position_id,
        amount: req.amount,
        asset_id: req.asset_id,
    };
    let hash = transfer_hash(&transfer, 0);
    let private_key = private_key_from_string(private_key).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig)
}

#[cfg(test)]
mod test {
    use super::sign_transfer;
    use super::TransferRequest;
    use crate::common::OrderBase;
    use zkdex_utils::tx::packed_public_key::{private_key_from_string, public_key_from_private};
    use zkdex_wasm::PublicKeyType;

    #[test]
    pub fn test_sign_transfer() {
        let prv_key = "0x05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let pub_key = public_key_from_private(&private_key);
        let expire = 1684832800i64;
        let pub_key = PublicKeyType::from(pub_key.clone());
        let req = TransferRequest {
            base: OrderBase {
                nonce: 1,
                public_key: pub_key.clone(),
                expiration_timestamp: expire,
            },
            sender_position_id: 0,
            receiver_public_key: Default::default(),
            amount: 1,
            receiver_position_id: 0,
            asset_id: Default::default(),
        };

        let w = sign_transfer(req, prv_key);
        assert!(w.is_ok());
    }

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
        "nonce": "1",
        "public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "expiration_timestamp": "11111111",
        "sender_position_id": "1",
        "receiver_public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "receiver_position_id": "1",
        "amount": "1",
        "asset_id": "0xa8"
        }
        "#;

        let ret = serde_json::from_str::<TransferRequest>(json);
        assert!(ret.is_ok());
    }
}
