use primitive_types::U256;

use crate::zkw::JubjubSignature;

pub type AssetIdType = i128;
pub type CollateralAssetId = U256;
pub type SignedAssetId = U256;
pub type PositionIdType = u64;
pub type OrderIdType = u64;
pub type HashType = U256;
pub type IndexType = i128;
pub type ResolutionType = u64;
pub type RiskFactorType = u64;
pub type OraclePriceQuorumType = u64;
pub type TimestampType = u32;
pub type TreeHeightType = u32;
pub type FundingRateType = i128;
pub type PriceType = u128;
pub type AmountType = u64;
pub type BalanceType = i64;
pub type ConditionType = u64;
pub type NonceType = u32;
pub type SpotAmountType = u128;
pub type SpotAssetIdType = u32;
pub type SpotPositionIdType = u32;
pub type SignatureType = JubjubSignature;

#[repr(u16)]
#[derive(Debug)]
pub enum TransactionType {
    Deposit = 0,
    ForcedTrade = 1,
    ForcedWithdrawal = 2,
    FundingTick = 3,
    OraclePricesTick = 4,
    Trade = 5,
    Transfer = 6,
    Liquidate = 7,
    Withdrawal = 8,
    Deleverage = 9,
    ConditionalTransfer = 10,
    MultiTransaction = 11,
    RawOraclePricesTick = 12,
    SpotDeposit = 50,
    SpotWithdrawal = 51,
    SpotTrade = 52,
    SpotTransfer = 53,
    SpotForcedWithdrawal = 54,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Deposit
    }
}
