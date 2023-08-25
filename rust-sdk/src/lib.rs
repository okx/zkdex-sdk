#![feature(test)]
extern crate test as other_test;

use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::{Error, Result};
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
use num_bigint::BigUint;
use num_traits::Num;
use pairing_ce::bn256::Bn256;
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
use crate::tx::convert::FeConvert;
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

// #[wasm_bindgen(js_name = privateKeyFromSeed)]
pub fn private_key_from_seed(seed: &[u8]) -> Result<String> {
    if seed.len() < 32 {
        return Err(Error::msg("seed is too short"));
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
            return Ok(hex::encode(raw_priv_key));
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

pub fn private_key_to_pubkey_xy(private_key: &str) -> Result<(String, String)> {
    let pri_key = private_key_from_string(private_key)?;
    let packed_pk: PackedPublicKey = public_key_from_private(&pri_key);
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let pk = PublicKey::from_private(&pri_key, p_g, &AltJubjubBn256::new());
    let (pk_x, _) = pk.0.into_xy();
    let x = packed_pk.serialize_packed()?;
    Ok((hex::encode(x), pk_x.to_hex()))
}


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

pub fn sign(private_key: &str, msg: &str) -> Result<JubjubSignature> {
    let hash = HashType::from_str(msg)?;
    let private_key = private_key_from_string(private_key)?;
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_bytes());
    Ok(sig.into())
}

pub fn verify_signature(sig_r: &str, sig_s: &str, pub_key_x: &str, pub_key_y: &str, msg: &str) -> Result<bool> {
    let sig = JubjubSignature::from_str(sig_r, sig_s);
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg)?;
    let pubkey = PublicKeyType::deserialize_str(pub_key_x)?;
    Ok(sig.verify(&pubkey.0, msg.as_bytes()))
}

pub fn is_on_curve(x: &str, y: &str) -> Result<bool> {
    let pubKey = PublicKeyType::deserialize_str(x)?;
    let (pk_x, pk_y) = pubKey.0.0.into_xy();
    let y = Fr::from_hex(y)?;
    Ok(pk_x == y)
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct L1Signature {
    pub x: String,
    pub y: String,
    pub s: String,
    pub pk_x: String,
    pub pk_y: String,
}


pub fn l1_sign(msg: &str, private_key: &str) -> Result<L1Signature> {
    let msg = if msg.starts_with("0x") {
        &msg[2..]
    } else {
        msg
    };

    let b = BigUint::from_str_radix(msg, 16)?;
    let msg = &hex::encode(b.to_bytes_le());
    let private_key = private_key_from_string(private_key)?;
    let msg = HashType::from_str(msg)?;
    let (sig, packed_pk) = TxSignature::sign_msg(&private_key, msg.as_bytes());
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let pk = PublicKey::from_private(&private_key, p_g, &AltJubjubBn256::new());
    let (pk_x, pk_y) = pk.0.into_xy();
    let (x, y) = sig.signature.0.r.into_xy();
    Ok(L1Signature {
        x: "0x".to_owned() + &x.to_hex(),
        y: "0x".to_owned() + &y.to_hex(),
        s: "0x".to_owned() + &sig.signature.0.s.to_hex(),
        pk_x: packed_pk.to_string(),
        pk_y: "0x".to_owned() + &pk_x.to_hex(),
    })
}


#[test]
pub fn test_l1_sign() {
    let msg = "0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3";
    let priv_key = "0x05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
    let s = l1_sign(msg, priv_key).unwrap();
    println!("{:#?}", s.clone());
    let expected = L1Signature {
        x: "0x02c5c5ab6dc2ae39c6bf239acd233c412ceebba1370cd4679ff78c3e57a33f90".to_string(),
        y: "0x1fc29405cb5021e77aec60bfdd9ed43b245569e4cfc6e5720207e015662fd3b9".to_string(),
        s: "0x03fcedddaa3803bc26fa98926d224f13857c1b600a3e99ba01cfcee8d54deaa3".to_string(),
        pk_x: "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9".to_string(),
        pk_y: "0x210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b".to_string(),
    };
    assert!(s == expected)
}

#[test]
pub fn test_verify() {
    let r = "0x353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6";
    let s = "0xc80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502";
    let pub_key_x = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
    let pub_key_y = "210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
    let msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab082f1a520";
    let msg1 = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab00af1a520";
    let ret = verify_signature(r, s, pub_key_x, pub_key_y, msg).unwrap();
    assert!(ret);
    let ret = verify_signature(r, s, pub_key_x, pub_key_y,msg1).unwrap();
    assert!(!ret)
}

#[cfg(test)]
mod test {
    use other_test::Bencher;

    use crate::{hash_transfer, is_on_curve, private_key_from_seed, private_key_to_pubkey_xy, sign_transfer, verify_signature};

    #[bench]
    fn bench_verify_transfer(b: &mut Bencher) {
        b.iter(|| {
            let transfer_req = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
            let hash = hash_transfer(transfer_req).unwrap();
            let sig_r = "0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099";
            let sig_s = "b7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05";
            let pub_key_x = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
            let pub_key_y = "210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
            assert!(verify_signature(sig_r, sig_s, pub_key_x, pub_key_y,&hash).unwrap());
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


    #[test]
    fn test_private_key_to_pubkey_xy() {
        let pri_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let (x, y) = private_key_to_pubkey_xy(pri_key).unwrap();
        println!("x:{x}  y:{y}");
    }

    #[test]
    fn test_private_key_from_seed() {
        let seed = "hello world good life 996 very nice";
        let priKey = private_key_from_seed(seed.as_bytes()).unwrap();
        assert!(priKey == "02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9")
    }

    #[test]
    fn test_is_on_curve() {
        let pri_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let (x, y) = private_key_to_pubkey_xy(pri_key).unwrap();
        println!("x:{x}  y:{y}");
        assert!(is_on_curve(&x, &y).unwrap())
    }
}