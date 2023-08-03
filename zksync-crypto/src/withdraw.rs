use std::convert::TryFrom;
use franklin_crypto::redjubjub::PrivateKey;
use num_bigint::BigInt;
use primitive_types::{H256, U256};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use wasm_bindgen::JsValue;
use zkwasm_rust_sdk::{BabyJubjubPoint, JubjubSignature};

use crate::common::OrderBase;
use crate::constant::{AMOUNT_UPPER_BOUND_U256, EXPIRATION_TIMESTAMP_UPPER_BOUND_U256, NONCE_UPPER_BOUND_U256, POSITION_ID_UPPER_BOUND_U256};

use crate::new_public_key::PublicKeyType;
use crate::{privkey_to_pubkey_internal, sign_musig_without_hash_msg};
use crate::hash::hash2;
use crate::tx::packed_public_key::{PackedPublicKey, private_key_from_string, public_key_from_private};
use crate::tx::TxSignature;
use crate::types::h256_to_u256;
use crate::utils::fr_from_bigint;

// use wasm_bindgen::JsValue;
// use time::OffsetDateTime;
//

pub type AmountType = u64;
pub type PositionIdType = u64;

#[derive(Clone, Debug, Deserialize,Serialize)]
pub struct WithdrawRequest {
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
    mut withdrawal: WithdrawRequest,
    asset_id_collateral: &CollateralAssetId,
    prvk: &str,
) -> Result<WithdrawRequest, JsValue> {
    let hash = withdrawal_hash(&withdrawal, asset_id_collateral);
    let private_key = private_key_from_string(prvk).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_bytes());
    withdrawal.base.signature = sig;
    Ok(withdrawal)
}

pub type CollateralAssetId = U256;
pub type HashType = H256;

pub fn withdrawal_hash(
    withdrawal: &WithdrawRequest,
    asset_id_collateral: &CollateralAssetId,
) -> HashType {
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
        packed_message0 = h256_to_u256(&message);
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


#[test]
pub fn test_withdraw() {
    let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    let private_key = private_key_from_string(prv_key).unwrap();
    let binding = hex::decode(&prv_key).unwrap();
    let prv_bytes = binding.as_slice();
    let pub_key = privkey_to_pubkey_internal(prv_bytes).unwrap();
    let pub_key = public_key_from_private(&private_key);
    let expire = 1684832800i64;
    let pub_key = PublicKeyType::from(pub_key.clone());
    let req = WithdrawRequest {
        base: OrderBase {
            nonce: 1,
            public_key: pub_key.clone(),
            expiration_timestamp: expire,
            signature: JubjubSignature {
                sig_r: BabyJubjubPoint {
                    x: Default::default(),
                    y: Default::default(),
                },
                sig_s: [0; 4],
            },
        },
        position_id: 1,
        amount: 1,
        owner_key: pub_key.clone(),
    };

    let w = sign_withdraw(req, &CollateralAssetId::one(), prv_key).unwrap();
    println!("{:?}", w);
}