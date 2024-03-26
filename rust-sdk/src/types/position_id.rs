use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct PositionIdType(pub u32);

impl From<u32> for PositionIdType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Into<u32> for PositionIdType {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<u64> for PositionIdType {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl AsRef<u32> for PositionIdType {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

type SerdeUtils = crate::serde_wrapper::SerdeAsString<10, u32>;

impl Serialize for PositionIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerdeUtils::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for PositionIdType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = SerdeUtils::deserialize(deserializer)?;

        Ok(Self(data))
    }
}
