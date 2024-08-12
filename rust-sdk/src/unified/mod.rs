use crate::crypto::packed_public_key::private_key_from_string;
use crate::hash_type::hash_type_to_string_with_0xprefix;
use crate::types::HashType;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::transactions::{
    Liquidate, PerpetualTrade, SignedOraclePrice, SpotTrade, Transfer, Withdrawal,
};
use crate::zkw::JubjubSignature;

mod transactions;
mod types;

/// Sign a withdrawal transaction
/// json: the withdrawal transaction in json format
/// private_key: the private key in hex format
/// return: the JubjubSignature
pub fn unified_sign_withdrawal(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Withdrawal = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key);
    Ok(signature.into())
}

/// Hash a withdrawal transaction
/// json: the withdrawal transaction in json format
/// return: the hash in hex format
pub fn unified_hash_withdrawal(json: &str) -> anyhow::Result<String> {
    let req: Withdrawal = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(req.hash() as HashType))
}

/// Sign a spot trade transaction
/// json: the spot trade transaction in json format
/// private_key_a: the private key of party A in hex format
/// private_key_b: the private key of party B in hex format
/// return: the JubjubSignature of both A and B
pub fn unified_sign_spot_trade(
    json: &str,
    private_key_a: &str,
    private_key_b: &str,
) -> anyhow::Result<(JubjubSignature, JubjubSignature)> {
    let req: SpotTrade = serde_json::from_str(json)?;
    let private_key_a = private_key_from_string(private_key_a)?;
    let private_key_b = private_key_from_string(private_key_b)?;
    let signature_a = req.party_a_order.sign(&private_key_a);
    let signature_b = req.party_b_order.sign(&private_key_b);
    Ok((signature_a.into(), signature_b.into()))
}

/// Hash a spot trade transaction
/// json: the spot trade transaction in json format
pub fn unified_hash_spot_trade(json: &str) -> anyhow::Result<(String, String)> {
    let req: SpotTrade = serde_json::from_str(json)?;
    Ok((
        hash_type_to_string_with_0xprefix(req.party_a_order.hash() as HashType),
        hash_type_to_string_with_0xprefix(req.party_b_order.hash() as HashType),
    ))
}

/// Sign a perpetual trade transaction
/// json: the perpetual trade transaction in json format
/// private_key_a: the private key of party A in hex format
/// private_key_b: the private key of party B in hex format
/// return: the JubjubSignature of both A and B
pub fn unified_sign_perpetual_trade(
    json: &str,
    private_key_a: &str,
    private_key_b: &str,
) -> anyhow::Result<(JubjubSignature, JubjubSignature)> {
    let req: PerpetualTrade = serde_json::from_str(json)?;
    let private_key_a = private_key_from_string(private_key_a)?;
    let private_key_b = private_key_from_string(private_key_b)?;
    let signature_a = req.party_a_order.sign(&private_key_a);
    let signature_b = req.party_b_order.sign(&private_key_b);
    Ok((signature_a.into(), signature_b.into()))
}

/// Hash a perpetual trade transaction
/// json: the perpetual trade transaction in json format
/// return: the hash of both A and B in hex format
pub fn unified_hash_perpetual_trade(json: &str) -> anyhow::Result<(String, String)> {
    let req: PerpetualTrade = serde_json::from_str(json)?;
    Ok((
        hash_type_to_string_with_0xprefix(req.party_a_order.hash() as HashType),
        hash_type_to_string_with_0xprefix(req.party_b_order.hash() as HashType),
    ))
}

/// Sign a transfer transaction
/// json: the transfer transaction in json format
/// private_key: the private key in hex format
/// return: the JubjubSignature
pub fn unified_sign_transfer(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Transfer = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key);
    Ok(signature.into())
}

/// Hash a transfer transaction
/// json: the transfer transaction in json format
/// return: the hash in hex format
pub fn unified_hash_transfer(json: &str) -> anyhow::Result<String> {
    let req: Transfer = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(req.hash() as HashType))
}

/// Sign an oracle price transaction
/// json: the oracle price transaction in json format
/// private_key: the private key in hex format
/// return: the JubjubSignature
pub fn unified_sign_oracle_price(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key);
    Ok(signature.into())
}

/// Hash an oracle price transaction
/// json: the oracle price transaction in json format
/// return: the hash in hex format
pub fn unified_hash_oracle_price(json: &str) -> anyhow::Result<String> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(req.hash() as HashType))
}

/// Sign a liquidate transaction
/// json: the liquidate transaction in json format
/// private_key: the private key in hex format
/// return: the JubjubSignature
pub fn unified_sign_liquidate(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Liquidate = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key);
    Ok(signature.into())
}

/// Hash a liquidate transaction
/// json: the liquidate transaction in json format
/// return: the hash in hex format
pub fn unified_hash_liquidate(json: &str) -> anyhow::Result<String> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(req.hash() as HashType))
}

/// Sign a spot limit order transaction
/// json: the spot limit order transaction in json format
/// private_key: the private key in hex format
/// return: the JubjubSignature
pub fn unified_sign_spot_limit_order(
    json: &str,
    private_key: &str,
) -> anyhow::Result<JubjubSignature> {
    let req: transactions::order::spot::LimitOrder = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key);
    Ok(signature.into())
}

/// Hash a spot limit order transaction
/// json: the spot limit order transaction in json format
/// return: the hash in hex format
pub fn unified_hash_spot_limit_order(json: &str) -> anyhow::Result<String> {
    let req: transactions::order::spot::LimitOrder = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(req.hash() as HashType))
}

/// Sign a perpetual limit order transaction
/// json: the perpetual limit order transaction in json format
/// private_key: the private key in hex format
/// return: the JubjubSignature
pub fn unified_sign_perpetual_limit_order(
    json: &str,
    private_key: &str,
) -> anyhow::Result<JubjubSignature> {
    let req: transactions::order::perpetual::LimitOrder = serde_json::from_str(json)?;
    let private_key = private_key_from_string(private_key)?;
    let signature = req.sign(&private_key);
    Ok(signature.into())
}

/// Hash a perpetual limit order transaction
/// json: the perpetual limit order transaction in json format
/// return: the hash in hex format
pub fn unified_hash_perpetual_limit_order(json: &str) -> anyhow::Result<String> {
    let req: transactions::order::perpetual::LimitOrder = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(req.hash() as HashType))
}
