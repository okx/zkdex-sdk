use {
    crate::U64SerdeStr,
    serde::{Deserialize, Serialize},
};

use crate::unified::transactions::order::perpetual::LimitOrder;
use crate::unified::types::{AmountType, SignedAmountType};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct PerpetualTrade {
    pub party_a_order: LimitOrder,
    pub party_b_order: LimitOrder,
    #[serde(with = "U64SerdeStr")]
    pub actual_collateral: AmountType,
    #[serde(with = "U64SerdeStr")]
    pub actual_synthetic: AmountType,
    pub actual_a_fee: SignedAmountType,
    pub actual_b_fee: SignedAmountType,
}

#[cfg(test)]
mod tests {
    use crate::unified::transactions::test::sign_and_verify;

    use super::*;

    #[test]
    fn test_trade_serde() {
        let js = r##"
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
        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7",
        "signature": {
            "r": "0x9d4ed071faf946d6e22aa9f72016b4dcb07137bfdd976a1482d26c862bc1cd6e",
            "s": "0x2baed04d00fbf2109d9d53235f6168d6e43b8858fc9b435bb8823e7d3aff335"
        }
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
        "public_key":"0x6b974202431eb8c0692c9c8111528d947bc7e70f7ffefaffbab7455dfa5d4f7",
        "signature": {
            "r": "0x9d4ed071faf946d6e22aa9f72016b4dcb07137bfdd976a1482d26c862bc1cd6e",
            "s": "0x2baed04d00fbf2109d9d53235f6168d6e43b8858fc9b435bb8823e7d3aff335"
        }
    },
    "actual_a_fee":"87916620",
    "actual_b_fee":"-9309",
    "actual_collateral":"775817",
    "actual_synthetic":"1530808",
    "type":"UNIFIED_PERP_TRADE"
}
        "##;
        let tx = serde_json::from_str::<PerpetualTrade>(js);

        assert_eq!(tx.is_ok(), true);
    }

    #[test]
    fn test_sign_verify() {
        let js = r##"
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
    "actual_synthetic":"1530808"
}
        "##;
        let tx = serde_json::from_str::<PerpetualTrade>(js);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx.party_a_order);
        sign_and_verify(tx.party_b_order);
    }
}
