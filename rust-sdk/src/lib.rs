#![feature(test)]
extern crate core;
extern crate test as other_test;

use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use anyhow::{Error, Result};
pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
use franklin_crypto::{
    alt_babyjubjub::{fs::FsRepr, AltJubjubBn256, FixedGenerators},
    bellman::pairing::ff::{PrimeField, PrimeFieldRepr},
    eddsa::{PrivateKey, PublicKey, Signature as EddsaSignature},
    jubjub::JubjubEngine,
};
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
use crate::hash_type::{hash_type_to_string_with_0xprefix};
use crate::transaction::limit_order::{limit_order_hash, LimitOrderRequest};
use crate::transaction::liquidate::Liquidate;
use crate::transaction::oracle_price::{signed_oracle_price_hash, SignedOraclePrice};
use crate::transaction::transfer::{transfer_hash, Transfer};
use crate::transaction::types::HashType;
use crate::transaction::withdraw::{Withdraw, WithdrawRequest};
use crate::transaction::{limit_order, oracle_price, transfer, withdraw};
use crate::tx::convert::FeConvert;
use crate::tx::packed_public_key::{
    convert_to_pubkey, private_key_from_string, public_key_from_private, PackedPublicKey,
};
use crate::tx::packed_signature::PackedSignature;
use crate::tx::sign::TxSignature;
use crate::tx::withdraw::withdrawal_hash;
use crate::tx::{h256_to_u256, u256_to_h256};
use crate::utils::set_panic_hook;
use crate::zkw::{BabyJubjubPoint, JubjubSignature};

mod common;
mod constant;
mod convert;
mod fr;

mod models;

mod types;
mod utils;

pub mod byte_tools;
pub mod env_tools;
pub mod format;
mod hash;
pub mod java_bridge;
pub mod javascript_bridge;
pub mod serde_wrapper;
pub mod transaction;
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
    // pub static ref RESCUE_HASHER: BabyRescueHasher = BabyRescueHasher::default();
    pub static ref RESCUE_PARAMS_CONST: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

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

pub fn sign_withdraw(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let withdrawReq: WithdrawRequest = serde_json::from_str(json)?;
    let withdraw = Withdraw {
        base: withdrawReq.base,
        position_id: withdrawReq.position_id,
        amount: withdrawReq.amount,
        owner_key: withdrawReq.owner_key,
    };
    Ok(withdraw::sign_withdraw(
        withdraw,
        &withdrawReq.asset_id,
        private_key,
    )?)
}

pub fn hash_withdraw(json: &str) -> Result<String> {
    let withdrawReq: WithdrawRequest = serde_json::from_str(json)?;
    let withdraw = Withdraw {
        base: withdrawReq.base,
        position_id: withdrawReq.position_id,
        amount: withdrawReq.amount,
        owner_key: withdrawReq.owner_key,
    };
    Ok(hash_type_to_string_with_0xprefix(withdrawal_hash(
        &withdraw,
        &withdrawReq.asset_id,
    )))
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
    Ok(limit_order::sign_limit_order(
        req.liquidator_order,
        private_key,
    )?)
}

pub fn hash_liquidate(json: &str) -> Result<String> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(limit_order_hash(
        &req.liquidator_order,
    )))
}

pub fn sign_signed_oracle_price(json: &str, private_key: &str) -> Result<JubjubSignature> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    Ok(oracle_price::sign_signed_oracle_price(req, private_key)?)
}

pub fn hash_signed_oracle_price(json: &str) -> Result<String> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(signed_oracle_price_hash(
        &req,
    )))
}

pub fn private_key_to_pubkey_xy(private_key: &str) -> Result<(String, String)> {
    let pri_key = private_key_from_string(private_key)?;
    let packed_pk: PackedPublicKey = public_key_from_private(&pri_key);
    Ok(pub_key_to_xy(&packed_pk.to_string())?)
}

