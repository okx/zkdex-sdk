#![feature(test)]
extern crate core;
extern crate test as other_test;

use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use anyhow::{Error, Result};
use franklin_crypto::{
    alt_babyjubjub::{AltJubjubBn256, FixedGenerators, fs::FsRepr},
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

use crate::felt::LeBytesConvert;
use crate::hash_type::{hash_type_to_string_with_0xprefix, string_to_hash_type};
use crate::transaction::{limit_order, oracle_price, transfer, withdraw};
use crate::transaction::limit_order::{limit_order_hash, LimitOrderRequest};
use crate::transaction::liquidate::Liquidate;
use crate::transaction::oracle_price::{signed_oracle_price_hash, SignedOraclePrice};
use crate::transaction::transfer::{Transfer, transfer_hash};
use crate::transaction::types::HashType;
use crate::transaction::withdraw::{Withdraw, WithdrawRequest};
use crate::tx::{h256_to_u256, u256_to_h256};
use crate::tx::convert::FeConvert;
use crate::tx::packed_public_key::{convert_to_pubkey, PackedPublicKey, private_key_from_string, public_key_from_private, PublicKeyType};
use crate::tx::packed_signature::PackedSignature;
use crate::tx::sign::TxSignature;
use crate::tx::withdraw::withdrawal_hash;
use crate::utils::set_panic_hook;
use crate::zkw::{BabyJubjubPoint, JubjubSignature};

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
pub fn zkdex_init() {
    JUBJUB_PARAMS.with(|_| {});
    RESCUE_PARAMS.with(|_| {});
    set_panic_hook();
}


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
            return Ok("0x".to_string() + &hex::encode(raw_priv_key));
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

pub fn sign_transfer(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let req: Transfer = serde_json::from_str(json).unwrap();
    Ok(transfer::sign_transfer(req, private_key)?)
}


pub fn hash_transfer(json: &str) -> Result<String> {
    let req: Transfer = serde_json::from_str(json).unwrap();
    Ok(hash_type_to_string_with_0xprefix(transfer_hash(&req, 0)))
}


pub fn sign_withdraw(
    json: &str,
    private_key: &str,
) -> Result<JubjubSignature> {
    let withdrawReq: WithdrawRequest = serde_json::from_str(json)?;
    let withdraw = Withdraw {
        base: withdrawReq.base,
        position_id: withdrawReq.position_id,
        amount: withdrawReq.amount,
        owner_key: withdrawReq.owner_key,
    };
    Ok(withdraw::sign_withdraw(withdraw, &withdrawReq.asset_id, private_key)?)
}


pub fn hash_withdraw(json: &str) -> Result<String> {
    let withdrawReq: WithdrawRequest = serde_json::from_str(json)?;
    let withdraw = Withdraw {
        base: withdrawReq.base,
        position_id: withdrawReq.position_id,
        amount: withdrawReq.amount,
        owner_key: withdrawReq.owner_key,
    };
    Ok(hash_type_to_string_with_0xprefix(withdrawal_hash(&withdraw, &withdrawReq.asset_id)))
}


pub fn sign_limit_order(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let req: LimitOrderRequest = serde_json::from_str(json)?;
    Ok(limit_order::sign_limit_order(req, private_key)?)
}


pub fn hash_limit_order(json: &str) -> Result<String> {
    let req: LimitOrderRequest = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(limit_order_hash(&req)))
}


pub fn sign_liquidate(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(limit_order::sign_limit_order(req.liquidator_order, private_key)?)
}


pub fn hash_liquidate(json: &str) -> Result<String> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix( limit_order_hash(&req.liquidator_order)))
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
    Ok(hash_type_to_string_with_0xprefix(signed_oracle_price_hash(&req)))
}

pub fn private_key_to_pubkey_xy(private_key: &str) -> Result<(String, String)> {
    let pri_key = private_key_from_string(private_key)?;
    let packed_pk: PackedPublicKey = public_key_from_private(&pri_key);
    Ok(pub_key_to_xy(&packed_pk.to_string())?)
}

