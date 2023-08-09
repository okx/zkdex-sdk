use crate::new_public_key::PublicKeyType;
use primitive_types::{H256, U256};

// pub use self::poseidon::poseidon_push;

// pub use self::poseidon::poseidon_new;

// pub use self::poseidon::poseidon_finalize;

pub trait Hasher {
    fn update_single<T: ToHashable>(&mut self, _data: &T);
    fn update<T: ToHashable>(&mut self, data: &[T]) {
        self.update_single(&(data.len() as u64));
        for d in data {
            self.update_single(d);
        }
    }
    fn finalize(&mut self) -> H256;
}

pub fn new_hasher() -> impl Hasher {
    return zkw::Poseidon::new();
}

mod zkw {
    use super::*;
    use crate::new_public_key::u256_to_h256;
    use crate::zkw::PoseidonHasher;

    pub struct Poseidon {
        poseidon: PoseidonHasher,
        index: u8,
    }

    impl Poseidon {
        pub fn new() -> Self {
            Self {
                poseidon: PoseidonHasher::new(),
                index: 0,
            }
        }
    }

    impl Hasher for Poseidon {
        fn update_single<T: ToHashable>(&mut self, data: &T) {
            let mut d = data.to_hashable();

            let first_take = (3 - self.index) as usize;

            if self.index != 0 {
                d.iter()
                    .take(first_take)
                    .for_each(|i| self.poseidon.update(*i));

                if d.len() <= first_take {
                    self.index += d.len() as u8;
                    return;
                }

                self.index = 0;
                d = &d[first_take..];
            }

            for c in d.chunks(3) {
                self.poseidon.update(0);
                for i in c {
                    self.poseidon.update(*i);
                }

                // latest chunk
                if c.len() != 3 {
                    self.index = c.len() as u8;
                }
            }
        }

        fn finalize(&mut self) -> H256 {
            u256_to_h256(&U256(self.poseidon.finalize()))
        }
    }
}

pub trait ToHashable {
    fn to_hashable(&self) -> &[u64];
}

impl ToHashable for i128 {
    fn to_hashable(&self) -> &[u64] {
        unsafe { std::slice::from_raw_parts(self as *const i128 as *const u64, 2) }
    }
}

impl ToHashable for u128 {
    fn to_hashable(&self) -> &[u64] {
        unsafe { std::slice::from_raw_parts(self as *const u128 as *const u64, 2) }
    }
}

impl ToHashable for u64 {
    fn to_hashable(&self) -> &[u64] {
        unsafe { std::slice::from_raw_parts(self as *const u64, 1) }
    }
}

impl ToHashable for i64 {
    fn to_hashable(&self) -> &[u64] {
        unsafe { std::slice::from_raw_parts(self as *const i64 as *const u64, 1) }
    }
}

impl ToHashable for U256 {
    fn to_hashable(&self) -> &[u64] {
        &self.0
    }
}

impl ToHashable for PublicKeyType {
    fn to_hashable(&self) -> &[u64] {
        self.0.y.to_hashable()
        // let array = self as *const PublicKeyType as *const u64 as *const [u64; 8];
        // unsafe { &*array }
    }
}

impl ToHashable for H256 {
    fn to_hashable(&self) -> &[u64] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u64, 4) }
    }
}

pub fn hash2<T1: ToHashable, T2: ToHashable>(a: &T1, b: &T2) -> H256 {
    let mut hasher = new_hasher();
    hasher.update_single(a);
    hasher.update_single(b);
    hasher.finalize()
}

#[cfg(test)]
mod test {
    use crate::hash::{hash2, ToHashable};
    use crate::new_public_key::PublicKeyType;
    use crate::zkw::BabyJubjubPoint;
    use primitive_types::U256;

    #[test]
    fn test_to_hash() {
        let pk = PublicKeyType {
            0: BabyJubjubPoint {
                x: U256::from(u64::MAX),
                y: U256::from(u64::MAX) << 192,
            },
        };

        let bz = pk.to_hashable();
        assert_eq!(bz.len(), 4);
        assert_eq!(bz[0], 0);
        assert_eq!(bz[1], 0u64);
        assert_eq!(bz[2], 0u64);
        assert_eq!(bz[3], u64::MAX);
    }

    #[test]
    fn test_hash() {

    }
}