use serde::{Deserialize, Serialize};
use zkdex_utils::tx::packed_public_key::private_key_from_string;
use zkdex_utils::tx::sign::TxSignature;
use zkdex_utils::U64SerdeAsString;
use zkdex_wasm::exchange::{mock_signature, OrderBase};

use anyhow::Result;
use zkdex_utils::tx::baby_jubjub::JubjubSignature;
use zkdex_wasm::perpetual::{withdrawal_hash, Withdrawal};
use zkdex_wasm::PublicKeyType;
use zkdex_wasm::{CollateralAssetId, LeBytesConvert};

pub type AmountType = u64;
pub type PositionIdType = u64;
use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithdrawRequest {
    #[serde(flatten)]
    pub base: crate::common::OrderBase,
    #[serde(rename = "position_id", with = "U64SerdeAsString")]
    pub position_id: PositionIdType,
    #[serde(rename = "amount", with = "U64SerdeAsString")]
    pub amount: AmountType,
    #[serde(rename = "eth_address")]
    pub owner_key: PublicKeyType,
    #[serde(rename = "asset_id", with = "U256SerdeAsRadix16Prefix0xString")]
    pub asset_id: CollateralAssetId,
}

pub fn sign_withdraw(withdraw_req: WithdrawRequest, prvk: &str) -> Result<JubjubSignature> {
    let withdraw = Withdrawal {
        base: OrderBase {
            nonce: withdraw_req.base.nonce,
            public_key: withdraw_req.base.public_key,
            expiration_timestamp: withdraw_req.base.expiration_timestamp,
            signature: mock_signature(),
        },
        position_id: withdraw_req.position_id,
        amount: withdraw_req.amount,
        owner_key: withdraw_req.owner_key,
    };

    let hash = withdrawal_hash(&withdraw, &withdraw_req.asset_id);
    let private_key = private_key_from_string(prvk).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig)
}

#[cfg(test)]
mod test {
    use zkdex_utils::tx::packed_public_key::{private_key_from_string, public_key_from_private};
    use zkdex_wasm::{CollateralAssetId, PublicKeyType};

    use crate::transaction::withdraw::{sign_withdraw, WithdrawRequest};

    #[test]
    pub fn test_withdraw() {
        let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let pub_key = public_key_from_private(&private_key);
        let pub_key = PublicKeyType::from(pub_key.clone());
        let req = WithdrawRequest {
            base: crate::common::OrderBase {
                nonce: 1,
                public_key: pub_key.clone(),
                expiration_timestamp: 1684832800i64,
            },
            position_id: 1,
            amount: 1,
            owner_key: pub_key.clone(),
            asset_id: CollateralAssetId::from(1),
        };
        let w = sign_withdraw(req, prv_key);
        assert!(w.is_ok());
    }

    #[test]
    pub fn test_deserialize() {
        let json = r#"
        {
        "nonce":"1",
        "public_key":"0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "expiration_timestamp":"1684832800",
        "position_id":"2",
        "amount":"3",
        "eth_address":"0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "asset_id": "0x1a"
        }
        "#;

        let withdraw = serde_json::from_str::<WithdrawRequest>(json);
        assert!(withdraw.is_ok());
    }
}
