use serde::{Deserialize, Serialize};

use crate::serde_utils::serde_str;
use crate::unified::transactions::order::spot::LimitOrder;
use crate::unified::types::{AmountType, SignedAmountType};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SpotTrade {
    pub party_a_order: LimitOrder,
    pub party_b_order: LimitOrder,
    #[serde(with = "serde_str")]
    pub actual_a_sold: AmountType,
    #[serde(with = "serde_str")]
    pub actual_b_sold: AmountType,
    pub actual_a_fee: SignedAmountType,
    pub actual_b_fee: SignedAmountType,
}

#[cfg(test)]
mod tests {
    use crate::unified::transactions::spot_trade::SpotTrade;
    use crate::unified::transactions::test::sign_and_verify;

    #[test]
    fn test_trade_serde() {
        let js = r##"
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
        }
        "##;
        let tx = serde_json::from_str::<SpotTrade>(js);
        assert!(tx.is_ok());
    }

    #[test]
    fn test_sign_and_verify() {
        let js = r##"
        {
            "party_a_order": {
                "amount_buy": "80",
                "amount_sell": "70",
                "amount_fee": "111",
                "expiration_timestamp": "3396833",
                "nonce": "1654615998",
                "public_key": "0x19c78df8f4ff31e78de58575487ce1eaf19922ad9b8a714e61a441c12e0c8b2",
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
                "asset_buy": "0x2222",
                "asset_sell": "0x111",
                "position_id": "9223"
            },
            "actual_a_sold": "30",
            "actual_b_sold": "40",
            "actual_a_fee": "1",
            "actual_b_fee": "-2"
        }
        "##;
        let tx = serde_json::from_str::<SpotTrade>(js);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx.party_a_order);
        sign_and_verify(tx.party_b_order);
    }
}