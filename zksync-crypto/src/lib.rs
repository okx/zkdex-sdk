//! Utils for signing zksync transactions.
//! This crate is compiled into wasm to be used in `zksync.js`.
use std::str::FromStr;

pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
use franklin_crypto::{
    alt_babyjubjub::{edwards, fs::FsRepr, AltJubjubBn256, FixedGenerators},
    bellman::pairing::ff::{PrimeField, PrimeFieldRepr},
    eddsa::{PrivateKey, PublicKey, Seed, Signature as EddsaSignature},
    jubjub::JubjubEngine,
};
use num_bigint::BigInt;
use num_traits::Num;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

pub use convert::*;
pub use format::*;
use js_ok_zksync_crypto::merkle_tree::rescue_hasher::BabyRescueHasher;
use js_types::common::packed_public_key::PublicKeyType;
pub use serde_wrapper::*;

use crate::limit_order::LimitOrderRequest;
use crate::transfer::TransferRequest;
use crate::tx::h256_to_u256;
use crate::tx::packed_signature::{PackedSignature, SignatureSerde};
use crate::tx::sign::TxSignature;
use crate::utils::set_panic_hook;
use crate::withdraw::{CollateralAssetId, WithdrawRequest};

mod common;
mod constant;
mod convert;
mod fr;
mod limit_order;
mod models;
mod new_public_key;
mod transfer;
mod types;
mod utils;
mod withdraw;

pub mod byte_tools;
pub mod env_tools;
pub mod format;
mod hash;
pub mod serde_wrapper;
pub mod tx;
mod zkw;

const PACKED_POINT_SIZE: usize = 32;
const PACKED_SIGNATURE_SIZE: usize = 64;

pub type Fs = <Engine as JubjubEngine>::Fs;

thread_local! {
    pub static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}
