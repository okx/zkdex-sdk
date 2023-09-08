use std::cmp::Ordering;
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use crate::tx::packed_public_key::{fr_to_u256, is_address};
use crate::tx::packed_signature::{get_r_from_xy, get_xy_from_r};
use crate::tx::PackedPublicKey;
use crate::zkw::BabyJubjubPoint;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PublicKeyType(pub BabyJubjubPoint);

impl PublicKeyType {
    pub fn get_y_u256(&self) -> U256 {
        self.0.y.clone()
    }

    pub fn from_xy(x: U256, y: U256) -> Self {
        Self(BabyJubjubPoint { x, y })
    }

    pub fn zero() -> Self {
        Self::from_xy(U256::zero(), U256::one())
    }

    pub fn is_address(&self) -> bool {
        return if self.0.x == U256::zero() && is_address(&(&self.0.y)) {
            true
        } else {
            false
        };
    }
}


impl From<PackedPublicKey> for PublicKeyType {
    fn from(value: PackedPublicKey) -> Self {
        if value.is_address() {
            PublicKeyType(BabyJubjubPoint {
                x: Default::default(),
                y: value.0,
            })
        } else {
            let (x, y) = get_xy_from_r(&value.0);
            let x = fr_to_u256(&x).unwrap();
            let y = fr_to_u256(&y).unwrap();
            PublicKeyType(BabyJubjubPoint { x, y })
        }
    }
}


impl Into<PackedPublicKey> for PublicKeyType {
    fn into(self) -> PackedPublicKey {
        let r = get_r_from_xy(&self.0.x, &self.0.y);
        PackedPublicKey(r)
    }
}


impl Serialize for PublicKeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let x = self.0.x.clone();
        let y = self.0.y.clone();
        PackedPublicKey::from((x, y)).serialize(serializer)
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
        PublicKeyType(BabyJubjubPoint {
            x: U256([
                2527087222397613622,
                6695204439272418284,
                327476452638867716,
                3486998266802970665,
            ]),
            y: U256([0, 0, 0, 0]),
        })
    }
}

impl PartialOrd for PublicKeyType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.x.partial_cmp(&other.0.x) {
            Some(Ordering::Equal) => self.0.y.partial_cmp(&other.0.y),
            Some(ord) => Some(ord),
            None => None,
        }
    }
}

impl Eq for PublicKeyType {}

impl Ord for PublicKeyType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.x.cmp(&other.0.x) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.0.y.cmp(&other.0.y),
        }
    }
}