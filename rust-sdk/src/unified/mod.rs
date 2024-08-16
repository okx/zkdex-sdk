//! Unified API for all transactions.
//! In this module, we provide a unified API for all transactions, including withdrawal, spot trade, perpetual trade, transfer, oracle price, and liquidate.
//! And also, we provide the hash and sign functions for each transaction.

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

#[cfg(test)]
mod test {
    use crate::helper::PRI_KEY;
    use crate::{
        unified_hash_liquidate, unified_hash_oracle_price, unified_hash_perpetual_limit_order,
        unified_hash_perpetual_trade, unified_hash_spot_limit_order, unified_hash_spot_trade,
        unified_hash_transfer, unified_hash_withdrawal, unified_sign_liquidate,
        unified_sign_oracle_price, unified_sign_perpetual_limit_order,
        unified_sign_perpetual_trade, unified_sign_spot_limit_order, unified_sign_spot_trade,
        unified_sign_transfer, unified_sign_withdrawal,
    };

    #[test]
    fn test_sign_withdrawal() {
        let json = r#"
        {
            "amount": "1682637359498011204",
            "eth_address": "0xB6aD5EfBd6aDfa29dEfad5BC0f8cE0ad57d4c5Fb",
            "expiration_timestamp": "2101470722",
            "asset_id": "0x11111",
            "nonce": "4265854110",
            "position_id": "775817640",
            "fee":"0",
            "public_key": "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a",
            "chain_id": "123"
        }
        "#;
        assert!(unified_sign_withdrawal(json, PRI_KEY).is_ok());
    }

    #[test]
    fn test_hash_withdrawal() {
        let json = r#"
        {
            "amount": "1682637359498011204",
            "eth_address": "0xB6aD5EfBd6aDfa29dEfad5BC0f8cE0ad57d4c5Fb",
            "expiration_timestamp": "2101470722",
            "asset_id": "0x11111",
            "nonce": "4265854110",
            "position_id": "775817640",
            "fee":"0",
            "public_key": "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a",
            "chain_id": "123"
        }
        "#;
        assert!(unified_hash_withdrawal(json).is_ok());
    }

    #[test]
    fn test_spot_trade() {
        let json = r#"
        {
            "party_a_order": {
                "amount_buy": "80",
                "amount_sell": "70",
                "amount_fee": "111",
                "expiration_timestamp": "3396833",
                "nonce": "1654615998",
                "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
                "signature": {
                    "r": "0x9d4ed071faf946d6e22aa9f72016b4dcb07137bfdd976a1482d26c862bc1cd6e",
                    "s": "0x2baed04d00fbf2109d9d53235f6168d6e43b8858fc9b435bb8823e7d3aff335"
                 },
                "asset_buy": "0x22222",
                "asset_sell": "0x1111",
                "position_id": "922337"
            },
            "party_b_order": {
                "amount_buy": "80",
                "amount_sell": "70",
                "amount_fee": "111",
                "expiration_timestamp": "3396833",
                "nonce": "1654615998",
                "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
                "signature": {
                    "r": "0x9d4ed071faf946d6e22aa9f72016b4dcb07137bfdd976a1482d26c862bc1cd6e",
                    "s": "0x2baed04d00fbf2109d9d53235f6168d6e43b8858fc9b435bb8823e7d3aff335"
                 },
                "asset_buy": "0x2222",
                "asset_sell": "0x111",
                "position_id": "9223"
            },
            "actual_a_sold": "30",
            "actual_b_sold": "40",
            "actual_a_fee": "1",
            "actual_b_fee": "-2",
            "type": "UNIFIED_SPOT_TRADE"
        }"#;
        assert!(unified_sign_spot_trade(json, PRI_KEY, PRI_KEY).is_ok());
        assert!(unified_hash_spot_trade(json).is_ok());
    }

    #[test]
    fn test_spot_limit_order() {
        let json = r#"
        {
                "amount_buy": "80",
                "amount_sell": "70",
                "amount_fee": "111",
                "expiration_timestamp": "3396833",
                "nonce": "1654615998",
                "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
                "signature": {
                    "r": "0x9d4ed071faf946d6e22aa9f72016b4dcb07137bfdd976a1482d26c862bc1cd6e",
                    "s": "0x2baed04d00fbf2109d9d53235f6168d6e43b8858fc9b435bb8823e7d3aff335"
                 },
                "asset_buy": "0x22222",
                "asset_sell": "0x1111",
                "position_id": "922337"
        }
        "#;
        assert!(unified_sign_spot_limit_order(json, PRI_KEY).is_ok());
        assert!(unified_hash_spot_limit_order(json).is_ok());
    }

