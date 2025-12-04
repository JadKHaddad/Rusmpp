use rusmpp_core::values::DataCoding;

/// A trait for encoding messages into byte vectors.
pub trait Encoder {
    /// The type of errors that can occur during encoding.
    type Error;

    /// The associated [`DataCoding`] for this encoder.
    fn data_coding(&self) -> DataCoding;

    /// Encodes the given message into a vector of bytes.
    fn encode(&self, message: &str) -> Result<alloc::vec::Vec<u8>, Self::Error>;
}
