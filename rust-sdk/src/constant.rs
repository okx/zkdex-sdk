use num_bigint::BigInt;
use primitive_types::U256;

// This is the lower bound for actual synthetic asset and limit order collateral amounts. Those
// amounts can't be 0 to prevent order replay and arbitrary actual fees.
#[allow(dead_code)]
pub const POSITIVE_AMOUNT_LOWER_BOUND: u64 = 1;

// ASSET_ID_UPPER_BOUND is set so that PositionAsset could be packed into a field element.
// const ASSET_ID_UPPER_BOUND = 2 ** 120;
#[allow(dead_code)]
pub const ASSET_ID_UPPER_BOUND: i128 = 1 << 120;

// A valid balance satisfies BALANCE_LOWER_BOUND < balance < BALANCE_UPPER_BOUND.
// const BALANCE_UPPER_BOUND = 2 ** 63;
#[allow(dead_code)]
pub const BALANCE_UPPER_BOUND: i128 = 1 << 63;
#[allow(dead_code)]
pub const BALANCE_LOWER_BOUND: i128 = -BALANCE_UPPER_BOUND;

// const TOTAL_VALUE_UPPER_BOUND = 2 ** 63;
#[allow(dead_code)]
pub const TOTAL_VALUE_UPPER_BOUND: u64 = 1 << 63;
// const TOTAL_VALUE_LOWER_BOUND = -(2 ** 63);
#[allow(dead_code)]
pub const TOTAL_VALUE_LOWER_BOUND: i128 = -(TOTAL_VALUE_UPPER_BOUND as i128);

// const TOTAL_RISK_UPPER_BOUND = 2 ** 64;
#[allow(dead_code)]
pub const TOTAL_RISK_UPPER_BOUND: u128 = 1 << 64;

// const N_ASSETS_UPPER_BOUND = 2 ** 16;
#[allow(dead_code)]
pub const N_ASSETS_UPPER_BOUND: u32 = 1 << 16;

// const POSITION_MAX_SUPPORTED_N_ASSETS = 2 ** 6;
#[allow(dead_code)]
pub const POSITION_MAX_SUPPORTED_N_ASSETS: u64 = 1 << 6;

// Fixed point (.32) representation of the number 1.
// const FXP_32_ONE = 2 ** 32;
#[allow(dead_code)]
pub const FXP_32_ONE: u64 = 1 << 32;
// Oracle prices are signed by external entities, which use a fixed point representation where
// 10**18 is 1.0 .
// const EXTERNAL_PRICE_FIXED_POINT_UNIT = 10 ** 18;
#[allow(dead_code)]
pub const EXTERNAL_PRICE_FIXED_POINT_UNIT: u64 = 1_000_000_000_000_000_000;

// const ORACLE_PRICE_QUORUM_LOWER_BOUND = 1;
#[allow(dead_code)]
pub const ORACLE_PRICE_QUORUM_LOWER_BOUND: u64 = 1;
// const ORACLE_PRICE_QUORUM_UPPER_BOUND = 2 ** 32;
#[allow(dead_code)]
pub const ORACLE_PRICE_QUORUM_UPPER_BOUND: u64 = 1 << 32;

// const POSITION_ID_UPPER_BOUND = 2 ** 64;
#[allow(dead_code)]
pub const POSITION_ID_UPPER_BOUND: u128 = 1 << 64;
pub const POSITION_ID_UPPER_BOUND_U256: U256 = U256([0, 1, 0, 0]);
// const ORDER_ID_UPPER_BOUND = 2 ** 64;
#[allow(dead_code)]
pub const ORDER_ID_UPPER_BOUND: u128 = 1 << 64;
// Fixed point (32.32).
// const FUNDING_INDEX_UPPER_BOUND = 2 ** 63;
#[allow(dead_code)]
pub const FUNDING_INDEX_UPPER_BOUND: u64 = 1 << 63;
// const FUNDING_INDEX_LOWER_BOUND = -(2 ** 63);
#[allow(dead_code)]
pub const FUNDING_INDEX_LOWER_BOUND: i128 = -(FUNDING_INDEX_UPPER_BOUND as i128);

