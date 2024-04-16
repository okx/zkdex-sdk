use primitive_types::U256;

pub trait HashTrait{
    fn hash(&self) -> U256;
}