mod transactions;
mod types;

use transactions::transfer::{Transfer};
use crate::tx::{JubjubSignature, private_key_from_string};
use crate::unified::transactions::Withdrawal;

pub fn unified_sign_transfer(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Transfer = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key)?;
    Ok(signature.into())
}

pub fn unified_sign_withdrawal(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Withdrawal = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key)?;
    Ok(signature.into())
}