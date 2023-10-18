#[cfg(feature = "js")]
pub mod javascript_bridge {
    use crate::utils::set_panic_hook;
    use crate::{
        hash_limit_order, hash_liquidate, hash_signed_oracle_price, hash_transfer, hash_withdraw,
        is_on_curve, l1_sign, private_key_from_seed, private_key_to_pubkey_xy, pub_key_to_xy, sign,
        sign_limit_order, sign_liquidate, sign_signed_oracle_price, sign_transfer, sign_withdraw,
        verify_signature, JUBJUB_PARAMS, RESCUE_PARAMS,
    };
    use serde::Serialize;
    use wasm_bindgen::prelude::wasm_bindgen;
    use wasm_bindgen::JsValue;

    #[wasm_bindgen(start)]
    /// This method initializes params for current thread, otherwise they will be initialized when signing
    /// first message.
    pub fn zkdex_init() {
        JUBJUB_PARAMS.with(|_| {});
        RESCUE_PARAMS.with(|_| {});
        set_panic_hook();
    }

    /// sign_transfer, sign a transfer transaction.
    /// @param {string} json  json of transfer transaction.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @returns {string} json signature of transfer transaction.
    #[wasm_bindgen(js_name = sign_transfer, skip_jsdoc)]
    pub fn js_sign_transfer(json: &str, private_key: &str) -> Result<String, JsValue> {
        match sign_transfer(json, private_key) {
            Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// hash_transfer, hash a transfer transaction.
    /// @param {string} json  json of transfer transaction.
    /// @returns {string} string hash of transfer transaction with 0x prefix.
    #[wasm_bindgen(js_name = hash_transfer, skip_jsdoc)]
    pub fn js_hash_transfer(json: &str) -> Result<String, JsValue> {
        match hash_transfer(json) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// sign_withdraw, sign a withdraw transaction.
    /// @param {string} json  json of withdraw transaction.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @returns {string} json signature of withdraw transaction.
    #[wasm_bindgen(js_name = sign_withdraw, skip_jsdoc)]
    pub fn js_sign_withdraw(json: &str, private_key: &str) -> Result<String, JsValue> {
        let withdraw = sign_withdraw(json, private_key);
        match withdraw {
            Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// hash_withdraw, hash a withdraw transaction.
    /// @param {string} json  json of withdraw transaction.
    /// @returns {string} string hash of withdraw transaction with 0x prefix.
    #[wasm_bindgen(js_name = hash_withdraw, skip_jsdoc)]
    pub fn js_hash_withdraw(json: &str) -> Result<String, JsValue> {
        match hash_withdraw(json) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// sign_limit_order, sign a limit order transaction.
    /// @param {string} json  json of limit order transaction.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @returns {string} json signature of limit order transaction.
    #[wasm_bindgen(js_name = sign_limit_order, skip_jsdoc)]
    pub fn js_sign_limit_order(json: &str, private_key: &str) -> Result<String, JsValue> {
        match sign_limit_order(json, private_key) {
            Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// hash_limit_order, sign a limit order transaction.
    /// @param {string} json  json of limit order transaction.
    /// @returns {string} string hash of limit order transaction with 0x prefix.
    #[wasm_bindgen(js_name = hash_limit_order, skip_jsdoc)]
    pub fn js_hash_limit_order(json: &str) -> Result<String, JsValue> {
        match hash_limit_order(json) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// sign_liquidate, sign a liquidate transaction.
    /// @param {string} json  json of liquidate transaction.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @returns {string} json signature of liquidate transaction.
    #[wasm_bindgen(js_name = sign_liquidate, skip_jsdoc)]
    pub fn js_sign_liquidate(json: &str, private_key: &str) -> Result<String, JsValue> {
        match sign_liquidate(json, private_key) {
            Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// hash_liquidate, hash a liquidate transaction.
    /// @param {string} json  json of liquidate transaction.
    /// @returns {string} string hash of liquidate transaction with 0x prefix.
    #[wasm_bindgen(js_name = hash_liquidate, skip_jsdoc)]
    pub fn js_hash_liquidate(json: &str) -> Result<String, JsValue> {
        match hash_liquidate(json) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// sign_signed_oracle_price, sign a signed oracle price transaction.
    /// @param {string} json  json of liquidate transaction.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @returns {string} json signature of liquidate transaction.
    #[wasm_bindgen(js_name = sign_signed_oracle_price, skip_jsdoc)]
    pub fn js_sign_signed_oracle_price(json: &str, private_key: &str) -> Result<String, JsValue> {
        match sign_signed_oracle_price(json, private_key) {
            Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// hash_signed_oracle_price, hash a signed oracle price transaction.
    /// @param {string} json  json of signed oracle transaction.
    /// @returns {string} string hash of signed oracle transaction with 0x prefix.
    #[wasm_bindgen(js_name = hash_signed_oracle_price, skip_jsdoc)]
    pub fn js_hash_signed_oracle_price(json: &str) -> Result<String, JsValue> {
        match hash_signed_oracle_price(json) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// verify_signature, verify a signature.
    /// @param {string} sig_r  r of signature.
    /// @param {string} sig_s  s of signature.
    /// @param {string} pub_key_x  x of public key.
    /// @param {string} pub_key_y  y of public key.
    /// @param {string} msg  msg hex with 0x prefix.
    /// @returns {bool} whether the signature is valid.
    #[wasm_bindgen(js_name = verify_signature, skip_jsdoc)]
    pub fn js_verify_signature(
        sig_r: &str,
        sig_s: &str,
        pub_key_x: &str,
        pub_key_y: &str,
        msg: &str,
    ) -> Result<bool, JsValue> {
        match verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// l1 sign, sign a msg on l1 when signing a eth address.
    /// @param {string} msg  msg coding in hex with 0x prefix.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @param {string} string of signature.
    #[wasm_bindgen(js_name = l1_sign, skip_jsdoc)]
    pub fn js_l1_sign(msg: &str, private_key: &str) -> Result<String, JsValue> {
        match l1_sign(msg, private_key) {
            Ok(ret) => {
                let r = serde_json::to_string(&ret).unwrap();
                Ok(r)
            }

            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// is_on_curve, check the (x,y) is on curve.
    /// @param {string} pub_key_x  x of public key with 0x prefix.
    /// @param {string} pub_key_y  y of public key with 0x prefix.
    /// @returns {bool} whether the (x,y) is on curve.
    #[wasm_bindgen(js_name = is_on_curve, skip_jsdoc)]
    pub fn js_is_on_curve(pub_key_x: &str, pub_key_y: &str) -> Result<bool, JsValue> {
        match is_on_curve(pub_key_x, pub_key_y) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// sign, sign a msg on l2, is a generic signature methods.
    /// @param {string} msg  msg coding in hex with 0x prefix.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @returns {string} json string of the signature.
    #[wasm_bindgen(js_name = sign, skip_jsdoc)]
    pub fn js_sign(private_key: &str, msg: &str) -> Result<String, JsValue> {
        match sign(private_key, msg) {
            Ok(ret) => Ok(serde_json::to_string(&ret).unwrap()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// private_key_from_seed, derive a private key from a random seed, the seed could be anything.
    /// @param {string} seed  anything string.
    /// @returns {string} string of private coding in hex with 0x prefix.
    #[wasm_bindgen(js_name = private_key_from_seed, skip_jsdoc)]
    pub fn js_private_key_from_seed(seed: &str) -> Result<String, JsValue> {
        match private_key_from_seed(seed.as_bytes()) {
            Ok(ret) => Ok(ret),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// private_key_to_pubkey_xy, derive a public with xy from private key.
    /// @param {string} private_key private key hex with 0x prefix.
    /// @returns {string} json string of public key xy.
    #[wasm_bindgen(js_name = private_key_to_pubkey_xy, skip_jsdoc)]
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
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    /// public_key_to_xy, convert public key to xy.
    /// @param {string} pub_key public key hex with 0x prefix.
    /// @returns {string} json string of public key xy.
    #[wasm_bindgen(js_name = public_key_to_xy, skip_jsdoc)]
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
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }
}
