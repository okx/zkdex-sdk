use hex::FromHexError;

use pairing_ce::ff::{PrimeField, PrimeFieldDecodingError, PrimeFieldRepr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Incorrect input size. Actual: {size}, expected: {expected_size}")]
    IncorrectInputSize { size: usize, expected_size: usize },
    #[error("Cannot decode hex: {0}")]
    HexDecodingError(#[from] FromHexError),
    #[error("Cannot parse value {0}")]
    ParsingError(std::io::Error),
    #[error("Cannot convert into prime field value: {0}")]
    PrimeFieldDecodingError(#[from] PrimeFieldDecodingError),
}

/// Extension trait denoting common conversion method for field elements.
pub trait FeConvert: PrimeField {
    /// Converts the field element into a byte array.
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(32);
        self.into_repr().write_be(&mut buf).unwrap();

        buf
    }

    /// Reads a field element from its byte sequence representation.
    fn from_bytes(value: &[u8]) -> Result<Self, ConversionError> {
        let mut repr = Self::Repr::default();

        // `repr.as_ref()` converts `repr` to a list of `u64`. Each element has 8 bytes,
        // so to obtain size in bytes, we multiply the array size with the size of `u64`.
        let expected_input_size = repr.as_ref().len() * 8;
        if value.len() != expected_input_size {
            return Err(ConversionError::IncorrectInputSize {
                size: value.len(),
                expected_size: expected_input_size,
            });
        }
        repr.read_be(value).map_err(ConversionError::ParsingError)?;
        Self::from_repr(repr).map_err(From::from)
    }

    /// Returns hex representation of the field element without `0x` prefix.
    fn to_hex(&self) -> String {
        let mut buf: Vec<u8> = Vec::with_capacity(32);

        self.into_repr().write_be(&mut buf).unwrap();
        hex::encode(&buf)
    }

    /// Reads a field element from its hexadecimal representation.
    fn from_hex(value: &str) -> Result<Self, ConversionError> {
        let value = if let Some(value) = value.strip_prefix("0x") {
            value
        } else {
            value
        };

        // Buffer is reversed and read as little endian, since we pad it with zeros to
        // match the expected length.
        let mut buf = hex::decode(value)?;
        buf.reverse();
        let mut repr = Self::Repr::default();

        // `repr.as_ref()` converts `repr` to a list of `u64`. Each element has 8 bytes,
        // so to obtain size in bytes, we multiply the array size with the size of `u64`.
        buf.resize(repr.as_ref().len() * 8, 0);

        repr.read_le(&buf[..])
            .map_err(ConversionError::ParsingError)?;
        Self::from_repr(repr).map_err(From::from)
    }
}

impl<T> FeConvert for T where T: PrimeField {}
