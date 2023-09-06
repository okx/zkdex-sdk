use std::cmp::Ordering;

use crate::tx::packed_public_key::{fr_to_u256, is_address, PackedPublicKey};
use crate::tx::packed_signature::{get_r_from_xy, get_xy_from_r};
use crate::zkw::BabyJubjubPoint;
use primitive_types::{H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PublicKeyType(pub BabyJubjubPoint);

impl PublicKeyType {
    pub fn get_y_h256(&self) -> H256 {
        u256_to_h256(&self.0.y)
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

pub fn h256_to_u256(h: &H256) -> U256 {
    U256::from_little_endian(&h[..])
}



pub fn u256_to_h256(u: &U256) -> H256 {
    let mut h = [0u8; 32];
    u.to_little_endian(&mut h[..]);
    H256(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TmpSerde {
        v: PublicKeyType,
    }

    #[test]
    fn test_address() {
        let address = r##"{"v":"0x8b6c8fd93d6f4cea42bbb345dbc6f0dfdb5bec73"}"##;
        let key: TmpSerde = serde_json::from_str(address).unwrap();
        let after = u256_to_h256(&key.v.0.y).0;
        println!("{:?}", after);
        assert_eq!(
            after,
            [
                139, 108, 143, 217, 61, 111, 76, 234, 66, 187, 179, 69, 219, 198, 240, 223, 219,
                91, 236, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
    #[test]
    pub fn test_zero() {
        let origin = TmpSerde {
            v: PublicKeyType(PackedPublicKey(U256([0u64; 4])).into()),
        };
        let json_str = serde_json::to_string(&origin).unwrap();
        println!("{:?}", &json_str);
        let key: TmpSerde = serde_json::from_str(json_str.as_str()).unwrap();
        assert_eq!(origin, key);
    }
    #[test]
    pub fn test_address_to_publickey() {
        let key: PublicKeyType = PublicKeyType(
            PackedPublicKey::new_address_public_key(
                "0x8b6c8fd93d6f4cea42bbb345dbc6f0dfdb5bec73".to_string(),
            )
            .into(),
        );
        let temp = TmpSerde { v: key };
        let str = serde_json::to_string(&temp).unwrap();
        println!("{:?}", str);

        let after = serde_json::from_str::<TmpSerde>(str.as_str()).unwrap();
        assert_eq!(temp, after);
    }
}
