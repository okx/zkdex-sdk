use crate::common::OrderBase;
use crate::constant::{SPOT_WITHDRAWAL, SPOT_WITHDRAWAL_TO_OWNER_KEY};
use crate::felt::LeBytesConvert;
use crate::hash;
use crate::hash::Hasher;
use crate::serde_utils::serde_str;
use crate::serde_wrapper::u32_serde::U32SerdeAsString;
use crate::spot::types::AmountType;
use crate::tx::public_key_type::PublicKeyType;
use crate::tx::{private_key_from_string, HashType, TxSignature};
use crate::types::asset_id::AssetIdType;
use crate::types::position_id::PositionIdType;
use crate::zkw::JubjubSignature;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Withdrawal {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(with = "serde_str")]
    pub amount: AmountType,
    #[serde(rename = "eth_address")]
    pub owner_key: PublicKeyType,
    #[serde(rename = "asset_id")]
    pub asset_id: AssetIdType,
    #[serde(rename = "position_id")]
    pub position_id: PositionIdType,
    #[serde(with = "serde_str", default)]
    pub fee: AmountType,
    #[serde(rename = "chain_id", with = "U32SerdeAsString")]
    pub chain_id: u32,
}

impl Withdrawal {
    pub fn hash(&self) -> HashType {
        let mut hasher = hash::new_hasher();
        // If owner_key is equal to public key, this is a withdrawal of the old API and therefore the
        // transaction type id is different and the owner_key is not part of the message.
        let prefix;
        let has_address = &self.owner_key != &self.base.public_key;
        if !has_address {
            prefix = SPOT_WITHDRAWAL;
            hasher.update_single(&prefix);
            hasher.update_single(&(self.asset_id.0 as u64));
        } else {
            prefix = SPOT_WITHDRAWAL_TO_OWNER_KEY;
            hasher.update_single(&prefix);
            hasher.update_single(&(self.asset_id.0 as u64));
            hasher.update_single(&self.owner_key);
        }

        let packed0 = U256([self.chain_id as u64, self.fee, 0, 0]);
        hasher.update_single(&packed0);

        let packed_message1 = U256([
            (self.base.expiration_timestamp as u64) << 32 | self.base.nonce as u64,
            self.amount,
            0,
            u64::from(self.position_id.0),
        ]);

        hasher.update_single(&packed_message1);

        hasher.finalize()
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

#[cfg(test)]
mod test {
    use crate::spot::Withdrawal;

    #[test]
    pub fn test_deserialize() {
        let json_str = r##"
        {
        "nonce": "1",
        "public_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
        "expiration_timestamp": "3608164305",
        "amount": "1000000",
        "asset_id": "0x00001",
        "position_id": "1",
        "eth_address": "0x0",
        "chain_id": "1",
        "signature": {"r":"0x1c929aba1dd2f9cacf5c857e014b2ea1bbd98e5758821a20293b12c869e51732","s":"0x03d739463c57a40e49b8e52f54c18acce5f205ee9ffcee2b96ac83bc3fbcf476"}
        }
        "##;

        let req = serde_json::from_str::<Withdrawal>(json_str);
        assert!(req.is_ok());
        assert_eq!(
            req.unwrap().hash().to_string(),
            "19341009309045799639710615130384553665838605383371747846905355585001394883521"
        )
    }
}
