use crate::{impl_basic, impl_serde};
use primitive_types::U256;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PositionIdType(pub u32);

impl_basic!(PositionIdType, u32);

impl Into<u64> for PositionIdType {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl Into<U256> for PositionIdType {
    fn into(self) -> U256 {
        U256::from(self.0)
    }
}

impl_serde!(PositionIdType, u32, 10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_id_serde() {
        let position_id = PositionIdType(1);
        let serialized = serde_json::to_string(&position_id).unwrap();
        assert_eq!(serialized, "\"1\"");
        let deserialized: PositionIdType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, position_id);
    }
}
