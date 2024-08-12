pub mod amount;
pub mod asset_id;
pub mod position_id;

use primitive_types::U256;

pub type AssetIdType = i128;
pub type CollateralAssetId = U256;
pub type SignedAssetId = U256;
pub type PositionIdType = u64;
pub type PriceType = u128;
pub type AmountType = u64;
