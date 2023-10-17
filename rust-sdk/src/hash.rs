use primitive_types::U256;

use crate::tx::public_key_type::PublicKeyType;

pub trait Hasher {
    fn update_single<T: ToHashable>(&mut self, _data: &T);
    fn update<T: ToHashable>(&mut self, data: &[T]) {
        self.update_single(&(data.len() as u64));
        for d in data {
            self.update_single(d);
        }
    }
    fn finalize(&mut self) -> U256;
}

pub fn new_hasher() -> impl Hasher {
    return zkw::Poseidon::new();
}

mod zkw {
    use crate::hash::Hasher;
    use crate::zkw::PoseidonHasher;

    use super::*;

    pub struct Poseidon {
        poseidon: PoseidonHasher,
    }

    impl Poseidon {
        pub fn new() -> Self {
            Self {
                poseidon: PoseidonHasher::new(),
            }
        }
    }

    impl Hasher for Poseidon {
        fn update_single<T: ToHashable>(&mut self, data: &T) {
            let d = data.to_hashable();
            assert!(d.len() <= 4);
            if d.len() == 4 {
                self.poseidon.update(d[0]);
                self.poseidon.update(d[1]);
                self.poseidon.update(d[2]);
                self.poseidon.update(d[3] & (u64::MAX << 1 >> 1)); // for packed public key
            } else {
                for u in d {
                    self.poseidon.update(*u);
                }
                for _ in 0..4 - d.len() {
                    self.poseidon.update(0);
                }
            }
        }

        fn finalize(&mut self) -> U256 {
            U256(self.poseidon.finalize())
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
        self.0.to_hashable()
    }
}

pub fn hash2<T1: ToHashable, T2: ToHashable>(a: &T1, b: &T2) -> U256 {
    let mut hasher = new_hasher();
    hasher.update_single(a);
    hasher.update_single(b);
    hasher.finalize()
}

#[cfg(test)]
mod test {
    use std::thread::{JoinHandle, spawn};

    use primitive_types::U256;

    use crate::hash::{hash2, new_hasher};

    use super::Hasher;

    #[test]
    fn test_concurrent_hash() {
        let mut handler: Vec<JoinHandle<()>> = Vec::new();
        let hash = hash2(&U256::from(1), &U256::from(2));
        for _ in 0..200 {
            let hash = hash.clone();
            let t = spawn(move || {
                let hash1 = hash2(&U256::from(1), &U256::from(2));
                assert!(hash1 == hash)
            });

            handler.push(t);
        }

        for x in handler {
            _ = x.join();
        }
    }

    #[test]
    fn test_hash_1() {
        let mut hasher = new_hasher();
        let a = hasher.finalize();
        println!("{}",a.to_string());
        let hash = hash2(&U256::from(1), &U256::from(2));
        let str =  hash.to_string();
        println!("{}", str.clone());
        assert!("12161893061466977591326716549227327416251121218164330599584971528678000121369" == str);
    }
}
