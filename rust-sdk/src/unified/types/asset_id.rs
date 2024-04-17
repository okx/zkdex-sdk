use crate::{impl_basic, impl_ord, impl_serde};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct AssetIdType(pub u32);

impl_basic!(AssetIdType, u32);

impl_ord!(AssetIdType, u32);

impl_serde!(AssetIdType, u32, 16);

impl Into<u64> for AssetIdType {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl std::fmt::LowerHex for AssetIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_id_serde() {
        let asset_id = AssetIdType(0x1234);
        let encoded = serde_json::to_string(&asset_id).unwrap();
        assert_eq!(encoded, "\"0x1234\"");
        let decoded: AssetIdType = serde_json::from_str(&encoded).unwrap();
        assert_eq!(asset_id, decoded);
    }
}
