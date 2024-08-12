mod transactions;
mod types;

use crate::hash_type::hash_type_to_string_with_0xprefix;
use crate::spot;
use crate::spot::transactions::{limit_order, sign_transfer, sign_withdrawal, transfer_hash, Transfer, Withdrawal};
use crate::zkw::JubjubSignature;


pub fn sign_spot_transfer(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Transfer = serde_json::from_str(json).unwrap();
    Ok(sign_transfer(req, private_key)?)
}

pub fn hash_spot_transfer(json: &str) -> anyhow::Result<String> {
    let req: spot::Transfer = serde_json::from_str(json).unwrap();
    Ok(hash_type_to_string_with_0xprefix(transfer_hash(&req)))
}

pub fn sign_spot_limit_order(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: spot::limit_order::LimitOrder = serde_json::from_str(json).unwrap();
    Ok(spot::limit_order::sign_limit_order(&req, private_key)?)
}

pub fn hash_spot_limit_order(json: &str) -> anyhow::Result<String> {
    let req: limit_order::LimitOrder = serde_json::from_str(json).unwrap();
    Ok(hash_type_to_string_with_0xprefix(req.hash()))
}

pub fn sign_spot_withdrawal(json: &str, private_key: &str) -> anyhow::Result<JubjubSignature> {
    let req: Withdrawal = serde_json::from_str(json).unwrap();
    Ok(sign_withdrawal(&req, private_key)?)
}

pub fn hash_spot_withdrawal(json: &str) -> anyhow::Result<String> {
    let req: Withdrawal = serde_json::from_str(json).unwrap();
    Ok(hash_type_to_string_with_0xprefix(req.hash()))
}

#[cfg(test)]
mod test {
    use crate::helper::{PRI_KEY, PUB_KEY};
    use crate::spot::{
        hash_spot_limit_order, hash_spot_transfer, hash_spot_withdrawal, sign_spot_limit_order,
        sign_spot_transfer, sign_spot_withdrawal,
    };
    use crate::verify_jubjub_signature;

    #[test]
    pub fn test_sign_spot_transfer() {
        let json = r#"
        {

        "nonce": "1",
        "sender_public_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
        "expiration_timestamp": "3608164305",
        "amount": "10",
        "asset_id": "0x00001",
        "receiver_position_id": "1",
        "receiver_public_key": "0x0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
        "sender_position_id": "1"

        }"#;

        let sig = sign_spot_transfer(json, PRI_KEY).unwrap();
        assert!(verify_jubjub_signature(sig, PUB_KEY, &hash_spot_transfer(json).unwrap()).unwrap());
    }

    #[test]
    pub fn test_sign_spot_limit_order() {
        let json = r#"{
            "nonce": "0",
            "expiration_timestamp": "0",
            "public_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
            "amount_buy": "0",
            "amount_sell": "0",
            "amount_fee": "0",
            "asset_buy":"0x01",
            "asset_sell":"0x02",
            "position_id":"1"

            }"#;
        let sig = sign_spot_limit_order(json, PRI_KEY).unwrap();
        assert!(
            verify_jubjub_signature(sig, PUB_KEY, &hash_spot_limit_order(json).unwrap()).unwrap()
        );
    }

    #[test]
    pub fn test_sign_spot_withdrawal() {
        let json_str = r##"{
        "nonce": "1",
        "public_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
        "expiration_timestamp": "3608164305",
        "amount": "1000000",
        "asset_id": "0x00001",
        "position_id": "1",
        "chain_id": "1",
        "fee": "0",
        "eth_address": "0x0"
        }"##;

        let sig = sign_spot_withdrawal(json_str, PRI_KEY).unwrap();
        println!("sig: {}", serde_json::to_string(&sig).unwrap());
        assert!(
            verify_jubjub_signature(sig, PUB_KEY, &hash_spot_withdrawal(json_str).unwrap())
                .unwrap()
        );
    }
}