pub fn pub_key_to_xy(pub_key: &str) -> Result<(String, String)> {
    let pub_key = pub_key.trim_start_matches("0x").trim_start_matches("0X");
    let bytes = hex::decode(pub_key)?;
    let packed_pk = PackedPublicKey::deserialize_packed(bytes.as_slice())?;

    let jubjub_pk:BabyJubjubPoint = packed_pk.into();
    let mut x_point = [0; 32];
    jubjub_pk.x.to_big_endian(&mut x_point);

    Ok(("0x".to_owned() + &pub_key.to_string(), "0x".to_owned() + &hex::encode(x_point.to_vec())))
}

#[test]
fn test_sign12() {
    let hash = "0x231a10c16831385e52dd1dad738077e09f0b75e680c0c7ed76a0d76103815f53";
    let pri =  "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
    sign(pri, hash).unwrap();
}

pub fn sign(private_key: &str, msg: &str) -> Result<JubjubSignature> {
    let hash = string_to_hash_type(msg)?;
    let private_key = private_key_from_string(private_key)?;
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

pub fn verify_signature(sig_r: &str, sig_s: &str, pub_key_x: &str, pub_key_y: &str, msg: &str) -> Result<bool> {
    let sig = JubjubSignature::from_str(sig_r, sig_s);
    let sig = PackedSignature::from(sig);
    let msg = string_to_hash_type(msg)?;
    let packed_pk = PackedPublicKey::try_from(pub_key_x.to_string())?;
    let jubjub_pk:BabyJubjubPoint = packed_pk.into();
    let pk = convert_to_pubkey(&jubjub_pk.x, &jubjub_pk.y)?;
    Ok(sig.verify(&pk, msg.as_le_bytes()))
}

pub fn is_on_curve(x: &str, y: &str) -> Result<bool> {
    let x = x.trim_start_matches("0x").trim_start_matches("0X");
    let y = y.trim_start_matches("0x").trim_start_matches("0X");
    let (x1,y1) = pub_key_to_xy(x)?;
    let x1 = x1.trim_start_matches("0x").trim_start_matches("0X");
    let y1 = y1.trim_start_matches("0x").trim_start_matches("0X");
    Ok(x1==x && y1 == y)
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

    let private_key = private_key_from_string(private_key)?;
    let msg = string_to_hash_type(msg)?;
    let (sig, packed_pk) = TxSignature::sign_msg(&private_key, msg.as_le_bytes());
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

pub fn reverse_hex(str: &str) -> anyhow::Result<String>{
    let mut ret = hex::decode(str)?;
    ret.as_mut_slice().reverse();
    Ok(hex::encode(ret))
}


#[test]
pub fn test_l1_sign() {
    let msg = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea";
    let priv_key = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
    let s = l1_sign(msg, priv_key).unwrap();
    println!("{:#?}", s.clone());
    let expected = L1Signature {
        x: "0x2a10ad3523853ca4db3951fd5cb369c4b3209f11440afa326ab39288b50523b8".to_string(),
        y: "0x1a3f4ef8c96e77c41ede330c83c8eb1f8030a8d6792cf7dfac5c07cc7efe1843".to_string(),
        s: "0x05bff783626c524cdd04cea7d9168b44c47c89fc2e86e08cb922e016e811ddd0".to_string(),
        pk_x: "0x96c4d93a49c8159e27542601ba19fdfce52b3e9b43dafaefe9aa9cd32efded86".to_string(),
        pk_y: "0x0cc8a68b8dba85bd5418e308b34439ddffca3a0f6589a32f02adf60da6e73f55".to_string(),
    };
    assert!(s == expected)
}

#[test]
pub fn test_verify() {
    let r = "0xa2726da3b5111e07834effa5c81aaecf576e56f52091965a92c4326ea7063226";
    let s = "0x3c9356695594551802208b3ef4fc268ea80bca27fe3a8848ef64e1f85c7f728";
    let pub_key_x = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
    let pub_key_y = "210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
    let msg = "0x15318118b6f0c3fe74923ff85fbd7d225c75672469280ec9db92509e46bff197";
    let msg1 = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d160ab00af1a520";
    let ret = verify_signature(r, s, pub_key_x, pub_key_y, msg).unwrap();
    assert!(ret);
    let ret = verify_signature(r, s, pub_key_x, pub_key_y, msg1).unwrap();
    assert!(!ret)
}

#[test]
pub fn test_sign_oracle_price() {
    let json1 = r#"
    {
  "signer_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
  "external_price": "28409392522000000000000",
  "timestamp": "1693907824",
  "signed_asset_id": "0x425443555344434f4b580000000000005374437277"
    }
    "#;
    let pri1 = "01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
    let sig1 = sign_signed_oracle_price(json1,pri1).unwrap();
    let hash1 = hash_signed_oracle_price(json1).unwrap();
    println!("hash1: {}", hash1);
    println!("sig1: {}", serde_json::to_string(&sig1).unwrap());

    let json2 = r#"
    {
  "signer_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
  "external_price": "6652695000000000000",
  "timestamp": "1693971434",
  "signed_asset_id": "0x534f4c555344434f4b580000000000005374437277"
    }
    "#;
    let pri2 = "0376204fa0b554ee3d8a03c6ccdb73f7b98d1965fbeaa3a9f88723669a23893f";
    let sig2 = sign_signed_oracle_price(json2,pri2).unwrap();
    println!("sig2: {}", serde_json::to_string(&sig2).unwrap());

    let json3 = r#"
    {
  "signer_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
  "external_price": "1854072360000000000000",
  "timestamp": "1693971569",
  "signed_asset_id": "0x455448555344434f4b580000000000005374437277"
    }
    "#;
    let pri3 = "060a45bcd72c9e3c82bc1c57f63ad15b25f56bb13ce01d15fd4ab3f8f2de35bb";
    let sig3 = sign_signed_oracle_price(json3,pri3).unwrap();
    println!("sig3: {}", serde_json::to_string(&sig3).unwrap());

    let pri_arr = vec![pri1, pri2, pri3];
    for x in pri_arr {
        let pri = private_key_from_string(x).unwrap();
        let pk = public_key_from_private(&pri);
        println!("{}", pk.to_string())
    }
    // let hash = hash_signed_oracle_price(json1).unwrap();
    // let a = verify_signature("0x27b3ab353b810ab24cbec9ebcacd54afdea9dc906d4006fb455c4500163d0032","0x1f09b981ac723863aef7cbc89529e67ede075246b07a27d4cdea470264f144a","0x87e5235c9c3916ef2b0def77111366ecef72914613f52febad308440b6463f83","0x87e5235c9c3916ef2b0def77111366ecef72914613f52febad308440b6463f83",&hash.to_string());
    // assert!(a.unwrap())
}

#[cfg(test)]
mod test {
    use other_test::Bencher;

    use crate::{hash_transfer, is_on_curve, private_key_from_seed, private_key_to_pubkey_xy, pub_key_to_xy, reverse_hex, sign_transfer, verify_signature};
    use crate::tx::{private_key_from_string, public_key_from_private};

    #[bench]
    fn bench_verify_transfer(b: &mut Bencher) {
        b.iter(|| {
            let transfer_req = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
            let hash = hash_transfer(transfer_req).unwrap();
            let sig_r = "0c2b9b07a37711498dc9cdd2585c66b07d110fc69c2b31e43376cdf16d266099";
            let sig_s = "b7d9032ae2e7ff265910db676685e60eb22aa01f1e6c6587beb024373b58fa05";
            let pub_key_x = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
            let pub_key_y = "210add7128da8f626145394a55df3e022f3994164c31803b3c8ac18edc91730b";
            assert!(verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, &hash).unwrap());
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
        let pri_key = "0x028dd913a169cf3732c306959e9c2a66a0075663e54e086977ed71c61fd7c273";
        let (x, y) = private_key_to_pubkey_xy(pri_key).unwrap();
        let prk = private_key_from_string(pri_key).unwrap();
        let pk = public_key_from_private(&prk);
        println!("{:?}", pk.to_string());
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
        let pri_key = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
        let (x, y) = private_key_to_pubkey_xy(pri_key).unwrap();
        println!("x:{x}  y:{y}");
        assert!(is_on_curve(&x, &y).unwrap())
    }

    #[test]
    fn test_pub_key_to_xy() {
        let pub_key = "8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let (x,y) = pub_key_to_xy(pub_key).unwrap();
        println!("x:{x} y:{y}")
    }

    #[test]
    fn test_reverse_hex() {
        let num = "12ba9000";
        println!("{:?}",reverse_hex(num).unwrap());
    }


}

#[test]
fn test_hash_liquidate() {

    let pri = private_key_from_seed("hldsadsadsadsadsadsadsadsaddsdsa".as_bytes()).unwrap();
    println!("{}", &pri);
    let pk = private_key_to_pubkey_xy(&pri).unwrap();
    println!("{:?} : {}", pk.0, pk.1);
    let json = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
    let hash = hash_liquidate(json).unwrap();
    println!("{:?}", &hash);

    assert!(hash == "0x00fe1979a24a2e94a59facd5084432a2a403ec0132549859289b2d49e4ec9750");
}

#[test]
fn test_hash_limit_order(){
    let json1 = r#"
        {
      "nonce": "1",
      "public_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
      "expiration_timestamp": "3608164305",
      "signature": {
        "r": "0xaff60be77ca88a6bd9f25c06ee58e80cc27567022cb75a39d8de9bfad32b8f20",
        "s": "0xc876eb02c24f639e47383e5da2a24a0fea9da2201077ba2fe75ba88c2d242304"
      },
      "amount_synthetic": "10000000000",
      "amount_collateral": "30000000000",
      "amount_fee": "0",
      "asset_id_synthetic": "0x4254432d3130000000000000000000",
      "asset_id_collateral": "0xa21edc9d9997b1b1956f542fe95922518a9e28ace11b7b2972a1974bf5971f",
      "position_id": "10026",
      "is_buying_synthetic": true
    }
    "#;

    let json2 = r#"
        {
      "nonce": "0",
      "public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
      "expiration_timestamp": "3608164305",
      "signature": {
        "r": "0x532a22fcdcf55ea7badfca68c1f04c2cd1eeaa1020d69c0c589aafd429fe040f",
        "s": "0xad4c23dd453a9b313c451596d3797c9fbcdeb1e2fd654c4e24fe3a56350d7900"
      },
      "amount_synthetic": "10000000000",
      "amount_collateral": "30000000000",
      "amount_fee": "0",
      "asset_id_synthetic": "0x4254432d3130000000000000000000",
      "asset_id_collateral": "0xa21edc9d9997b1b1956f542fe95922518a9e28ace11b7b2972a1974bf5971f",
      "position_id": "10027",
      "is_buying_synthetic": false
    }
    "#;

    let json_arr = vec![json1,json2];
    let pri_arr = vec!["0279df312299a1400f0438e38a46432136306c531359a5edd359ae6556adf6cc","042f82c4c360326263672ae3feefd4509201989e0660c0f625f47af81c975fc8"];

    for (i, v) in json_arr.into_iter().enumerate() {
        println!("pk{}: {}",i,public_key_from_private(&private_key_from_string(pri_arr[i]).unwrap()).to_string());
        let sig = sign_limit_order(v,pri_arr[i]).unwrap();
        println!("sig{}: {}",i, serde_json::to_string(&sig).unwrap())
    }




    #[test]
    fn test_hash_signed_oracle_price() {
        let data = r#"
        {"external_price":"28409392522000000000000","signed_asset_id":"0x425443555344434f4b580000000000005374437277","signer_key":"0x2a7cbe3ca4491e20263bab3451b6179f9268097a9af449eaab7a88093a694a0d","timestamp":"1693907824"}
        "#;

        let sig = sign_signed_oracle_price(data,"01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe").unwrap();
        let a = serde_json::to_string(&sig).unwrap();
        println!("{:#?}", a);

        let hash = hash_signed_oracle_price(data).unwrap();
        println!("{:?}", hash);

        // let x = U256::from(1);
        // let y = U256::from(2);
        // let a = hash2(&x, &y);
        // println!("{:#?}", a)
    }
}



