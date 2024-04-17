#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct AssetIdType(pub u32);

impl From<u32> for AssetIdType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Into<u32> for AssetIdType {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<u64> for AssetIdType {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl AsRef<u32> for AssetIdType {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    type SerdeUtils = crate::serde_wrapper::SerdeAsString<16, u32>;

    impl Serialize for AssetIdType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            SerdeUtils::serialize(&self.0, serializer)
        }
    }

    impl<'de> Deserialize<'de> for AssetIdType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let data = SerdeUtils::deserialize(deserializer)?;

            Ok(Self(data))
        }
    }
}
