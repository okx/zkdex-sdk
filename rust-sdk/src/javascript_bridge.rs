use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{hash_limit_order, hash_liquidate, hash_signed_oracle_price, hash_transfer, hash_withdraw, is_on_curve, l1_sign, private_key_from_seed, private_key_to_pubkey_xy, pub_key_to_xy, sign, sign_limit_order, sign_liquidate, sign_signed_oracle_price, sign_transfer, sign_withdraw, verify_signature};

/// sign a transfer transaction
#[wasm_bindgen(js_name = sign_transfer)]
pub fn js_sign_transfer(json: &str, private_key: &str) -> Result<String, JsValue> {
    match sign_transfer(json, private_key) {
        Ok(ret) => {
            Ok(serde_json::to_string(&ret).unwrap())
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

/// hash a transfer transaction
#[wasm_bindgen(js_name = hash_transfer)]
pub fn js_hash_transfer(json: &str) -> Result<String, JsValue> {
    match hash_transfer(json) {
        Ok(ret) => {
            Ok(ret)
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

/// sign a withdraw transaction
#[wasm_bindgen(js_name = sign_withdraw)]
pub fn js_sign_withdraw(
    json: &str,
    private_key: &str,
) -> Result<String, JsValue> {
    let withdraw = sign_withdraw(json, private_key);
    match withdraw {
        Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// hash a withdraw transaction
#[wasm_bindgen(js_name = hash_withdraw)]
pub fn js_hash_withdraw(json: &str) -> Result<String, JsValue> {
    match hash_withdraw(json) {
        Ok(ret) => {
            Ok(ret)
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

/// sign a limit order transaction
#[wasm_bindgen(js_name = sign_limit_order)]
pub fn js_sign_limit_order(json: &str, private_key: &str) -> Result<String, JsValue> {
    match sign_limit_order(json, private_key) {
        Ok(ret) => {
            Ok(serde_json::to_string(&ret).unwrap())
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

/// sign a limit order transaction
#[wasm_bindgen(js_name = hash_limit_order)]
pub fn js_hash_limit_order(json: &str) -> Result<String, JsValue> {
    match hash_limit_order(json) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// sign a liquidate transaction
#[wasm_bindgen(js_name = sign_liquidate)]
pub fn js_sign_liquidate(json: &str, private_key: &str) -> Result<String, JsValue> {
    match sign_liquidate(json, private_key) {
        Ok(ret) => {
            Ok(serde_json::to_string(&ret).unwrap())
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

/// hash a liquidate transaction
#[wasm_bindgen(js_name = hash_liquidate)]
pub fn js_hash_liquidate(json: &str) -> Result<String, JsValue> {
    match hash_liquidate(json) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// sign a signed oracle price transaction
#[wasm_bindgen(js_name = sign_signed_oracle_price)]
pub fn js_sign_signed_oracle_price(
    json: &str,
    private_key: &str,
) -> Result<String, JsValue> {
    match sign_signed_oracle_price(json, private_key) {
        Ok(ret) => {
            Ok(serde_json::to_string(&ret).unwrap())
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

/// hash a signed oracle price transaction
#[wasm_bindgen(js_name = hash_signed_oracle_price)]
pub fn js_hash_signed_oracle_price(json: &str) -> Result<String, JsValue> {
    match hash_signed_oracle_price(json) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// verify a signature
#[wasm_bindgen(js_name = verify_signature)]
pub fn js_verify_signature(sig_r: &str, sig_s: &str, pub_key_x: &str, pub_key_y: &str, msg: &str) -> Result<bool, JsValue> {
    match verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// l1 sign
/// sign a msg on l1 when signing a eth address
#[wasm_bindgen(js_name = l1_sign)]
pub fn js_l1_sign(msg: &str, private_key: &str) -> Result<String, JsValue> {
    match l1_sign(msg, private_key) {
        Ok(ret) => {
            let r = serde_json::to_string(&ret).unwrap();
            Ok(r)
        }

        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// check the (x,y) is on curve
#[wasm_bindgen(js_name = is_on_curve)]
pub fn js_is_on_curve(pub_key_x: &str, pub_key_y: &str) -> Result<bool, JsValue> {
    match is_on_curve(pub_key_x, pub_key_y) {
        Ok(ret) => {
            Ok(ret)
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

/// sign a msg on l2
#[wasm_bindgen(js_name = sign)]
pub fn js_sign(pri_key: &str, msg: &str) -> Result<String, JsValue> {
    match sign(pri_key, msg) {
        Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// derive a private key from a random seed, the seed could be anything
#[wasm_bindgen(js_name = private_key_from_seed)]
pub fn js_private_key_from_seed(seed: &str) -> Result<String, JsValue> {
    match private_key_from_seed(seed.as_bytes()) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// derive a public with xy from private key
#[wasm_bindgen(js_name = private_key_to_pubkey_xy)]
pub fn js_private_to_public_key_xy(pri_key: &str) -> Result<String, JsValue> {
    match private_key_to_pubkey_xy(pri_key) {
        Ok(ret) => {
            #[derive(Serialize)]
            struct XY {
                x: String,
                y: String,
            }
            Ok(serde_json::to_string(&XY { x: ret.0, y: ret.1 }).unwrap())
        }
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

/// convert public key to xy
#[wasm_bindgen(js_name = public_key_to_xy)]
pub fn js_public_key_to_xy(pub_key: &str) -> Result<String, JsValue> {
    match pub_key_to_xy(pub_key) {
        Ok(ret) => {
            #[derive(Serialize)]
            struct XY {
                x: String,
                y: String,
            }
            Ok(serde_json::to_string(&XY { x: ret.0, y: ret.1 }).unwrap())
        }
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}