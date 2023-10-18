use std::ops::ShlAssign;

use anyhow::Result;
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::common::OrderBase;
use crate::felt::LeBytesConvert;
use crate::hash;
use crate::hash::Hasher;
use crate::serde_wrapper::I128SerdeAsRadix16Prefix0xString;
use crate::serde_wrapper::U256SerdeAsRadix16Prefix0xString;
use crate::serde_wrapper::U64SerdeAsString;
use crate::transaction::types::{
    AmountType, AssetIdType, CollateralAssetId, HashType, PositionIdType,
};
use crate::tx::packed_public_key::private_key_from_string;
use crate::tx::TxSignature;
use crate::zkw::JubjubSignature;

const LIMIT_ORDER_WITH_FEES: u64 = 3;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct LimitOrderRequest {
    #[serde(flatten)]
    pub base: OrderBase,
    #[serde(rename = "amount_synthetic", with = "U64SerdeAsString")]
    pub amount_synthetic: AmountType,
    #[serde(rename = "amount_collateral", with = "U64SerdeAsString")]
    pub amount_collateral: AmountType,
    #[serde(rename = "amount_fee", with = "U64SerdeAsString")]
    pub amount_fee: AmountType,
    #[serde(
        rename = "asset_id_synthetic",
        with = "I128SerdeAsRadix16Prefix0xString"
    )]
    pub asset_id_synthetic: AssetIdType,
    #[serde(
        rename = "asset_id_collateral",
        with = "U256SerdeAsRadix16Prefix0xString"
    )]
    pub asset_id_collateral: CollateralAssetId,
    #[serde(rename = "position_id", with = "U64SerdeAsString")]
    pub position_id: PositionIdType,
    #[serde(rename = "is_buying_synthetic")]
    pub is_buying_synthetic: bool,
}

