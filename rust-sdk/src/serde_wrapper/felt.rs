use primitive_types::U256;

pub struct Felt<'a, T> {
    inner: &'a T,
}

impl<'a, T> Felt<'a, T> {
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }
}

impl<'a, T> IntoIterator for Felt<'a, T>
where
    Felt<'a, T>: Into<&'a [u64]>,
{
    type Item = u64;
    type IntoIter = FeltIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let data: &[u64] = self.into();
        // host::assert(data.len() <= 4);
        FeltIter { data, idx: 0 }
    }
}

pub struct FeltIter<'a> {
    data: &'a [u64],
    idx: usize,
}

impl<'a> Iterator for FeltIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.data.len() {
            let d = self.data[self.idx];
            self.idx += 1;
            Some(d)
        } else if self.idx < 4 {
            self.idx += 1;
            Some(0)
        } else {
            None
        }
    }
}

macro_rules! impl_felt_into_u64_slice {
   (
       $($ty:ty),*
   ) => {
       $(
       impl<'a> Into<&'a [u64]> for Felt<'a, $ty> {
           fn into(self) -> &'a [u64] {
               static_assertions::const_assert!(std::mem::size_of::<$ty>() >= std::mem::size_of::<u64>());
               unsafe { std::slice::from_raw_parts(self.inner as *const $ty as *const u64, std::mem::size_of::<$ty>() / std::mem::size_of::<u64>()) }
           }
       }
       )*
    };
}

macro_rules! impl_felt_into_u8_array {
   (
       $($ty:ty),*
   ) => {
       $(
       impl<'a> Into<&'a [u8; std::mem::size_of::<$ty>()]> for Felt<'a, $ty> {
           fn into(self) -> &'a [u8; std::mem::size_of::<$ty>()] {
               unsafe { &*(self.inner as *const $ty as *const [u8; std::mem::size_of::<$ty>()]) }
           }
       }
       )*
    };
}

impl_felt_into_u64_slice!(u64, i64, i128, u128, U256);

impl_felt_into_u8_array!(u8, u16, u32, u64, i64, i128, u128, U256);

pub trait LeBytesConvert<const N: usize> {
    fn as_le_array(&self) -> &[u8; N];

    fn as_le_bytes(&self) -> &[u8] {
        self.as_le_array().as_slice()
    }
}

macro_rules! impl_as_le_array_ref {
   (
       $($ty:ty),*
   ) => {
       $(
       impl LeBytesConvert<{std::mem::size_of::<$ty>()}> for $ty {
           fn as_le_array(&self) -> &[u8; std::mem::size_of::<$ty>()] {
               unsafe { &*(self as *const $ty as *const [u8; std::mem::size_of::<$ty>()]) }
           }
       }
       )*
    };
}

impl_as_le_array_ref!(u8, u16, u32, u64, i64, i128, u128, U256);

// impl LeBytesConvert<32> for PublicKeyType {
//     fn as_le_array(&self) -> &[u8; 32] {
//         self.0.y.as_le_array()
//     }
// }

#[cfg(test)]
mod tests {
    use primitive_types::U256;

    use crate::felt::Felt;
    use crate::felt::LeBytesConvert;

    #[test]
    fn test_felt() {
        let data = 1u64;
        let felt = Felt::new(&data);

        let mut dst = vec![];
        for d in felt.into_iter() {
            dst.push(d);
        }

        assert_eq!(dst, vec![1, 0, 0, 0]);

        let data = U256::one();
        let felt = Felt::new(&data);

        let mut expect = [0u8; 32];
        expect[0] = 1;

        assert_eq!(&expect, to_array(felt));

        let u8_arr = data.as_le_array();

        assert_eq!(&expect, u8_arr);
    }

    fn to_array<'a, T: Into<&'a [u8; N]>, const N: usize>(t: T) -> &'a [u8; N] {
        t.into()
    }
}
