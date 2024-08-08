use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::common::OrderBase;
use crate::constant::{UNIFIED_WITHDRAWAL, UNIFIED_WITHDRAWAL_TO_OWNER_KEY};
use crate::hash::new_hasher;
use crate::hash::Hasher;
use crate::serde_utils::serde_str;
use crate::tx::public_key_type::PublicKeyType;
use crate::types::AmountType;
use crate::unified::transactions::hash_trait::HashTrait;
use crate::unified::transactions::sign_trait::SignTrait;
use crate::unified::types::chain_id::ChainIdType;
use crate::unified::types::{AssetIdType, PositionIdType};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Withdrawal {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "eth_address")]
    pub owner_key: PublicKeyType,
    #[serde(with = "serde_str")]
    pub amount: AmountType,
    #[serde(with = "serde_str")]
    pub fee: AmountType,
    pub asset_id: AssetIdType,
    pub position_id: PositionIdType,
    pub chain_id: ChainIdType,
}

impl HashTrait for Withdrawal {
    fn hash(&self) -> U256 {
        let mut hasher = new_hasher();

        // If owner_key is equal to public key, this is a withdrawal of the old API and therefore the
        // transaction type id is different and the owner_key is not part of the message.
        let prefix;
        let has_address = &self.owner_key != &self.base.public_key;
        if !has_address {
            prefix = UNIFIED_WITHDRAWAL;
            hasher.update_single(&prefix);
            hasher.update_single(&(self.asset_id.0 as u64));
        } else {
            prefix = UNIFIED_WITHDRAWAL_TO_OWNER_KEY;
            hasher.update_single(&prefix);
            hasher.update_single(&(self.asset_id.0 as u64));
            hasher.update_single(&self.owner_key);
        }

        let packed_message = U256([self.chain_id.into(), self.fee, 0, 0]);

        hasher.update_single(&packed_message);

        let packed_message = U256([
            (self.base.expiration_timestamp as u64) << 32 | self.base.nonce as u64,
            self.amount,
            0,
            (prefix << 32) | self.position_id.0 as u64,
        ]) << 17;

        hasher.update_single(&packed_message);

        hasher.finalize()
    }
}

impl SignTrait for Withdrawal {}

#[cfg(test)]
mod tests {
    use crate::unified::transactions::test::sign_and_verify;
    use crate::unified::transactions::Withdrawal;

    #[test]
    fn test_withdrawal_serde() {
        let js = r##"
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
        "##;
        let tx = serde_json::from_str::<Withdrawal>(js);
        assert!(tx.is_ok());
    }

    #[test]
    fn test_sign_and_verify() {
        let js = r##"
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
        "##;
        let tx = serde_json::from_str::<Withdrawal>(js);
        assert!(tx.is_ok());
        let tx = tx.unwrap();
        sign_and_verify(tx);
    }
}
