use std::convert::TryFrom;
use std::str::FromStr;

use franklin_crypto::{
    alt_babyjubjub::{AltJubjubBn256, edwards, FixedGenerators, fs::FsRepr},
    bellman::pairing::ff::{PrimeField, PrimeFieldRepr},
    eddsa::{PrivateKey, PublicKey, Signature as EddsaSignature},
    jubjub::JubjubEngine,
};
pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
use hex::ToHex;
use jni::objects::*;
use num_traits::Num;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

pub use convert::*;
pub use format::*;
pub use serde_wrapper::*;

use crate::transaction::{limit_order, oracle_price, transfer, withdraw};
use crate::transaction::limit_order::{limit_order_hash, LimitOrderRequest};
use crate::transaction::liquidate::Liquidate;
use crate::transaction::oracle_price::{signed_oracle_price_hash, SignedOraclePrice};
use crate::transaction::transfer::{transfer_hash, TransferRequest};
use crate::transaction::types::HashType;
use crate::transaction::withdraw::{CollateralAssetId, WithdrawRequest};
use crate::tx::{h256_to_u256, u256_to_h256};
use crate::tx::packed_public_key::{convert_to_pubkey, PublicKeyType};
use crate::tx::packed_signature::PackedSignature;
use crate::tx::sign::TxSignature;
use crate::tx::withdraw::withdrawal_hash;
use crate::utils::set_panic_hook;
use crate::zkw::JubjubSignature;

mod common;
mod constant;
mod convert;
mod fr;

mod models;
mod new_public_key;

mod types;
mod utils;


pub mod byte_tools;
pub mod env_tools;
pub mod format;
mod hash;
pub mod serde_wrapper;
pub mod tx;
mod zkw;
mod transaction;
pub mod java_bridge;


const PACKED_POINT_SIZE: usize = 32;
const PACKED_SIGNATURE_SIZE: usize = 64;

pub type Fs = <Engine as JubjubEngine>::Fs;

thread_local! {
    pub static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}
lazy_static::lazy_static! {
    // pub static ref RESCUE_HASHER: BabyRescueHasher = BabyRescueHasher::default();
    pub static ref RESCUE_PARAMS_CONST: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

pub type Signature = EddsaSignature<Engine>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
/// This method initializes params for current thread, otherwise they will be initialized when signing
/// first message.
pub fn zksync_crypto_init() {
    JUBJUB_PARAMS.with(|_| {});
    RESCUE_PARAMS.with(|_| {});
    set_panic_hook();
}

#[wasm_bindgen(js_name = privateKeyFromSeed)]
pub fn private_key_from_seed(seed: &[u8]) -> Result<Vec<u8>, JsValue> {
    if seed.len() < 32 {
        return Err(JsValue::from_str("Seed is too short"));
    };

    let sha256_bytes = |input: &[u8]| -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(input);
        hasher.result().to_vec()
    };

    let mut effective_seed = sha256_bytes(seed);

    loop {
        let raw_priv_key = sha256_bytes(&effective_seed);
        let mut fs_repr = FsRepr::default();
        fs_repr
            .read_be(&raw_priv_key[..])
            .expect("failed to read raw_priv_key");
        if Fs::from_repr(fs_repr).is_ok() {
            return Ok(raw_priv_key);
        } else {
            effective_seed = raw_priv_key;
        }
    }
}

fn read_signing_key(private_key: &[u8]) -> Result<PrivateKey<Engine>, JsValue> {
    let mut fs_repr = FsRepr::default();
    fs_repr
        .read_be(private_key)
        .map_err(|_| JsValue::from_str("couldn't read private key repr"))?;
    Ok(PrivateKey::<Engine>(
        Fs::from_repr(fs_repr).expect("couldn't read private key from repr"),
    ))
}

fn privkey_to_pubkey_internal(private_key: &[u8]) -> Result<PublicKey<Engine>, JsValue> {
    let p_g = FixedGenerators::SpendingKeyGenerator;

    let sk = read_signing_key(private_key)?;

    Ok(JUBJUB_PARAMS.with(|params| PublicKey::from_private(&sk, p_g, params)))
}

#[wasm_bindgen(js_name = pubKeyHash)]
pub fn pub_key_hash(pubkey: &[u8]) -> Result<Vec<u8>, JsValue> {
    let pubkey = JUBJUB_PARAMS
        .with(|params| PublicKey::read(pubkey, params))
        .map_err(|_| JsValue::from_str("couldn't read public key"))?;
    Ok(utils::pub_key_hash(&pubkey))
}

#[wasm_bindgen]
pub fn private_key_to_pubkey_hash(private_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    Ok(utils::pub_key_hash(&privkey_to_pubkey_internal(
        private_key,
    )?))
}

#[wasm_bindgen]
pub fn sign_transfer(json: &str, private_key: &str) -> Result<String, JsValue> {
    let req: TransferRequest = serde_json::from_str(json).unwrap();
    let ret = transfer::sign_transfer(req, private_key)?;
    match serde_json::to_string(&ret) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
    }
}

#[wasm_bindgen]
pub fn hash_transfer(json: &str) ->Result<String,JsValue>{
    let req: TransferRequest = serde_json::from_str(json).unwrap();
    Ok(transfer_hash(&req,0).encode_hex::<String>())
}

#[wasm_bindgen]
pub fn sign_withdraw(
    json: &str,
    asset_id_collateral: &str,
    private_key: &str,
) -> Result<String, JsValue> {
    let asset_id = CollateralAssetId::from_str(asset_id_collateral).unwrap();
    let withdraw: WithdrawRequest = serde_json::from_str(json).unwrap();
    let withdraw = withdraw::sign_withdraw(withdraw, &asset_id, private_key)?;
    match serde_json::to_string(&withdraw) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
    }
}

