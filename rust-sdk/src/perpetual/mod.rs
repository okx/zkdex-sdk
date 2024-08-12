use crate::hash_type::hash_type_to_string_with_0xprefix;
use crate::perpetual::transaction::limit_order::LimitOrderRequest;
use crate::perpetual::transaction::liquidate::Liquidate;
use crate::perpetual::transaction::oracle_price::{signed_oracle_price_hash, SignedOraclePrice};
use crate::perpetual::transaction::transfer::{transfer_hash, Transfer};
use crate::perpetual::transaction::withdraw::{withdrawal_hash, Withdraw, WithdrawRequest};
use crate::perpetual::transaction::{limit_order, oracle_price, transfer, withdraw};
use crate::zkw::JubjubSignature;

mod transaction;
mod types;

pub fn sign_transfer(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Transfer = serde_json::from_str(json).unwrap();
    Ok(transfer::sign_transfer(req, private_key)?)
}

pub fn hash_transfer(json: &str) -> anyhow::Result<String> {
    let req: Transfer = serde_json::from_str(json).unwrap();
    Ok(hash_type_to_string_with_0xprefix(transfer_hash(&req, 0)))
}

/// Hash a perpetual limit order transaction
/// json: the perpetual limit order transaction in json format
/// return: the hash in hex format
pub fn sign_withdraw(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let withdraw_req: WithdrawRequest = serde_json::from_str(json)?;
    let withdraw = Withdraw {
        base: withdraw_req.base,
        position_id: withdraw_req.position_id,
        amount: withdraw_req.amount,
        owner_key: withdraw_req.owner_key,
    };
    Ok(withdraw::sign_withdraw(
        withdraw,
        &withdraw_req.asset_id,
        private_key,
    )?)
}

/// Hash a perpetual limit order transaction
/// json: the perpetual limit order transaction in json format
/// return: the hash in hex format
pub fn hash_withdraw(json: &str) -> anyhow::Result<String> {
    let withdraw_req: WithdrawRequest = serde_json::from_str(json)?;
    let withdraw = Withdraw {
        base: withdraw_req.base,
        position_id: withdraw_req.position_id,
        amount: withdraw_req.amount,
        owner_key: withdraw_req.owner_key,
    };
    Ok(hash_type_to_string_with_0xprefix(withdrawal_hash(
        &withdraw,
        &withdraw_req.asset_id,
    )))
}

pub fn sign_limit_order(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: LimitOrderRequest = serde_json::from_str(json)?;
    Ok(limit_order::sign_limit_order(req, private_key)?)
}

pub fn hash_limit_order(json: &str) -> anyhow::Result<String> {
    let req: LimitOrderRequest = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(
        limit_order::hash_limit_order(req),
    ))
}

pub fn sign_liquidate(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(limit_order::sign_limit_order(
        req.liquidator_order,
        private_key,
    )?)
}

pub fn hash_liquidate(json: &str) -> anyhow::Result<String> {
    let req: Liquidate = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(
        limit_order::hash_limit_order(req.liquidator_order),
    ))
}

pub fn sign_signed_oracle_price(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    Ok(oracle_price::sign_signed_oracle_price(req, private_key)?)
}

pub fn hash_signed_oracle_price(json: &str) -> anyhow::Result<String> {
    let req: SignedOraclePrice = serde_json::from_str(json)?;
    Ok(hash_type_to_string_with_0xprefix(signed_oracle_price_hash(
        &req,
    )))
}

