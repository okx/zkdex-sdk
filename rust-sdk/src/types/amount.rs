use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct AmountType(pub u128);

impl From<u128> for AmountType {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl Into<u128> for AmountType {
    fn into(self) -> u128 {
        self.0
    }
}

impl AsRef<u128> for AmountType {
    fn as_ref(&self) -> &u128 {
        &self.0
    }
}

impl std::ops::Add<AmountType> for AmountType {
    type Output = Self;

    fn add(self, rhs: AmountType) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<AmountType> for AmountType {
    fn add_assign(&mut self, rhs: AmountType) {
        self.0 += rhs.0
    }
}

impl std::ops::Sub<AmountType> for AmountType {
    type Output = Self;

    fn sub(self, rhs: AmountType) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign<AmountType> for AmountType {
    fn sub_assign(&mut self, rhs: AmountType) {
        self.0 -= rhs.0
    }
}

impl num_traits::ops::checked::CheckedAdd for AmountType {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.0.checked_add(v.0.clone()).map(|v| Self(v))
    }
}

impl num_traits::ops::checked::CheckedSub for AmountType {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.0.checked_sub(v.0.clone()).map(|v| Self(v))
    }
}

impl num_traits::Zero for AmountType {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

type SerdeUtils = crate::serde_wrapper::SerdeAsString<10, u128>;

impl Serialize for AmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerdeUtils::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for AmountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = SerdeUtils::deserialize(deserializer)?;

        Ok(Self(data))
    }
}