    #[test]
    fn test_liquidate() {
        let json = r##"
{
    "actual_collateral":"7758176404715800194",
    "actual_liquidator_fee":"8791662011684601223",
    "actual_synthetic":"15308084094301570617",
    "liquidated_position_id":"1541968236",
    "liquidated_type":"PERP_CROSS",
    "liquidator_order":{
        "amount_collateral":"8187132600743567510",
        "amount_fee":"11081939229867047606",
        "amount_synthetic":"16558026091473266411",
        "asset_id_collateral":"0x57d05d1",
        "asset_id_synthetic":"0x2",
        "expiration_timestamp":"1430804514",
        "is_buying_synthetic":false,
        "type":"PERP_CROSS",
        "nonce":"3900315155",
        "position_id":"11534",
        "public_key":"0x5db665983e23607de57d6dc068797336bfdcb954238044688bec922ca296d3e"
    }
}
        "##;
        assert!(unified_sign_liquidate(json, PRI_KEY).is_ok());
        assert!(unified_hash_liquidate(json).is_ok());
    }

    #[test]
    fn test_perpetual_limit_order() {
        let json = r#"
        {
        "type":"PERP_CROSS",
        "amount_collateral":"15334874",
        "amount_fee":"1767749",
        "amount_synthetic":"15460142",
        "asset_id_collateral":"0x57d05d",
        "asset_id_synthetic":"0x2",
        "expiration_timestamp":"3608164305",
        "is_buying_synthetic":true,
        "nonce":"1210484339",
        "order_type":"LIMIT_ORDER_WITH_FEES",
        "position_id":"4805234",
        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7"
        }
        "#;
        assert!(unified_sign_perpetual_limit_order(json, PRI_KEY).is_ok());
        assert!(unified_hash_perpetual_limit_order(json).is_ok());
    }

    #[test]
    fn test_oracle_price() {
        let json = r##"
            {
            "signer_key": "0x87e5235c9c3916ef2b0def77111366ecef72914613f52febad308440b6463f83",
            "external_price": "30000000",
            "timestamp": "1651148012",
            "signed_asset_id": "0x425443555344000000000000000000004d616b6572"
            }
        "##;
        assert!(unified_sign_oracle_price(json, PRI_KEY).is_ok());
        assert!(unified_hash_oracle_price(json).is_ok());
    }

    #[test]
    fn test_transfer() {
        let json = r##"
        {
            "amount": "7758176404715800194",
            "asset_id": "0x1234",
            "synthetic_id" : "0x0",
            "expiration_timestamp": "2404381470",
            "nonce": "2195908194",
            "receiver_position_id": "609106",
            "receiver_public_key": "0x259f432e6f4590b9a164106cf6a659eb4862b21fb97d43588561712e8e5216b",
            "sender_position_id": "93098",
            "sender_public_key": "0x28e4d45cd0538ffa6fdc09e70f0fea4e56c47fda87a2a969c22b4fdfe997f60"
        }
        "##;
        assert!(unified_sign_transfer(json, PRI_KEY).is_ok());
        assert!(unified_hash_transfer(json).is_ok());
    }

    #[test]
    fn test_perpetual_trade() {
        let json = r##"
{
    "party_a_order":{
        "type":"PERP_CROSS",
        "amount_collateral":"15334874",
        "amount_fee":"1767749",
        "amount_synthetic":"15460142",
        "asset_id_collateral":"0x57d05d",
        "asset_id_synthetic":"0x2",
        "expiration_timestamp":"3608164305",
        "is_buying_synthetic":true,
        "nonce":"1210484339",
        "order_type":"LIMIT_ORDER_WITH_FEES",
        "position_id":"4805234",
        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7"

    },
    "party_b_order":{
        "type":"PERP_CROSS",
        "amount_collateral":"15334874138764573096",
        "amount_fee":"17677494534592486883",
        "amount_synthetic":"15460142528840632302",
        "asset_id_collateral":"0x57d05d",
        "asset_id_synthetic":"0x2",
        "expiration_timestamp":"36081",
        "is_buying_synthetic":true,
        "nonce":"12104",
        "order_type":"LIMIT_ORDER_WITH_FEES",
        "position_id":"48052349",
        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7"
    },
    "actual_a_fee":"87916620",
    "actual_b_fee":"-9309",
    "actual_collateral":"775817",
    "actual_synthetic":"1530808",
    "type":"UNIFIED_PERP_TRADE"
}
        "##;
        assert!(unified_sign_perpetual_trade(json, PRI_KEY, PRI_KEY).is_ok());
        assert!(unified_hash_perpetual_trade(json).is_ok());
    }
}
