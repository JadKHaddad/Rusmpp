use crate::values::DataCoding;

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
}

/// Implements [`Encoder`] for any function or closure that matches the signature.
impl<F, T, E> Encoder<T> for F
where
    F: Fn(T) -> Result<alloc::vec::Vec<u8>, E>,
{
    type Error = E;

    fn encode(&self, value: T) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        self(value)
    }

    fn data_coding(&self) -> DataCoding {
        DataCoding::default()
    }
}