// Fixed point (0.32).
// const RISK_FACTOR_LOWER_BOUND = 1;
#[allow(dead_code)]
pub const RISK_FACTOR_LOWER_BOUND: u64 = 1;
// const RISK_FACTOR_UPPER_BOUND = FXP_32_ONE;
#[allow(dead_code)]
pub const RISK_FACTOR_UPPER_BOUND: u64 = FXP_32_ONE;

// Fixed point (32.32).
// const PRICE_LOWER_BOUND = 1;
#[allow(dead_code)]
pub const PRICE_LOWER_BOUND: u128 = 1;
// const PRICE_UPPER_BOUND = 2 ** 64;
#[allow(dead_code)]
pub const PRICE_UPPER_BOUND: u128 = 1 << 64;

// const EXTERNAL_PRICE_UPPER_BOUND = 2 ** 120;
#[allow(dead_code)]
pub const EXTERNAL_PRICE_UPPER_BOUND: u128 = 1 << 120;

// const ASSET_RESOLUTION_LOWER_BOUND = 1;
#[allow(dead_code)]
pub const ASSET_RESOLUTION_LOWER_BOUND: u64 = 1;
// const ASSET_RESOLUTION_UPPER_BOUND = 2 ** 64;
#[allow(dead_code)]
pub const ASSET_RESOLUTION_UPPER_BOUND: i128 = 1 << 64;

// const COLLATERAL_ASSET_ID_UPPER_BOUND = 2 ** 250;
#[allow(dead_code)]
pub const COLLATERAL_ASSET_ID_UPPER_BOUND: U256 = U256([0, 0, 0, 288230376151711744]);

// General Cairo constants.
// const SIGNED_MESSAGE_BOUND = 2 ** 251;
#[allow(dead_code)]
pub const SIGNED_MESSAGE_BOUND: U256 = U256([0, 0, 0, 576460752303423488]);
// const RANGE_CHECK_BOUND = 2 ** 128;
#[allow(dead_code)]
pub const RANGE_CHECK_BOUND: U256 = U256([0, 0, 1, 0]);
pub const RANGE_CHECK_BOUND_BYTES_LE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// PRIME = 2**251 + 17 * 2**192 + 1
#[allow(dead_code)]
pub const PRIME: U256 = U256([1, 0, 0, 576460752303423505]); // 0x0800000000000011000000000000000000000000000000000000000000000001
pub const PRIME_BYTES_LE: [u8; 32] = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 8,
];

#[allow(dead_code)]
pub fn get_prime_as_bigint() -> BigInt {
    BigInt::from_bytes_le(num_bigint::Sign::Plus, &PRIME_BYTES_LE)
}

#[allow(dead_code)]
pub fn get_range_check_bound_as_bigint() -> BigInt {
    BigInt::from_bytes_le(num_bigint::Sign::Plus, &RANGE_CHECK_BOUND_BYTES_LE)
}

pub const AMOUNT_UPPER_BOUND_U256: U256 = U256([0, 1, 0, 0]);
pub const EXPIRATION_TIMESTAMP_UPPER_BOUND_U256: U256 = U256([4294967296, 0, 0, 0]);
pub const NONCE_UPPER_BOUND_U256: U256 = U256([4294967296, 0, 0, 0]);

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use num_traits::One;
    use primitive_types::U256;

    #[test]
    fn test_constants() {
        let num = U256::one() << 250;
        assert_eq!(COLLATERAL_ASSET_ID_UPPER_BOUND, num);

        let num = U256::one() << 251;
        assert_eq!(SIGNED_MESSAGE_BOUND, num);

        let num = U256::one() << 128;
        assert_eq!(RANGE_CHECK_BOUND, num);

        let num = get_range_check_bound_as_bigint();
        assert_eq!(num, BigInt::from(2).pow(128));

        let num = (U256::one() << 251) + U256::from(17) * (U256::one() << 192) + U256::one();
        assert_eq!(PRIME, num);
        let num = get_prime_as_bigint();
        assert_eq!(
            num,
            BigInt::from(2).pow(251) + BigInt::from(17) * BigInt::from(2).pow(192) + BigInt::one()
        );

        assert_eq!(
            POSITION_ID_UPPER_BOUND_U256,
            U256::from(POSITION_ID_UPPER_BOUND)
        );
    }
}
