use std::cmp::Ordering;

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::tx::packed_public_key::is_address;
use crate::tx::PackedPublicKey;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PublicKeyType(pub U256);

impl PublicKeyType {
    pub fn new(packed: U256) -> Self {
        Self(packed)
    }

    pub fn get_y(&self) -> U256 {
        let mut y = self.0.clone();
        y.0[3] &= 0x7FFFFFFFFFFFFFFF;

        y
    }

    pub fn get_packed(&self) -> &U256 {
        &self.0
    }

    pub fn zero() -> Self {
        Self(U256::zero())
    }

    pub fn is_address(&self) -> bool {
        return if is_address(&(&self.0)) { true } else { false };
    }
}
impl Into<U256> for PublicKeyType {
    fn into(self) -> U256 {
        self.0
    }
}

impl From<PackedPublicKey> for PublicKeyType {
    fn from(value: PackedPublicKey) -> Self {
        Self(value.0)
    }
}

impl Into<PackedPublicKey> for PublicKeyType {
    fn into(self) -> PackedPublicKey {
        PackedPublicKey(self.0)
    }
}

impl Serialize for PublicKeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let p: PackedPublicKey = self.clone().into();
        p.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicKeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let key = PackedPublicKey::deserialize(deserializer)?;
        let ret = PublicKeyType::from(key);
        Ok(ret)
    }
}
impl Default for PublicKeyType {
    fn default() -> Self {
        PublicKeyType(U256::zero())
    }
}
impl PartialOrd for PublicKeyType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for PublicKeyType {}

impl Ord for PublicKeyType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