pub fn sign_limit_order(req: LimitOrderRequest, prvk: &str) -> Result<JubjubSignature> {
    let hash = hash_limit_order(req);
    let private_key = private_key_from_string(prvk)?;
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

pub fn hash_limit_order(req: LimitOrderRequest) -> HashType {
    let exchange_limit_order = &mut ExchangeLimitOrder::default();
    exchange_limit_order.base = &req.base;
    exchange_limit_order.amount_fee = req.amount_fee;
    exchange_limit_order.asset_id_fee = req.asset_id_collateral;
    exchange_limit_order.vault_buy = req.position_id;
    exchange_limit_order.vault_sell = req.position_id;
    exchange_limit_order.vault_fee = req.position_id;

    if req.is_buying_synthetic {
        exchange_limit_order.asset_id_sell = req.asset_id_collateral;
        exchange_limit_order.asset_id_buy = U256::from(req.asset_id_synthetic);
        exchange_limit_order.amount_sell = req.amount_collateral;
        exchange_limit_order.amount_buy = req.amount_synthetic;
    } else {
        exchange_limit_order.asset_id_sell = U256::from(req.asset_id_synthetic);
        exchange_limit_order.asset_id_buy = req.asset_id_collateral;
        exchange_limit_order.amount_sell = req.amount_synthetic;
        exchange_limit_order.amount_buy = req.amount_collateral;
    }

    let hash = exchange_limit_order.hash();
    hash
}

#[derive(Default)]
pub struct LimitOrder {
    pub base: OrderBase,
    pub amount_buy: AmountType,
    pub amount_sell: AmountType,
    pub amount_fee: AmountType,
    pub asset_id_buy: CollateralAssetId,
    pub asset_id_sell: CollateralAssetId,
    pub asset_id_fee: CollateralAssetId,
    pub vault_buy: PositionIdType,
    pub vault_sell: PositionIdType,
    pub vault_fee: PositionIdType,
}

pub struct ExchangeLimitOrder {
    pub base: *const OrderBase,
    pub amount_buy: AmountType,
    pub amount_sell: AmountType,
    pub amount_fee: AmountType,
    pub asset_id_buy: CollateralAssetId,
    pub asset_id_sell: CollateralAssetId,
    pub asset_id_fee: CollateralAssetId,
    pub vault_buy: PositionIdType,
    pub vault_sell: PositionIdType,
    pub vault_fee: PositionIdType,
}

pub fn limit_order_hash_internal(limit_order: &ExchangeLimitOrder) -> HashType {
    // let (msg) = hash2{hash_ptr=pedersen_ptr}(
    //     x=limit_order.asset_id_sell, y=limit_order.asset_id_buy
    // );
    // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=msg, y=limit_order.asset_id_fee);
    let mut hasher = hash::new_hasher();
    hasher.update_single(&limit_order.asset_id_sell);
    hasher.update_single(&limit_order.asset_id_buy);
    hasher.update_single(&limit_order.asset_id_fee);

    // let packed_message0 = limit_order.amount_sell;
    // let packed_message0 = packed_message0 * AMOUNT_UPPER_BOUND + limit_order.amount_buy;
    // let packed_message0 = packed_message0 * AMOUNT_UPPER_BOUND + limit_order.amount_fee;

    let mut packed_message0 = U256([
        limit_order.amount_fee,
        limit_order.amount_buy,
        limit_order.amount_sell,
        0,
    ]);

    let limit_order_base = unsafe { &*limit_order.base };

    // let packed_message0 = packed_message0 * NONCE_UPPER_BOUND + limit_order.base.nonce;
    packed_message0.shl_assign(32);
    packed_message0.0[0] += limit_order_base.nonce as u64;

    // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=msg, y=packed_message0);
    hasher.update_single(&packed_message0);

    // let packed_message1 = LIMIT_ORDER_WITH_FEES;
    // let packed_message1 = packed_message1 * VAULT_ID_UPPER_BOUND + limit_order.vault_fee;
    // let packed_message1 = packed_message1 * VAULT_ID_UPPER_BOUND + limit_order.vault_sell;
    // let packed_message1 = packed_message1 * VAULT_ID_UPPER_BOUND + limit_order.vault_buy;

    let mut packed_message1 = U256([
        limit_order.vault_buy,
        limit_order.vault_sell,
        limit_order.vault_fee,
        LIMIT_ORDER_WITH_FEES,
    ]);

    // let packed_message1 = packed_message1 * EXPIRATION_TIMESTAMP_UPPER_BOUND + limit_order.base.expiration_timestamp;
    packed_message1.shl_assign(32);
    packed_message1.0[0] += limit_order_base.expiration_timestamp as u64;

    // let packed_message1 = packed_message1 * (2 ** 17);  // Padding.
    let packed_message1 = packed_message1 << 17; // Padding.

    // let (msg) = hash2{hash_ptr=pedersen_ptr}(x=msg, y=packed_message1);
    hasher.update_single(&packed_message1);

    hasher.finalize()
}

impl ExchangeLimitOrder {
    pub fn hash(&self) -> HashType {
        limit_order_hash_internal(self)
    }

    pub const fn const_default() -> Self {
        ExchangeLimitOrder {
            base: std::ptr::null(),
            amount_buy: 0,
            amount_sell: 0,
            amount_fee: 0,
            asset_id_buy: U256([0; 4]),
            asset_id_sell: U256([0; 4]),
            asset_id_fee: U256([0; 4]),
            vault_buy: 0,
            vault_sell: 0,
            vault_fee: 0,
        }
    }
}

impl Default for ExchangeLimitOrder {
    fn default() -> Self {
        Self::const_default()
    }
}

#[cfg(test)]
mod test {
    use crate::common::OrderBase;
    use crate::transaction::types::CollateralAssetId;
    use crate::tx::{LimitOrderRequest, private_key_from_string, public_key_from_private};
    use crate::tx::limit_order::sign_limit_order;
    use crate::tx::public_key_type::PublicKeyType;

    #[test]
    pub fn test_sign() {
        let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        let private_key = private_key_from_string(prv_key).unwrap();
        let pub_key = public_key_from_private(&private_key);
        let expire = 2;
        let pub_key = PublicKeyType::from(pub_key.clone());
        println!("{}", serde_json::to_string(&pub_key.clone()).unwrap());

        let req = LimitOrderRequest {
            base: OrderBase {
                nonce: 1,
                public_key: pub_key,
                expiration_timestamp: expire,
            },
            amount_synthetic: 3,
            amount_collateral: 4,
            amount_fee: 5,
            asset_id_synthetic: 6,
            asset_id_collateral: CollateralAssetId::from(7),
            position_id: 8,
            is_buying_synthetic: false,
        };

        let w = sign_limit_order(req, prv_key).unwrap();
        println!("{:?}", w);
    }

    #[cfg(test)]
    mod test {
        use std::str::FromStr;

        use franklin_crypto::alt_babyjubjub::FixedGenerators;
        use franklin_crypto::eddsa::PublicKey;

        use crate::felt::LeBytesConvert;
        use crate::tx::{
            HashType, JUBJUB_PARAMS, LimitOrderRequest, private_key_from_string, TxSignature,
        };

        #[test]
        pub fn test_sign2() {
            let hash = HashType::from_str(
                "0x1ca9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3",
            )
            .unwrap();
            let hash1 = HashType::from_str(
                "0x15a9d875223bda3a766a587f3b338fb372b2250e6add5cc3d6067f6ad5fce4f3",
            )
            .unwrap();
            println!("{:?}", hash.clone());
            let prv_key = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
            let private_key = private_key_from_string(prv_key).unwrap();
            let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());

            let pub_key = PublicKey::from_private(
                &private_key,
                FixedGenerators::SpendingKeyGenerator,
                &JUBJUB_PARAMS,
            );
            assert!(sig.verify(&pub_key, hash.as_le_bytes()));
            assert!(!sig.verify(&pub_key, hash1.as_le_bytes()));
        }

        #[test]
        fn test_deserialize() {
            let json = r#"
    {
  "nonce": "1",
  "public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
  "expiration_timestamp": "2",
  "amount_synthetic": "3",
  "amount_collateral": "4",
  "amount_fee": "5",
  "asset_id_synthetic": "6",
  "asset_id_collateral": "0xa",
  "position_id": "8",
  "is_buying_synthetic": false
    }
   "#;

            let ret = serde_json::from_str::<LimitOrderRequest>(json);
            assert!(ret.is_ok());
            println!("{:?}", ret.unwrap())
        }
    }
}