lazy_static::lazy_static! {
    pub static ref RESCUE_HASHER: BabyRescueHasher = BabyRescueHasher::default();
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

#[derive(Debug, Deserialize, Serialize)]
struct A {
    field1: String,
    field2: u64,
}

#[wasm_bindgen]
pub fn printA(jsonBytes: &str) -> u64 {
    let a: A = serde_json::from_str(jsonBytes).unwrap();
    a.field2 + a.field2 + a.field2
}

#[wasm_bindgen]
pub fn printAC(jsonBytes: &str) -> u64 {
    let a: A = serde_json::from_str(jsonBytes).unwrap();
    a.field2 + a.field2 + a.field2
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
pub fn sign_limit_order(json: &str, private_key: &str) -> Result<String, JsValue> {
    let req: LimitOrderRequest = serde_json::from_str(json).unwrap();
    let ret = limit_order::sign_limit_order(req, private_key)?;
    match serde_json::to_string(&ret) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
    }
}

fn hex_string_to_bigint(s: &str) -> BigInt {
    let num = BigInt::from_str_radix(
        s.trim_start_matches("0x")
            .trim_start_matches("0X")
            .trim_start_matches("-0x")
            .trim_start_matches("-0X"),
        16,
    )
    .unwrap();
    if s.starts_with('-') {
        -num
    } else {
        num
    }
}

#[wasm_bindgen]
pub fn printC(jsonBytes: &str) -> u64 {
    1
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

#[wasm_bindgen(js_name = "rescueHash")]
pub fn rescue_hash_tx_msg(msg: &[u8]) -> Vec<u8> {
    utils::rescue_hash_tx_msg(msg)
}

/// `msg` should be represented by 2 concatenated
/// serialized orders of the swap transaction
#[wasm_bindgen(js_name = "rescueHashOrders")]
pub fn rescue_hash_orders(msg: &[u8]) -> Vec<u8> {
    utils::rescue_hash_orders(msg)
}

#[wasm_bindgen]
/// We use musig Schnorr signature scheme.
/// It is impossible to restore signer for signature, that is why we provide public key of the signer
/// along with signature.
/// [0..32] - packed public key of signer.
/// [32..64] - packed r point of the signature.
/// [64..96] - s poing of the signature.
pub fn sign_musig_without_hash_msg(private_key: &[u8], msg: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut packed_full_signature = Vec::with_capacity(PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE);
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let private_key = read_signing_key(private_key)?;

    {
        let public_key =
            JUBJUB_PARAMS.with(|params| PublicKey::from_private(&private_key, p_g, params));
        public_key
            .write(&mut packed_full_signature)
            .expect("failed to write pubkey to packed_point");
    };

    let signature = JUBJUB_PARAMS.with(|jubjub_params| {
        RESCUE_PARAMS.with(|rescue_params| {
            let seed = Seed::deterministic_seed(&private_key, &msg);
            private_key.musig_rescue_sign(&msg, &seed, p_g, rescue_params, jubjub_params)
        })
    });

    signature
        .r
        .write(&mut packed_full_signature)
        .expect("failed to write signature");
    signature
        .s
        .into_repr()
        .write_le(&mut packed_full_signature)
        .expect("failed to write signature repr");

    assert_eq!(
        packed_full_signature.len(),
        PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE,
        "incorrect signature size when signing"
    );

    Ok(packed_full_signature)
}

#[wasm_bindgen]
/// We use musig Schnorr signature scheme.
/// It is impossible to restore signer for signature, that is why we provide public key of the signer
/// along with signature.
/// [0..32] - packed public key of signer.
/// [32..64] - packed r point of the signature.
/// [64..96] - s poing of the signature.
pub fn sign_musig(private_key: &[u8], msg: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut packed_full_signature = Vec::with_capacity(PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE);
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let private_key = read_signing_key(private_key)?;

    {
        let public_key =
            JUBJUB_PARAMS.with(|params| PublicKey::from_private(&private_key, p_g, params));
        public_key
            .write(&mut packed_full_signature)
            .expect("failed to write pubkey to packed_point");
    };

    let signature = JUBJUB_PARAMS.with(|jubjub_params| {
        RESCUE_PARAMS.with(|rescue_params| {
            let hashed_msg = utils::rescue_hash_tx_msg(msg);
            let seed = Seed::deterministic_seed(&private_key, &hashed_msg);
            private_key.musig_rescue_sign(&hashed_msg, &seed, p_g, rescue_params, jubjub_params)
        })
    });

    signature
        .r
        .write(&mut packed_full_signature)
        .expect("failed to write signature");
    signature
        .s
        .into_repr()
        .write_le(&mut packed_full_signature)
        .expect("failed to write signature repr");

    assert_eq!(
        packed_full_signature.len(),
        PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE,
        "incorrect signature size when signing"
    );

    Ok(packed_full_signature)
}

#[wasm_bindgen]
pub fn verify_musig(msg: &[u8], signature: &[u8]) -> Result<bool, JsValue> {
    if signature.len() != PACKED_POINT_SIZE + PACKED_SIGNATURE_SIZE {
        return Err(JsValue::from_str("Signature length is not 96 bytes. Make sure it contains both the public key and the signature itself."));
    }

    let pubkey = &signature[..PACKED_POINT_SIZE];
    let pubkey = JUBJUB_PARAMS
        .with(|params| edwards::Point::read(&*pubkey, params).map(PublicKey))
        .map_err(|_| JsValue::from_str("couldn't read public key"))?;

    let signature = deserialize_signature(&signature[PACKED_POINT_SIZE..])?;

    let msg = utils::rescue_hash_tx_msg(msg);
    let value = JUBJUB_PARAMS.with(|jubjub_params| {
        RESCUE_PARAMS.with(|rescue_params| {
            pubkey.verify_musig_rescue(
                &msg,
                &signature,
                FixedGenerators::SpendingKeyGenerator,
                rescue_params,
                jubjub_params,
            )
        })
    });

    Ok(value)
}

fn deserialize_signature(bytes: &[u8]) -> Result<Signature, JsValue> {
    let (r_bar, s_bar) = bytes.split_at(PACKED_POINT_SIZE);

    let r = JUBJUB_PARAMS
        .with(|params| edwards::Point::read(r_bar, params))
        .map_err(|_| JsValue::from_str("Failed to parse signature"))?;

    let mut s_repr = FsRepr::default();
    s_repr
        .read_le(s_bar)
        .map_err(|_| JsValue::from_str("Failed to parse signature"))?;

    let s = <Engine as JubjubEngine>::Fs::from_repr(s_repr)
        .map_err(|_| JsValue::from_str("Failed to parse signature"))?;

    Ok(Signature { r, s })
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