#[cfg(test)]
mod test {
    use crate::crypto::packed_public_key::{private_key_from_string, public_key_from_private};
    use crate::crypto::public_key_type::PublicKeyType;
    use crate::helper::{verify_valid_sig, PRI_KEY, PUB_KEY};
    use crate::perpetual::transaction::limit_order::LimitOrderRequest;
    use crate::perpetual::{
        hash_limit_order, hash_liquidate, hash_signed_oracle_price, hash_transfer, hash_withdraw,
        sign_limit_order, sign_liquidate, sign_signed_oracle_price, sign_transfer, sign_withdraw,
    };
    use crate::{private_key_to_pubkey_xy, verify_jubjub_signature, verify_signature};
    use other_test::Bencher;

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
        let sig = sign_withdraw(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_withdraw(json).unwrap()).unwrap());
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
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faaaa",
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
        "public_key":"0x92ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faacccccccccc",
        "expiration_timestamp":"1684832800",
        "position_id":"2",
        "amount":"3",
        "eth_address":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "asset_id":"0x1"
        }
        "#;
        let sig = sign_withdraw(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_withdraw(json).unwrap()).unwrap());
    }

    #[test]
    #[should_panic]
    pub fn test_withdraw_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        let sig = sign_withdraw(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_withdraw(json).unwrap()).unwrap());
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
        let sig = sign_transfer(json, PRI_KEY).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash).unwrap());
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
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faaaaa",
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
        "public_key":"0x7092ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faabbbbbbbbbb",
        "expiration_timestamp":"0",
        "sender_position_id":"0",
        "receiver_public_key":"0x7092ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa",
        "receiver_position_id":"0",
        "amount":"0",
        "asset_id":"0xa"
        }
        "#;
        let sig = sign_transfer(json, PRI_KEY).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash).unwrap());
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
        let sig = sign_transfer(json, PRI_KEY).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash).unwrap());
    }

    #[test]
    #[should_panic]
    pub fn test_sign_transfer_with_empty_json() {
        let json = r#"
        {
        }
        "#;
        let sig = sign_transfer(json, PRI_KEY).unwrap();
        let hash = hash_transfer(json).unwrap();
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash).unwrap());
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
        let sig = sign_limit_order(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_limit_order(json).unwrap()).unwrap())
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
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faaa",
        "expiration_timestamp":"2",
        "amount_synthetic":"3",
        "amount_collateral":"4",
        "amount_fee":"5",
        "asset_id_synthetic":"0x6",
        "asset_id_collateral":"0x7",
        "position_id":"8",
        "is_buying_synthetic":false
        }"#;
        let req: LimitOrderRequest = serde_json::from_str(json).unwrap();
        let _pk: PublicKeyType = req.base.public_key.into();

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
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faaa",
        "expiration_timestamp":"2",
        "amount_synthetic":"3",
        "amount_collateral":"4",
        "amount_fee":"5",
        "asset_id_synthetic":"0x6",
        "asset_id_collateral":"0x7",
        "position_id":"8",
        "is_buying_synthetic":false
        }"#;
        let sig = sign_limit_order(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_limit_order(json).unwrap()).unwrap())
    }

    #[test]
    #[should_panic]
    pub fn test_sign_limit_order_with_empty_json() {
        let json = r#"{

        }"#;
        let sig = sign_limit_order(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_limit_order(json).unwrap()).unwrap())
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

        let sig = sign_liquidate(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_liquidate(json).unwrap()).unwrap());
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
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faaa",
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
        "public_key":"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e586aaaaaaaaa",
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

        let sig = sign_liquidate(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_liquidate(json).unwrap()).unwrap());
    }

    #[test]
    #[should_panic]
    pub fn test_sign_liquidate_with_empty_json() {
        let json = r#"
        "#;

        let sig = sign_liquidate(json, PRI_KEY).unwrap();
        verify_valid_sig(&sig);
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_liquidate(json).unwrap()).unwrap());
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
        verify_valid_sig(&sig1);
        assert!(verify_jubjub_signature(
            sig1,
            &public_key_from_private(&private_key_from_string(pri1).unwrap()).to_string(),
            &hash1,
        )
        .unwrap());
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
        let hash2 = hash_signed_oracle_price(json2).unwrap();
        verify_valid_sig(&sig2);
        println!("sig2: {}", serde_json::to_string(&sig2).unwrap());
        assert!(verify_jubjub_signature(
            sig2,
            &public_key_from_private(&private_key_from_string(pri2).unwrap()).to_string(),
            &hash2,
        )
        .unwrap());

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
        let hash3 = hash_signed_oracle_price(json3).unwrap();
        verify_valid_sig(&sig3);
        assert!(verify_jubjub_signature(
            sig3,
            &public_key_from_private(&private_key_from_string(pri3).unwrap()).to_string(),
            &hash3,
        )
        .unwrap());

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
        let hash4 = hash_signed_oracle_price(json4).unwrap();
        verify_valid_sig(&sig4);
        assert!(verify_jubjub_signature(
            sig4,
            &public_key_from_private(&private_key_from_string(pri2).unwrap()).to_string(),
            &hash4,
        )
        .unwrap());
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
        "signer_key": "0x15d144b7facdffd112bc06640c3bd4e5f36ad077ca9f9b97ad3f8f85906236a4a",
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
        "signer_key": "0xa09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2aaaaaaaaaaaa",
        "external_price": "28409392522000000000000",
        "timestamp": "1693907824",
        "signed_asset_id": "0x425443555344434f4b580000000000005374437277"
        }
        "#;
        let pri1 = "01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
        let _ = sign_signed_oracle_price(json1, pri1).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn test_sign_oracle_price_with_empty_json() {
        let json = r#"
        {

        }
        "#;
        let _ = sign_signed_oracle_price(json, PRI_KEY).unwrap();

        let hash = hash_signed_oracle_price(json);
        assert!(hash.is_ok())
    }

    #[bench]
    fn bench_verify_transfer(b: &mut Bencher) {
        let json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        let sig_r = "0x1c929aba1dd2f9cacf5c857e014b2ea1bbd98e5758821a20293b12c869e51732";
        let sig_s = "0x03d739463c57a40e49b8e52f54c18acce5f205ee9ffcee2b96ac83bc3fbcf476";
        let (pk_x, pk_y) = private_key_to_pubkey_xy(PRI_KEY).unwrap();

        b.iter(|| {
            let hash = hash_transfer(json).unwrap();
            assert!(verify_signature(sig_r, sig_s, &pk_x, &pk_y, &hash).unwrap());
        })
    }

    #[bench]
    fn bench_sign_transfer(b: &mut Bencher) {
        let json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        b.iter(|| {
            assert!(sign_transfer(json, PRI_KEY).is_ok());
        })
    }
}