pub fn pub_key_to_xy(pub_key: &str) -> Result<(String, String)> {
    let pub_key = pub_key.trim_start_matches("0x").trim_start_matches("0X");
    let packed_pk =  PackedPublicKey::try_from(pub_key)?;

    let jubjub_pk: BabyJubjubPoint = packed_pk.into();
    let mut x_point = [0; 32];
    jubjub_pk.x.to_big_endian(&mut x_point);

    Ok((
        "0x".to_owned() + &pub_key.to_string(),
        "0x".to_owned() + &hex::encode(x_point.to_vec()),
    ))
}

pub fn sign(private_key: &str, msg: &str) -> Result<JubjubSignature> {
    let hash = HashType::from_str(msg)?;
    let private_key = private_key_from_string(private_key)?;
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

pub fn verify_signature(
    sig_r: &str,
    sig_s: &str,
    pub_key_x: &str,
    pub_key_y: &str,
    msg: &str,
) -> Result<bool> {
    let sig = JubjubSignature::from_str(sig_r, sig_s);
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg)?;
    let packed_pk = PackedPublicKey::try_from(pub_key_x)?;
    let jubjub_pk: BabyJubjubPoint = packed_pk.into();
    let pk = convert_to_pubkey(&jubjub_pk.x, &jubjub_pk.y)?;
    Ok(sig.verify(&pk, msg.as_le_bytes()))
}

pub fn verify_jubjub_signature(sig: JubjubSignature, pub_key: &str, msg: &str) -> Result<bool> {
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg)?;
    let packed_pk = PackedPublicKey::try_from(pub_key)?;
    let jubjub_pk: BabyJubjubPoint = packed_pk.into();
    let pk = convert_to_pubkey(&jubjub_pk.x, &jubjub_pk.y)?;
    Ok(sig.verify(&pk, msg.as_le_bytes()))
}

pub fn is_on_curve(x: &str, y: &str) -> Result<bool> {
    let x = x.trim_start_matches("0x").trim_start_matches("0X");
    let y = y.trim_start_matches("0x").trim_start_matches("0X");
    let (x1, y1) = pub_key_to_xy(x)?;
    let x1 = x1.trim_start_matches("0x").trim_start_matches("0X");
    let y1 = y1.trim_start_matches("0x").trim_start_matches("0X");
    Ok(x1 == x && y1 == y)
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
    let msg = msg.trim_start_matches("0x").trim_start_matches("0X");
    let private_key = private_key_from_string(private_key)?;
    let msg = HashType::from_str(msg)?;
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

pub fn reverse_hex(str: &str) -> anyhow::Result<String> {
    let mut ret = hex::decode(str)?;
    ret.as_mut_slice().reverse();
    Ok(hex::encode(ret))
}

#[derive(Debug, Deserialize, Serialize)]
struct Signature<'a> {
    r: &'a str,
    s: &'a str,
}

#[cfg(test)]
mod test {
    use other_test::Bencher;
    use pairing_ce::bn256::Fr;
    use pairing_ce::ff::PrimeField;


    use crate::{
        hash_limit_order, hash_liquidate, hash_signed_oracle_price, hash_transfer, hash_withdraw,
        is_on_curve, l1_sign, private_key_from_seed, private_key_to_pubkey_xy, pub_key_to_xy,
        reverse_hex, sign, sign_limit_order, sign_liquidate, sign_signed_oracle_price,
        sign_transfer, sign_withdraw, verify_jubjub_signature, verify_signature, L1Signature,
        Signature,
    };

    use crate::tx::{
        private_key_from_string, public_key_from_private, FeConvert, HashType, JubjubSignature,
    };

    const pri_key: &str = "0x01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
    const pub_key: &str = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";

    fn verify_valid_sig(sig: &JubjubSignature) {
        let json = serde_json::to_string(sig).unwrap();
        let sig: Signature = serde_json::from_str(&json).unwrap();
        assert!(sig.r.len() == 66);
        assert!(sig.s.len() == 66);
    }

    #[test]
    pub fn test_verify_signature() {
        let sigr = "0x2e39e39381ac5e962650072a8936b99716fc0b3fda124f59ef62066301fd0749";
        let sigs = "0x37fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f";
        let msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9";
        let err_msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d1a5ab082f1a520";
        let pub_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let pub_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        let ret = verify_signature(sigr,sigs, pub_x, pub_y, msg).unwrap();
        assert!(ret);
        let ret = verify_signature(sigr,sigs, pub_x, pub_y, err_msg).unwrap();
        assert!(!ret);
    }

