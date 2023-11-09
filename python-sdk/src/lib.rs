mod model;

use std::panic;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use crate::model::PublicKey;

/// A Python module implemented in Rust.
#[pymodule]
fn zkdex_python_sdk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(verify_signature, m)?)?;
    m.add_function(wrap_pyfunction!(sign_withdraw, m)?)?;
    m.add_function(wrap_pyfunction!(sign_transfer, m)?)?;
    m.add_function(wrap_pyfunction!(sign_limit_order, m)?)?;
    m.add_function(wrap_pyfunction!(sign_liquidate, m)?)?;
    m.add_function(wrap_pyfunction!(sign_signed_oracle_price, m)?)?;
    m.add_function(wrap_pyfunction!(hash_withdraw, m)?)?;
    m.add_function(wrap_pyfunction!(hash_transfer, m)?)?;
    m.add_function(wrap_pyfunction!(hash_limit_order, m)?)?;
    m.add_function(wrap_pyfunction!(hash_liquidate, m)?)?;
    m.add_function(wrap_pyfunction!(hash_signed_oracle_price, m)?)?;
    m.add_function(wrap_pyfunction!(sign, m)?)?;
    m.add_function(wrap_pyfunction!(eth_sign, m)?)?;
    m.add_function(wrap_pyfunction!(private_key_from_seed, m)?)?;
    m.add_function(wrap_pyfunction!(is_on_curve, m)?)?;
    m.add_function(wrap_pyfunction!(public_key_to_xy, m)?)?;
    m.add_function(wrap_pyfunction!(private_key_to_public_key_xy, m)?)?;
    Ok(())
}

#[pyfunction]
fn verify_signature(
    sig_r: String,
    sig_s: String,
    pub_key_x: String,
    pub_key_y: String,
    msg: String,
) -> PyResult<bool> {
    match panic::catch_unwind(|| {
      verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg)
            .expect("Couldn't get verify_signature result")
    }) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}


#[pyfunction]
fn sign_withdraw(json: String,pri_key: String) -> PyResult<String> {
    match panic::catch_unwind(|| {
        let sig = zkdex_sdk::sign_withdraw(&json, &pri_key).expect("Couldn get jubjubSignature");
        let json = serde_json::to_string(&sig).expect("Couldn't serialize signature");
        json
    }){
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn sign_transfer(
    json: String,
    pri_key: String,
) -> PyResult<String>{
    match panic::catch_unwind(|| {
        let sig = zkdex_sdk::sign_transfer(&json, &pri_key).expect("Couldn get jubjubSignature");
        let json = serde_json::to_string(&sig).expect("Couldn't serialize signature");
        json
    }) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
 fn sign_limit_order(
    json: String,
    pri_key: String,
) -> PyResult<String> {
    match panic::catch_unwind(|| {
        let sig = zkdex_sdk::sign_limit_order(&json, &pri_key).expect("Couldn get jubjubSignature");
        let json = serde_json::to_string(&sig).expect("Couldn't serialize signature");
        json
    }){
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn sign_liquidate(
    json: String,
    pri_key: String,
) -> PyResult<String> {
    match panic::catch_unwind(|| {
        let sig = zkdex_sdk::sign_liquidate(&json, &pri_key).expect("Couldn get jubjubSignature");
        let json = serde_json::to_string(&sig).expect("Couldn't serialize signature");
        json
    }) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn sign_signed_oracle_price(
    json: String,
    pri_key: String,
) -> PyResult<String>{
    match panic::catch_unwind(|| {
        let sig = zkdex_sdk::sign_signed_oracle_price(&json, &pri_key).expect("Couldn get jubjubSignature");
        let json = serde_json::to_string(&sig).expect("Couldn't serialize signature");
        json
    }){
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn hash_withdraw(
    json: String,
) -> PyResult<String> {
    match panic::catch_unwind(|| {
        zkdex_sdk::hash_withdraw(&json).expect("Couldn't get hash")
    }) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn hash_transfer(json: String) -> PyResult<String> {
     match panic::catch_unwind(|| {
        zkdex_sdk::hash_transfer(&json).expect("Couldn't get hash")
    }) {
         Ok(ret) => Ok(ret),
         Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
     }
}

#[pyfunction]
fn hash_limit_order(json: String) -> PyResult<String> {
    match panic::catch_unwind(|| {
        zkdex_sdk::hash_limit_order(&json).expect("Couldn't get hash")
    }){
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn hash_liquidate(json: String) -> PyResult<String> {
    match panic::catch_unwind(|| {
        zkdex_sdk::hash_liquidate(&json).expect("Couldn't get hash")
    }){
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn hash_signed_oracle_price(
    json: String,
) -> PyResult<String> {
    match panic::catch_unwind(|| {
        zkdex_sdk::hash_signed_oracle_price(&json).expect("Couldn't get hash")
    }) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn sign(private_key: String,msg: String) -> PyResult<String> {
     match panic::catch_unwind(|| {
        let sig = zkdex_sdk::sign(&private_key, &msg).expect("Couldn't sign msg");
        let json = serde_json::to_string(&sig).expect("Couldn't serialize signature");
        json
    }){
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn eth_sign(
    private_key: String,
    msg: String,
) -> PyResult<String> {
     match panic::catch_unwind(|| {
        let sig = zkdex_sdk::l1_sign(&msg, &private_key).expect("Couldn't sign msg");
        let json = serde_json::to_string(&sig).expect("Couldn't serialize signature");
         json
    }) {
         Ok(ret) => Ok(ret),
         Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
     }
}

#[pyfunction]
fn private_key_from_seed(
    seed: String,
) -> PyResult<String> {

     match panic::catch_unwind(|| {
        zkdex_sdk::private_key_from_seed(seed.as_bytes()).expect("Couldn't derive private key from seed")
    }) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
    }
}

#[pyfunction]
fn is_on_curve(
    x: String,
    y: String,
) -> PyResult<bool> {
     match panic::catch_unwind(|| {
        zkdex_sdk::is_on_curve(&x, &y).expect("Couldn't get verify xy is on curve")
     }) {
         Ok(ret) => Ok(ret),
         Err(e) => Err(PyValueError::new_err(format!("{:?}",e)))
     }
}

#[pyfunction]
fn public_key_to_xy(public_key: String) -> PyResult<String> {
    match panic::catch_unwind(|| {
        zkdex_sdk::pub_key_to_xy(&public_key).expect("Couldn't convert public key to xy")
    }) {
        Ok(ret) => {

            let pk = PublicKey::new(ret.0, ret.1);
            let pk_json = serde_json::to_string(&pk).expect("Couldn't serialize public key");
            Ok(pk_json)
        }
        Err(e) => {
            Err(PyValueError::new_err(format!("{:?}",e)))
        }
    }
}

#[pyfunction]
fn private_key_to_public_key_xy(private_key: String) -> PyResult<String> {
    match panic::catch_unwind(|| {
        zkdex_sdk::private_key_to_pubkey_xy(&private_key).expect("Couldn't convert private key to public key xy")
    }) {
        Ok(ret) => {

            let pk = PublicKey::new(ret.0, ret.1);
            let pk_json = serde_json::to_string(&pk).expect("Couldn't serialize public key");
            Ok(pk_json)
        }
        Err(e) => {
            Err(PyValueError::new_err(format!("{:?}",e)))
        }
    }
}