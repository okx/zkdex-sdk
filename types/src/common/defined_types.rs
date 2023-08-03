// use num_bigint::BigInt;
// use primitive_types::H256;
// use std::fmt::{Debug, Formatter};
// use zksync::zksync_types::tx::PackedSignature;
//
// pub type AssetIdType = BigInt;
// pub type PositionId = u64;
// pub type OrderId = u64;
// pub type HashType = H256;
// pub type PrivateKeyType = String;
// pub type IndexType = i128;
//
// #[derive(Clone)]
// pub struct SignatureType(pub PackedSignature);
//
// impl SignatureType {
//     pub fn serialize_packed(&self) -> std::io::Result<Vec<u8>> {
//         self.0.serialize_packed()
//     }
//
//     pub fn deserialize_packed(bytes: &[u8]) -> Result<Self, Error> {
//         // TODO: zksync::zksync_types::tx isn't export DeserializeError,
//         // so we have to unwrap result of deserialize_packed and
//         // define our error for temporary。
//         Ok(Self(PackedSignature::deserialize_packed(bytes).unwrap()))
//     }
//
//     pub fn default_value() -> Self {
//         SignatureType::deserialize_packed(
//             &hex::decode(
//                 &(String::from("0000000000000000000000000000000000000000000000000000000000000000")
//                     + "0000000000000000000000000000000000000000000000000000000000000000"),
//             )
//             .unwrap(),
//         )
//         .unwrap()
//     }
//     pub fn from_r_s(r: &str, s: &str) -> Self {
//         SignatureType::deserialize_packed(&hex::decode(&(String::from(r) + s)).unwrap()).unwrap()
//     }
// }
//
// impl Debug for SignatureType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "PackedSignature:[{:?}]",
//             self.0.serialize_packed().unwrap()
//         )
//     }
// }
//
// // TODO: how to implement PartialEq for SignatureType without unwrap?
// // or how to ignore PartialEq for SignatureType.
// impl PartialEq for SignatureType {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.serialize_packed().unwrap() == other.0.serialize_packed().unwrap()
//     }
//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }
// }
//
// #[derive(Debug)]
// pub enum Error {}
