#[macro_export]
macro_rules! impl_basic {
    ($name:ty, $inner_type:ty) => {
        impl From<$inner_type> for $name {
            fn from(value: $inner_type) -> Self {
                Self(value)
            }
        }

        impl Into<$inner_type> for $name {
            fn into(self) -> $inner_type {
                self.0
            }
        }

        impl AsRef<$inner_type> for $name {
            fn as_ref(&self) -> &$inner_type {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ord {
    ($name:ty, $inner_type:ty) => {
        impl PartialEq<$inner_type> for $name {
            fn eq(&self, other: &$inner_type) -> bool {
                self.0.eq(other)
            }
        }

        impl PartialOrd<$inner_type> for $name {
            fn partial_cmp(&self, other: &$inner_type) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_zero {
    ($name:ty) => {
        impl num_traits::Zero for $name {
            fn zero() -> Self {
                Self(0)
            }

            fn is_zero(&self) -> bool {
                self.0 == 0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_math {
    ($name:ty, $inner_type:ty) => {
        impl std::ops::Add<$name> for $name {
            type Output = Self;

            fn add(self, rhs: $name) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl std::ops::Add<&$name> for &$name {
            type Output = $name;

            fn add(self, rhs: &$name) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign<$name> for $name {
            fn add_assign(&mut self, rhs: $name) {
                self.0 += rhs.0
            }
        }

        impl std::ops::Sub<$name> for $name {
            type Output = Self;

            fn sub(self, rhs: $name) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl std::ops::Sub<&$name> for &$name {
            type Output = $name;

            fn sub(self, rhs: &$name) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl std::ops::SubAssign<$name> for $name {
            fn sub_assign(&mut self, rhs: $name) {
                self.0 -= rhs.0
            }
        }

        impl num_traits::ops::checked::CheckedAdd for $name {
            fn checked_add(&self, v: &Self) -> Option<Self> {
                self.0.checked_add(v.0.clone()).map(|v| Self(v))
            }
        }

        impl num_traits::ops::checked::CheckedSub for $name {
            fn checked_sub(&self, v: &Self) -> Option<Self> {
                self.0.checked_sub(v.0.clone()).map(|v| Self(v))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_serde {
    ($name:ty, $inner_type:ty, $radix:expr) => {
        mod serde_impl {
            use super::*;
            use serde::{Deserializer, Serializer};

            type SerdeUtils = crate::serde_wrapper::SerdeAsString<$radix, $inner_type>;

            impl serde::Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    SerdeUtils::serialize(&self.0, serializer)
                }
            }

            impl<'de> serde::Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let data = SerdeUtils::deserialize(deserializer)?;

                    Ok(Self(data))
                }
            }
        }
    };
}
