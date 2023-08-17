#![feature(test)]
extern crate test as other_test;

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
use primitive_types::{H256, U256};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use anyhow::Result;
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
use crate::tx::packed_public_key::{convert_to_pubkey, PackedPublicKey, private_key_from_string, public_key_from_private, PublicKeyType};
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
pub mod transaction;
pub mod java_bridge;
pub mod javascript_bridge;


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

pub fn privkey_to_pubkey_internal(private_key: &[u8]) -> Result<PublicKey<Engine>, JsValue> {
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


pub fn sign_transfer(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let req: TransferRequest = serde_json::from_str(json).unwrap();
    Ok(transfer::sign_transfer(req, private_key)?)
}


pub fn hash_transfer(json: &str) -> Result<String> {
    let req: TransferRequest = serde_json::from_str(json).unwrap();
    Ok(transfer_hash(&req, 0).encode_hex::<String>())
}


pub fn sign_withdraw(
    json: &str,
    asset_id_collateral: &str,
    private_key: &str,
) -> Result<JubjubSignature> {
    let asset_id = CollateralAssetId::from_str(asset_id_collateral)?;
    let withdraw: WithdrawRequest = serde_json::from_str(json)?;
    Ok(withdraw::sign_withdraw(withdraw, &asset_id.clone(), private_key)?)
}


pub fn hash_withdraw(json: &str, asset_id_collateral: &str) -> Result<String> {
    let req: WithdrawRequest = serde_json::from_str(json)?;
    let asset_id = CollateralAssetId::from_str(asset_id_collateral)?;
    Ok(withdrawal_hash(&req, &asset_id).encode_hex::<String>())
}


pub fn sign_limit_order(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let req: LimitOrderRequest = serde_json::from_str(json)?;
    Ok(limit_order::sign_limit_order(req, private_key)?)
}


pub fn hash_limit_order(json: &str) -> Result<String> {
    let req: LimitOrderRequest = serde_json::from_str(json)?;
    Ok(limit_order_hash(&req).encode_hex::<String>())
}


pub fn sign_liquidate(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(limit_order::sign_limit_order(req.liquidator_order, private_key)?)
}


pub fn hash_liquidate(json: &str) -> Result<String> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(limit_order_hash(&req.liquidator_order).encode_hex::<String>())
}


pub fn sign_signed_oracle_price(
    json: &str,
    private_key: &str,
) -> Result<JubjubSignature> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    Ok(oracle_price::sign_signed_oracle_price(req, private_key)?)
}


pub fn hash_signed_oracle_price(json: &str) -> Result<String> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    Ok(signed_oracle_price_hash(&req).encode_hex::<String>())
}


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

pub fn verify_signature(sig_r: &str, sig_s: &str, pub_key: &str, msg: &str) -> Result<bool> {
    let sig = JubjubSignature::from_str(sig_r, sig_s);
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg)?;
    let pubkey = PublicKeyType::deserialize_str(pub_key)?;
    Ok(sig.verify(&pubkey.0, msg.as_bytes()))
}


#[test]
pub fn test_verify() {
    let r = "0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6";
    let s = "0xc80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502";
    let pub_key = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
    let msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520";
    let msg1 = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab00af1a520";
    let ret = verify_signature(r, s, pub_key, msg).unwrap();
    assert!(ret);
    let ret = verify_signature(r, s, pub_key, msg1).unwrap();
    assert!(!ret);

    let x = "1cb6b94240a2f5a68b6e9b2197916714ec8b210dda99eeef69dd439c6324fe71";
    let y = "19b2665d3bc3c68205ca714a8e02356d6fb48c90f5280bef1dfd183889c536d0";
    let s = "53da455169c654cc5bc6808a0838b927e89afd5e8667a51116877a69196b5e00";

    let sig = JubjubSignature::from_x_y(x, y, s);
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str("0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3").unwrap();
    let pubkey = PublicKeyType::deserialize_str(pub_key).unwrap();
    assert!(sig.verify(&pubkey.0, msg.as_bytes()))
}


#[test]
pub fn test_verify2() {
    let sig_x = "1cb6b94240a2f5a68b6e9b2197916714ec8b210dda99eeef69dd439c6324fe71";
    let sig_y = "19b2665d3bc3c68205ca714a8e02356d6fb48c90f5280bef1dfd183889c536d0";
    let sig_s = "53da455169c654cc5bc6808a0838b927e89afd5e8667a51116877a69196b5e00";
    let pub_key_x = "0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
    let pub_key_y = "0x2917e2b130d3c0b999870048591eff578da75c0b5fb1c4c5c99a7fd9cbd3cb42";
    let hash = "0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3";
    let err_hash = "0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f9";

    assert!(verify_signature2(sig_x,sig_y,sig_s,pub_key_x,pub_key_y,hash));
    assert_eq!(verify_signature2(sig_x,sig_y,sig_s,pub_key_x,pub_key_y,err_hash), false);
}


pub fn verify_signature2(sig_x: &str, sig_y: &str, sig_s: &str, pk_x: &str, pk_y: &str, msg: &str) -> bool {
    let sig = JubjubSignature::from_x_y(sig_x, sig_y, sig_s);
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg).unwrap();
    let pubk = PackedPublicKey::from((u256_to_h256(U256::from_str_radix(pk_x, 16).unwrap()),
                                      u256_to_h256(U256::from_str_radix(pk_y, 16).unwrap())));
    let pubkey = PublicKeyType::deserialize_str(pubk.to_string().as_str()).unwrap();
    sig.verify(&pubkey.0, msg.as_bytes())
}


pub fn pubkey_from_x_y(x: &str, y: &str) -> PublicKeyType {
    let pubk = PackedPublicKey::from((u256_to_h256(U256::from_str_radix(x, 16).unwrap()),
                                      u256_to_h256(U256::from_str_radix(y, 16).unwrap())));
    PublicKeyType::deserialize_str(pubk.to_string().as_str()).unwrap()
}

#[cfg(test)]
mod test {
    use other_test::Bencher;
    use crate::{hash_transfer, sign_transfer, verify_signature};


    #[bench]
    fn bench_verify_transfer(b: &mut Bencher) {
        b.iter(|| {
            let transfer_req = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
            let hash = hash_transfer(transfer_req).unwrap();
            let sig_r = "0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099";
            let sig_s = "b7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05";
            let pub_key = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
            assert!(verify_signature(sig_r, sig_s, pub_key, &hash).unwrap());
        })
    }


    #[bench]
    fn bench_sign_transfer(b: &mut Bencher) {
        b.iter(|| {
            let transfer_req = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
            let pri_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
            assert!(sign_transfer(transfer_req, pri_key).is_ok());
        })
    }
}