    #[test]
    #[should_panic]
    pub fn test_verify_signature_with_err_sig() {
        let sigr = "0x2e39e39381ac5e962650072a893b99716fc0b3fda124f";
        let sigs = "0x37fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f";
        let msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9";
        let err_msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d1a5ab082f1a520";
        let pub_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let pub_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        let ret = verify_signature(sigr,sigs, pub_x, pub_y, msg).unwrap();
    }

    #[test]
    pub fn test_withdraw() {
        let json = r#"
        {
        "nonce":"1",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"1684832800",
        "position_id":"2",
        "amount":"3",
        "eth_address":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "asset_id":"0x1"
        }
        "#;
        let sig = sign_withdraw(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_withdraw(json).unwrap()).unwrap());
    }

    #[test]
    pub fn test_hash_withdraw() {
        let json = r#"
        {
        "nonce":"1",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"1684832800",
        "position_id":"2",
        "amount":"3",
        "eth_address":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "asset_id":"0x1"
        }
        "#;
        let hash = hash_withdraw(json).unwrap();
        assert!(hash.len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_withdraw_with_err_public_key() {
        let json = r#"
        {
        "nonce":"1",
        "public_key":"0x8f79f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"1684832800",
        "position_id":"2",
        "amount":"3",
        "eth_address":"0x82ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "asset_id":"0x1"
        }
        "#;
        let hash = hash_withdraw(json).unwrap();
        assert!(hash.len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_withdraw_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        let hash = hash_withdraw(json).unwrap();
        assert!(hash.len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_withdraw_with_err_public_key() {
        let json = r#"
        {
        "nonce":"1",
        "public_key":"0x92ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"1684832800",
        "position_id":"2",
        "amount":"3",
        "eth_address":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "asset_id":"0x1"
        }
        "#;
        let sig = sign_withdraw(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_withdraw(json).unwrap()).unwrap());
    }

    #[test]
    #[should_panic]
    pub fn test_withdraw_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        let sig = sign_withdraw(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_withdraw(json).unwrap()).unwrap());
    }

    #[test]
    pub fn test_sign_transfer() {
        let json = r#"
        {
        "nonce":"0",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "sender_position_id":"0",
        "receiver_public_key":"0x0000000000000000000000000000000000000000000000000000000000000000",
        "receiver_position_id":"0",
        "amount":"0",
        "asset_id":"0xa"
        }
        "#;
        let sig = sign_transfer(json, pri_key).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, pub_key, &hash).unwrap());
    }

    #[test]
    pub fn test_hash_transfer() {
        let json = r#"
        {
        "nonce":"0",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "sender_position_id":"0",
        "receiver_public_key":"0x0000000000000000000000000000000000000000000000000000000000000000",
        "receiver_position_id":"0",
        "amount":"0",
        "asset_id":"0xa"
        }
        "#;
        let hash = hash_transfer(json).unwrap();
        assert!(hash.len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_transfer_with_err_public_key() {
        let json = r#"
        {
        "nonce":"0",
        "public_key":"0x8f792ad4f9ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "sender_position_id":"0",
        "receiver_public_key":"0x8792ad4f9bad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "receiver_position_id":"0",
        "amount":"0",
        "asset_id":"0xa"
        }
        "#;
        let hash = hash_transfer(json).unwrap();
        assert!(hash.len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_transfer_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        let hash = hash_transfer(json).unwrap();
        assert!(hash.len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_sign_transfer_err_public_key() {
        let json = r#"
        {
        "nonce":"0",
        "public_key":"0x7092ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "sender_position_id":"0",
        "receiver_public_key":"0x7092ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "receiver_position_id":"0",
        "amount":"0",
        "asset_id":"0xa"
        }
        "#;
        let sig = sign_transfer(json, pri_key).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, pub_key, &hash).unwrap());
    }

    #[test]
    #[should_panic]
    pub fn test_sign_transfer_with_err_amount() {
        let json = r#"
        {
        "nonce":"0",
        "public_key":"0x7092ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "sender_position_id":"0",
        "receiver_public_key":"0x7092ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "receiver_position_id":"0",
        "amount":1,
        "asset_id":"0xa"
        }
        "#;
        let sig = sign_transfer(json, pri_key).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, pub_key, &hash).unwrap());
    }

    #[test]
    #[should_panic]
    pub fn test_sign_transfer_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        let sig = sign_transfer(json, pri_key).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, pub_key, &hash).unwrap());
    }

    #[test]
    pub fn test_sign_limit_order() {
        let json = r#"{
        "nonce":"1",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"2",
        "amount_synthetic":"3",
        "amount_collateral":"4",
        "amount_fee":"5",
        "asset_id_synthetic":"0x6",
        "asset_id_collateral":"0x7",
        "position_id":"8",
        "is_buying_synthetic":false
        }"#;
        let sig = sign_limit_order(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_limit_order(json).unwrap()).unwrap())
    }

    #[test]
    pub fn test_hash_limit_order() {
        let json = r#"{
        "nonce":"1",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"2",
        "amount_synthetic":"3",
        "amount_collateral":"4",
        "amount_fee":"5",
        "asset_id_synthetic":"0x6",
        "asset_id_collateral":"0x7",
        "position_id":"8",
        "is_buying_synthetic":false
        }"#;
        assert!(hash_limit_order(json).unwrap().len() == 66)
    }

    #[test]
    #[should_panic]
    pub fn test_hash_limit_order_with_err_public_key() {
        let json = r#"{
        "nonce":"1",
        "public_key":"0x8f7924f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"2",
        "amount_synthetic":"3",
        "amount_collateral":"4",
        "amount_fee":"5",
        "asset_id_synthetic":"0x6",
        "asset_id_collateral":"0x7",
        "position_id":"8",
        "is_buying_synthetic":false
        }"#;
        assert!(hash_limit_order(json).unwrap().len() == 66)
    }

    #[test]
    #[should_panic]
    pub fn test_hash_limit_order_with_empty_json() {
        let json = r#"{
        }"#;
        assert!(hash_limit_order(json).unwrap().len() == 66)
    }

    #[test]
    #[should_panic]
    pub fn test_sign_limit_order_with_err_public_key() {
        let json = r#"{
        "nonce":"1",
        "public_key":"0x82ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"2",
        "amount_synthetic":"3",
        "amount_collateral":"4",
        "amount_fee":"5",
        "asset_id_synthetic":"0x6",
        "asset_id_collateral":"0x7",
        "position_id":"8",
        "is_buying_synthetic":false
        }"#;
        let sig = sign_limit_order(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_limit_order(json).unwrap()).unwrap())
    }

    #[test]
    #[should_panic]
    pub fn test_sign_limit_order_with_empty_json() {
        let json = r#"{

        }"#;
        let sig = sign_limit_order(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_limit_order(json).unwrap()).unwrap())
    }

    #[test]
    pub fn test_sign_liquidate() {
        let json = r#"
    {
    "liquidator_order":{
        "nonce":"0",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "amount_synthetic":"1",
        "amount_collateral":"2",
        "amount_fee":"3",
        "asset_id_synthetic":"0x4",
        "asset_id_collateral":"0x5",
        "position_id":"6",
        "is_buying_synthetic":false
    },
    "liquidated_position_id":"7",
    "actual_collateral":"8",
    "actual_synthetic":"9",
    "actual_liquidator_fee":"10"
}
        "#;

        let sig = sign_liquidate(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_liquidate(json).unwrap()).unwrap());
    }

    #[test]
    pub fn test_hash_liquidate() {
        let json = r#"
    {
    "liquidator_order":{
        "nonce":"0",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "amount_synthetic":"1",
        "amount_collateral":"2",
        "amount_fee":"3",
        "asset_id_synthetic":"0x4",
        "asset_id_collateral":"0x5",
        "position_id":"6",
        "is_buying_synthetic":false
    },
    "liquidated_position_id":"7",
    "actual_collateral":"8",
    "actual_synthetic":"9",
    "actual_liquidator_fee":"10"
}
        "#;
        assert!(hash_liquidate(json).unwrap().len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_liquidate_with_err_public_key() {
        let json = r#"
    {
    "liquidator_order":{
        "nonce":"0",
        "public_key":"0x8f792ad49b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "expiration_timestamp":"0",
        "amount_synthetic":"1",
        "amount_collateral":"2",
        "amount_fee":"3",
        "asset_id_synthetic":"0x4",
        "asset_id_collateral":"0x5",
        "position_id":"6",
        "is_buying_synthetic":false
    },
    "liquidated_position_id":"7",
    "actual_collateral":"8",
    "actual_synthetic":"9",
    "actual_liquidator_fee":"10"
}
        "#;
        assert!(hash_liquidate(json).unwrap().len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_liquidate_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        assert!(hash_liquidate(json).unwrap().len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_sign_liquidate_with_err_public_key() {
        let json = r#"
    {
    "liquidator_order":{
        "nonce":"0",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e586aa",
        "expiration_timestamp":"0",
        "amount_synthetic":"1",
        "amount_collateral":"2",
        "amount_fee":"3",
        "asset_id_synthetic":"0x4",
        "asset_id_collateral":"0x5",
        "position_id":"6",
        "is_buying_synthetic":false
    },
    "liquidated_position_id":"7",
    "actual_collateral":"8",
    "actual_synthetic":"9",
    "actual_liquidator_fee":"10"
    }
        "#;

        let sig = sign_liquidate(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_liquidate(json).unwrap()).unwrap());
    }

    #[test]
    #[should_panic]
    pub fn test_sign_liquidate_with_empty_json() {
        let json = r#"
    {
    "liquidator_order":{
        "nonce":"0",
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e586aa",
        "expiration_timestamp":"0",
        "amount_synthetic":"1",
        "amount_collateral":"2",
        "amount_fee":"3",
        "asset_id_synthetic":"0x4",
        "asset_id_collateral":"0x5",
        "position_id":"6",
        "is_buying_synthetic":false
    },
    "liquidated_position_id":"7",
    "actual_collateral":"8",
    "actual_synthetic":"9",
    "actual_liquidator_fee":"10"
    }
        "#;

        let sig = sign_liquidate(json, pri_key).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, pub_key, &hash_liquidate(json).unwrap()).unwrap());
    }

    #[test]
    pub fn test_sign_oracle_price() {
        let json1 = r#"
        {
        "signer_key": "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a",
        "external_price": "28409392522000000000000",
        "timestamp": "1693907824",
        "signed_asset_id": "0x425443555344434f4b580000000000005374437277"
        }
        "#;
        let pri1 = "01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
        let sig1 = sign_signed_oracle_price(json1, pri1).unwrap();
        let hash1 = hash_signed_oracle_price(json1).unwrap();
        let json2 = r#"
        {
        "signer_key": "0x8af4f453400cf97cd47914af9179da6586ea06417ac4dec417f9f2b795719355",
        "external_price": "6652695000000000000",
        "timestamp": "1693971434",
        "signed_asset_id": "0x534f4c555344434f4b580000000000005374437277"
        }
        "#;
        let pri2 = "0376204fa0b554ee3d8a03c6ccdb73f7b98d1965fbeaa3a9f88723669a23893f";
        let sig2 = sign_signed_oracle_price(json2, pri2).unwrap();
        println!("sig2: {}", serde_json::to_string(&sig2).unwrap());

        let json3 = r#"
        {
        "signer_key": "0x15d144b7facdffd112bc06640c3bd4e5f36ad077ca9f9b97ad3f8f85906236a4",
        "external_price": "1854072360000000000000",
        "timestamp": "1693971569",
        "signed_asset_id": "0x455448555344434f4b580000000000005374437277"
        }
        "#;
        let pri3 = "060a45bcd72c9e3c82bc1c57f63ad15b25f56bb13ce01d15fd4ab3f8f2de35bb";
        let sig3 = sign_signed_oracle_price(json3, pri3).unwrap();

        let pri_arr = vec![pri1, pri2, pri3];
        for x in pri_arr {
            let pri = private_key_from_string(x).unwrap();
            let pk = public_key_from_private(&pri);
            println!("{}", pk.to_string())
        }

        let json4 = r#"
        {"external_price":"6462618000000000000","signed_asset_id":"0x534f4c555344434f4b580000000000005374437277","signer_key":"0x8af4f453400cf97cd47914af9179da6586ea06417ac4dec417f9f2b795719355","timestamp":"1694150131"}
        "#;
        let sig4 = sign_signed_oracle_price(json4, pri2).unwrap();
        verify_valid_sig(&sig1);
        verify_valid_sig(&sig2);
        verify_valid_sig(&sig3);
        verify_valid_sig(&sig4);
    }

    #[test]
    pub fn test_hash_oracle_price() {
        let json = r#"
        {
        "signer_key": "0x15d144b7facdffd112bc06640c3bd4e5f36ad077ca9f9b97ad3f8f85906236a4",
        "external_price": "1854072360000000000000",
        "timestamp": "1693971569",
        "signed_asset_id": "0x455448555344434f4b580000000000005374437277"
        }
        "#;
        assert!(hash_signed_oracle_price(json).unwrap().len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_oracle_price_with_err_signer_key() {
        let json = r#"
        {
        "signer_key": "0x1544b7facdffd112bc06640c3bd4e5f36ad077ca9f9b97ad3f8f85906236a4",
        "external_price": "1854072360000000000000",
        "timestamp": "1693971569",
        "signed_asset_id": "0x455448555344434f4b580000000000005374437277"
        }
        "#;
        assert!(hash_signed_oracle_price(json).unwrap().len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_hash_oracle_price_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        assert!(hash_signed_oracle_price(json).unwrap().len() == 66);
    }

    #[test]
    #[should_panic]
    pub fn test_sign_oracle_price_with_err_signer_key() {
        let json1 = r#"
        {
        "signer_key": "0xa09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a",
        "external_price": "28409392522000000000000",
        "timestamp": "1693907824",
        "signed_asset_id": "0x425443555344434f4b580000000000005374437277"
        }
        "#;
        let pri1 = "01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
        let sig1 = sign_signed_oracle_price(json1, pri1).unwrap();
        let hash1 = hash_signed_oracle_price(json1).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_sign_oracle_price_with_empty_json() {
        let json = r#"
        {

        }
        "#;
        let sig = sign_signed_oracle_price(json, pri_key).unwrap();
        let hash = hash_signed_oracle_price(json).unwrap();
    }

    #[bench]
    fn bench_verify_transfer(b: &mut Bencher) {
        let json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        let sig_r = "0x094a47cb182c7eb24e3c34a473def9d356bb30161179e4bbaeaa48c6d18844f8";
        let sig_s = "0x05534d29f2f1d3ba474f7cec4f9f545924924e5f4261577d09ed9a85df252d5d";
        let pub_key_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let pub_key_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";

        b.iter(|| {
            let hash = hash_transfer(json).unwrap();
            assert!(verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, &hash).unwrap());
        })
    }

    #[bench]
    fn bench_sign_transfer(b: &mut Bencher) {
        let json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        b.iter(|| {
            assert!(sign_transfer(json, pri_key).is_ok());
        })
    }

    #[test]
    fn test_private_key_to_pubkey_xy() {
        let (x, y) = private_key_to_pubkey_xy(pri_key).unwrap();
        assert!(x.len() == 66);
        assert!(y.len() == 66);
    }

    #[test]
    fn test_private_key_from_seed() {
        let seed = "hello world good life 996 very nice";
        let priKey = private_key_from_seed(seed.as_bytes()).unwrap();
        assert!(priKey.len() == 66);
    }

    #[test]
    #[should_panic(expected = "seed is too short")]
    fn test_private_key_from_empty_seed() {
        let seed = "";
        let priKey = private_key_from_seed(seed.as_bytes()).unwrap();
        assert!(priKey.len() == 66);
    }

    #[test]
    fn test_is_on_curve() {
        let x = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
        let y = "0x0a3b966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";
        assert!(is_on_curve(x, y).unwrap());

        let pri = "0x0376204fa0b554ee3d8a03c6ccdb73f7b98d1965fbeaa3a9f88723669a23893f";
        let (x,y) = private_key_to_pubkey_xy(pri).unwrap();
        assert!(is_on_curve(&x,&y).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_is_on_curve_with_err_x() {
        let x = "0x0d93a09887aaba49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
        let y = "0x0a966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";
        let ret = is_on_curve(&x, &y).unwrap();
        assert!(ret == false);
    }



    #[test]
    fn test_pub_key_to_xy() {
        let pk = "8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let (x, y) = pub_key_to_xy(pk).unwrap();
        assert!(x.len() == 66);
        assert!(y.len() == 66);
    }

    #[test]
    #[should_panic]
    fn test_pub_key_to_xy_with_err_pub_key() {
        let pk  = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7";
        let (x, y) = pub_key_to_xy(pk).unwrap();
        assert!(x.len() == 66);
        assert!(y.len() == 66);
    }

    #[test]
    fn test_sign() {
        let pri = "0x028bfbb9eafdacf8d76c1c35c1ed25979480d3e46d8bb391778f0fc9d40aaf70";
        let msg = "0x01b9c04067307822ea5909e9c86163128a76afbc90de47c77705cd4a4f33533f";
        let sig = sign(pri, msg).unwrap();
        let json = serde_json::to_string(&sig).unwrap();
        let sig_json: Signature = serde_json::from_str(&json).unwrap();
        assert!(sig_json.r.len() == 66);
        assert!(sig_json.s.len() == 66);
        let pk = public_key_from_private(&private_key_from_string(pri).unwrap());
        assert!(verify_jubjub_signature(sig, pk.to_string().as_str(), msg).unwrap())
    }

    #[test]
    fn test_reverse_hex() {
        let num = "12ba9000";
        println!("{:?}", reverse_hex(num).unwrap());
    }

    #[test]
    fn test_sign_trade_order() {
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

        let json_arr = vec![json1, json2];
        let pri_arr = vec![
            "0279df312299a1400f0438e38a46432136306c531359a5edd359ae6556adf6cc",
            "042f82c4c360326263672ae3feefd4509201989e0660c0f625f47af81c975fc8",
        ];

        for (i, v) in json_arr.into_iter().enumerate() {
            let pk =
                public_key_from_private(&private_key_from_string(pri_arr[i]).unwrap()).to_string();
            let sig = sign_limit_order(v, pri_arr[i]).unwrap();
            let hash = hash_limit_order(v).unwrap();
            verify_valid_sig(&sig);
            assert!(verify_jubjub_signature(sig, pk.as_str(), hash.as_str()).unwrap());
        }
    }

    #[test]
    fn test_hex_encode() {
        let a = [1u8; 32];
        let s = hex::encode(a.to_vec());
        assert!(s.len() == 64);

        let fr = Fr::from_hex("0x1111").unwrap();
        assert!(fr.to_hex().len() == 64);
    }

    #[test]
    pub fn test_l1_sign() {
        let msg = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea";
        let priv_key = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
        let s = l1_sign(msg, priv_key).unwrap();
        let expected = L1Signature {
            x: "0x062b74e4bde7c5655093bcfd717b2be2757fc7c85f2b5fdc0f43820df2ce510a".to_string(),
            y: "0x124c1159c6164b8f80348f23a39ff79af229ecb2f00e806e60798601607c4595".to_string(),
            s: "0x04f89ebc83800e89f19e3501562793e2d9097b921ee0759b5f37017b993238c4".to_string(),
            pk_x: "0x96c4d93a49c8159e27542601ba19fdfce52b3e9b43dafaefe9aa9cd32efded86".to_string(),
            pk_y: "0x0cc8a68b8dba85bd5418e308b34439ddffca3a0f6589a32f02adf60da6e73f55".to_string(),
        };
        assert!(s == expected)
    }
}
