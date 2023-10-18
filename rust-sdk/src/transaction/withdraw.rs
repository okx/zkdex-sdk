use anyhow::Result;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::common::OrderBase;
use crate::constant::{
    AMOUNT_UPPER_BOUND_U256, EXPIRATION_TIMESTAMP_UPPER_BOUND_U256, NONCE_UPPER_BOUND_U256,
    POSITION_ID_UPPER_BOUND_U256,
};
use crate::felt::LeBytesConvert;
use crate::hash::hash2;
use crate::tx::packed_public_key::private_key_from_string;
use crate::tx::public_key_type::PublicKeyType;
use crate::tx::{HashType, TxSignature};
use crate::zkw::JubjubSignature;
use crate::U256SerdeAsRadix16Prefix0xString;
use crate::U64SerdeAsString;

pub type AmountType = u64;
pub type PositionIdType = u64;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    withdrawal: Withdraw,
    asset_id_collateral: &CollateralAssetId,
    prvk: &str,
) -> Result<JubjubSignature> {
    let hash = withdrawal_hash(&withdrawal, asset_id_collateral);
    let private_key = private_key_from_string(prvk).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

pub type CollateralAssetId = U256;

pub fn withdrawal_hash(withdrawal: &Withdraw, asset_id_collateral: &CollateralAssetId) -> HashType {
    let packed_message0;
    let packed_message1;
    // If owner_key is equal to public key, this is a withdrawal of the old API and therefore the
    // transaction type id is different and the owner_key is not part of the message.
    // local has_address = withdrawal.owner_key - withdrawal.base.public_key;
    // TODO: check this
    let has_address = &withdrawal.owner_key != &withdrawal.base.public_key;

    const WITHDRAWAL: U256 = U256([6, 0, 0, 0]);
    const WITHDRAWAL_TO_OWNER_KEY: U256 = U256([7, 0, 0, 0]);

    if !has_address {
        packed_message0 = U256::from(asset_id_collateral);
        packed_message1 = WITHDRAWAL;
    } else {
        let message = hash2(asset_id_collateral, &withdrawal.owner_key);
        packed_message0 = message;
        packed_message1 = WITHDRAWAL_TO_OWNER_KEY;
    }
    let packed_message1 = packed_message1 * POSITION_ID_UPPER_BOUND_U256 + withdrawal.position_id;
    let packed_message1 = packed_message1 * NONCE_UPPER_BOUND_U256 + withdrawal.base.nonce;
    let packed_message1 = packed_message1 * AMOUNT_UPPER_BOUND_U256 + withdrawal.amount;
    let expiration_timestamp = withdrawal.base.expiration_timestamp;
    let packed_message1 =
        packed_message1 * EXPIRATION_TIMESTAMP_UPPER_BOUND_U256 + expiration_timestamp;
    // let packed_message1 = packed_message1 * (2 ** 49);  // Padding.
    let packed_message1 = packed_message1 << 49; // Padding.

    hash2(&packed_message0, &packed_message1)
}

#[cfg(test)]
mod test {
    use crate::common::OrderBase;
    use crate::transaction::withdraw::{sign_withdraw, CollateralAssetId};
    use crate::tx::public_key_type::PublicKeyType;
    use crate::tx::{private_key_from_string, public_key_from_private, Withdraw, WithdrawRequest};

    #[test]
    pub fn test_withdraw() {
        let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let pub_key = public_key_from_private(&private_key);
        let expire = 1684832800u32;
        let pub_key = PublicKeyType::from(pub_key.clone());
        let req = Withdraw {
            base: OrderBase {
                nonce: 1,
                public_key: pub_key.clone(),
                expiration_timestamp: expire,
            },
            position_id: 1,
            amount: 1,
            owner_key: pub_key.clone(),
        };

        // println!("{:#?}", serde_json::to_string(&req).unwrap());
        //c1434d28
        let w = sign_withdraw(req, &CollateralAssetId::from(10), prv_key).unwrap();
        println!("{:#?}", w);
    }

    #[test]
    pub fn test_deserialize() {
        let json = r#"{
        "nonce":"1",
        "public_key":"0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "expiration_timestamp":"1684832800",
        "position_id":"2",
        "amount":"3",
        "eth_address":"0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
        "asset_id": "0x1a"
    }"#;

        let withdraw = serde_json::from_str::<WithdrawRequest>(json);
        assert!(withdraw.is_ok());
    }
}
