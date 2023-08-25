use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{hash_limit_order, hash_liquidate, hash_signed_oracle_price, hash_transfer, hash_withdraw, l1_sign, sign_limit_order, sign_liquidate, sign_signed_oracle_price, sign_transfer, sign_withdraw, verify_signature};

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

#[wasm_bindgen(js_name = sign_withdraw)]
pub fn js_sign_withdraw(
    json: &str,
    asset_id_collateral: &str,
    private_key: &str,
) -> Result<String, JsValue> {
    let withdraw = sign_withdraw(json, &asset_id_collateral, private_key);
    match withdraw {
        Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

#[wasm_bindgen(js_name = hash_withdraw)]
pub fn js_hash_withdraw(json: &str, asset_id_collateral: &str) -> Result<String, JsValue> {
    match hash_withdraw(json, asset_id_collateral) {
        Ok(ret) => {
            Ok(ret)
        }
        Err(e) => {
            Err(JsValue::from_str(e.to_string().as_str()))
        }
    }
}

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

#[wasm_bindgen(js_name = hash_limit_order)]
pub fn js_hash_limit_order(json: &str) -> Result<String, JsValue> {
    match hash_limit_order(json) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

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

#[wasm_bindgen(js_name = hash_liquidate)]
pub fn js_hash_liquidate(json: &str) -> Result<String, JsValue> {
    match hash_liquidate(json) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

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

#[wasm_bindgen(js_name = hash_signed_oracle_price)]
pub fn js_hash_signed_oracle_price(json: &str) -> Result<String, JsValue> {
    match hash_signed_oracle_price(json) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

#[wasm_bindgen(js_name = verify_signature)]
pub fn js_verify_signature(sig_r: &str, sig_s: &str, pub_key_x: &str, pub_key_y:&str, msg: &str) -> Result<bool,JsValue> {
    match verify_signature(sig_r,sig_s,pub_key_x,pub_key_y,msg) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}

#[wasm_bindgen(js_name = l1_sign)]
pub fn js_l1_sign(msg: &str, private_key: &str)-> Result<String,JsValue> {

    match  l1_sign(msg,private_key) {
        Ok(ret) => {
            let r = serde_json::to_string(&ret).unwrap();
            Ok(r)
        }

        Err(e) => Err(JsValue::from_str(e.to_string().as_str()))
    }
}