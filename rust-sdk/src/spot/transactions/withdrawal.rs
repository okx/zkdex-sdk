use crate::common::OrderBase;
use crate::constant::{SPOT_WITHDRAWAL, SPOT_WITHDRAWAL_TO_OWNER_KEY};
use crate::felt::LeBytesConvert;
use crate::hash::hash2;
use crate::tx::public_key_type::PublicKeyType;
use crate::tx::{private_key_from_string, HashType, TxSignature};
use crate::types::{SpotAmountType, SpotAssetIdType, SpotPositionIdType};
use crate::zkw::JubjubSignature;
use primitive_types::U256;
use {
    crate::serde_wrapper::{
        SpotAmountTypeSerdeAsRadix10String, SpotAssetIdTypeSerdeAsRadix16String,
        SpotPositionIdTypeSerdeAsRadix10String,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Withdrawal {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "amount", with = "SpotAmountTypeSerdeAsRadix10String")]
    pub amount: SpotAmountType,
    #[serde(rename = "eth_address")]
    pub owner_key: PublicKeyType,
    #[serde(rename = "asset_id", with = "SpotAssetIdTypeSerdeAsRadix16String")]
    pub asset_id: SpotAssetIdType,
    #[serde(
        rename = "position_id",
        with = "SpotPositionIdTypeSerdeAsRadix10String"
    )]
    pub position_id: SpotPositionIdType,
}

impl Withdrawal {
    pub fn hash(&self) -> HashType {
        let packed_message0;

        // If owner_key is equal to public key, this is a withdrawal of the old API and therefore the
        // transaction type id is different and the owner_key is not part of the message.
        // local has_address = withdrawal.owner_key - withdrawal.base.public_key;
        let has_address = &self.owner_key != &self.base.public_key;

        let prefix;

        if !has_address {
            packed_message0 = U256::from(self.asset_id);
            prefix = SPOT_WITHDRAWAL;
        } else {
            packed_message0 = hash2(&(self.asset_id as u64), &self.owner_key);
            prefix = SPOT_WITHDRAWAL_TO_OWNER_KEY;
        }

        let packed_message1 = U256([
            (self.base.expiration_timestamp as u64) << 32 | self.base.nonce as u64,
            self.amount as u64,
            (self.amount >> 64) as u64,
            (prefix << 32) | self.position_id as u64,
        ]) << 17;

        hash2(&packed_message0, &packed_message1)
    }
}

pub fn sign_withdrawal(
    withdrawal: &Withdrawal,
    private_key: &str,
) -> anyhow::Result<JubjubSignature> {
    let hash = withdrawal.hash();
    let private_key = private_key_from_string(private_key).unwrap();
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}