#[wasm_bindgen]
pub fn hash_withdraw(json: &str,asset_id_collateral: &str) ->Result<String,JsValue>{
    let req: WithdrawRequest = serde_json::from_str(json).unwrap();
    let asset_id = CollateralAssetId::from_str(asset_id_collateral).unwrap();
    Ok(withdrawal_hash(&req, &asset_id).encode_hex::<String>())
}

#[wasm_bindgen]
pub fn sign_limit_order(json: &str, private_key: &str) -> Result<String, JsValue> {
    let req: LimitOrderRequest = serde_json::from_str(json).unwrap();
    let ret = limit_order::sign_limit_order(req, private_key)?;
    match serde_json::to_string(&ret) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
    }
}

#[wasm_bindgen]
pub fn hash_limit_order(json: &str) ->Result<String,JsValue>{
    let req: LimitOrderRequest = serde_json::from_str(json).unwrap();
    Ok(limit_order_hash(&req).encode_hex::<String>())
}

#[wasm_bindgen]
pub fn sign_liquidate(json: &str, private_key: &str) -> Result<String, JsValue> {
    let req: Liquidate = serde_json::from_str(json).unwrap();
    let ret = limit_order::sign_limit_order(req.liquidator_order, private_key)?;
    match serde_json::to_string(&ret) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
    }
}

#[wasm_bindgen]
pub fn hash_liquidate(json: &str) ->Result<String,JsValue>{
    let req: Liquidate = serde_json::from_str(json).unwrap();
    Ok(limit_order_hash(&req.liquidator_order).encode_hex::<String>())
}

#[wasm_bindgen]
pub fn sign_signed_oracle_price(
    json: &str,
    private_key: &str,
) -> Result<String, JsValue> {
    let req: SignedOraclePrice = serde_json::from_str(json).unwrap();
    let ret = oracle_price::sign_signed_oracle_price(req, private_key)?;
    match serde_json::to_string(&ret) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
    }
}

#[wasm_bindgen]
pub fn hash_signed_oracle_price(json: &str) ->Result<String,JsValue>{
    let req: SignedOraclePrice = serde_json::from_str(json).unwrap();
    Ok(signed_oracle_price_hash(&req).encode_hex::<String>())
}

#[wasm_bindgen]
pub fn private_key_to_pubkey(private_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut pubkey_buf = Vec::with_capacity(PACKED_POINT_SIZE);

    let pubkey = privkey_to_pubkey_internal(private_key)?;
    pubkey
        .write(&mut pubkey_buf)
        .expect("failed to write pubkey to buffer");

    Ok(pubkey_buf)
}

#[wasm_bindgen]
pub fn private_key_to_pubkey_with_xy(private_key: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut pubkey_buf = Vec::with_capacity(PACKED_POINT_SIZE + PACKED_POINT_SIZE);
    let pubkey = privkey_to_pubkey_internal(private_key)?;
    let (a, b) = pubkey.0.into_xy();
    a.into_repr()
        .write_be(&mut pubkey_buf)
        .expect("failed to write a to buffer");
    b.into_repr()
        .write_be(&mut pubkey_buf)
        .expect("failed to write b to buffer");
    Ok(pubkey_buf)
}

#[wasm_bindgen]
pub fn verify_signature(sig_r: &str, sig_s: &str, pub_key: &str, msg: &str) -> Result<bool, JsValue> {
    let sig = JubjubSignature::from_str(sig_r, sig_s);
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg).unwrap();
    let pubkey = PublicKeyType::deserialize_str(pub_key).unwrap();
    Ok(sig.verify(&pubkey.0, msg.as_bytes()))
}

#[test]
fn test_verify() {
    let r = "353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6";
    let s = "c80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502";
    let pub_key = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
    let msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520";
    let msg1 = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab00af1a520";
    let ret = verify_signature(r, s, pub_key, msg).unwrap();
    assert!(ret);
    let ret = verify_signature(r, s, pub_key, msg1).unwrap();
    assert!(!ret)
}


#[test]
pub fn test_sign_withdraw() {
    let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    let binding = hex::decode(&prv_key).unwrap();
    let prv_bytes = binding.as_slice();
    let pub_key = privkey_to_pubkey_internal(prv_bytes).unwrap();
    let pub_key = PublicKeyType(pub_key);
    let pub_str = hex::encode(pub_key.serialize_packed().unwrap());
    println!("pub_str:{:?}", pub_str);
    let data = r#"
    {"type":"WITHDRAWAL_TO_ADDRESS","nonce":"2","public_key":"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9","expiration_timestamp":"1687753430","signature":{"r":"0x0cd8af0b942b4a24a9ebceceef6dc85573287da5847f6f5c5eab4016f5a940a5","s":"0x0c4631784711a03d4f50c05451ed9f495751ebeff43f3ce54cdde360873bba04"},"position_id":"10001","amount":"100000000","eth_address":"0xDdF1706FE25a3018e5517D60d02dE2d99BED310D"}
    "#;

    let signature = sign_withdraw(
        data,
        "0x2c016e767840eb2cf7541b50619d9cafec1fbef4e46f29d2303452a7e19e222",
        prv_key,
    )
        .unwrap();
    let r = &signature[32..64];
    let s = &signature[64..];
    println!("r:{:?},s:{:?}", hex::encode(r), hex::encode(s));
}
