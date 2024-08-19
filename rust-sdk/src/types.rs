use crate::Engine;
use franklin_crypto::alt_babyjubjub::JubjubEngine;
use primitive_types::U256;

pub type HashType = U256;

pub type NonceType = u32;

pub type TimestampType = u32;

pub type PositionIdType = u64;
pub type ResolutionType = u64;
pub type RiskFactorType = u64;
pub type OraclePriceQuorumType = u64;
pub type FundingRateType = i128;

pub type Fs = <Engine as JubjubEngine>::Fs;
