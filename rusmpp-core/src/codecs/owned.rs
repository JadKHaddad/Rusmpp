use crate::{codecs::UdhType, values::DataCoding};

pub trait Encoder<T> {
    /// The associated error type for encoding operations.
    type Error;

    /// Encodes the input value into a new `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)` with the encoded bytes.
    /// - `Err(Self::Error)` if an encoding error occurs.
    fn encode(&self, value: T) -> Result<alloc::vec::Vec<u8>, Self::Error>;

    /// The corresponding data coding for the encoded value.
    fn data_coding(&self) -> DataCoding;

    /// Max bytes for no UDH, after encoding.
    fn max_bytes(&self) -> usize;

    /// Max bytes for UDH, after encoding.
    fn max_bytes_with_udh(&self, udh: &UdhType) -> usize;
}
