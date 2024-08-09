use crate::{impl_basic, impl_serde};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ChainIdType(pub u32);

impl_basic!(ChainIdType, u32);

impl Into<u64> for ChainIdType {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl_serde!(ChainIdType, u32, 10);
