use crate::values::DataCoding;

pub(super) trait SealedEncoder {}

#[allow(private_bounds)]
pub trait Encoder<T>: SealedEncoder {
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

    /// TODO: what should this be called?
    fn padding(&self) -> usize {
        0
    }
}

impl<F, E> SealedEncoder for F where F: Fn(&[u8]) -> Result<alloc::vec::Vec<u8>, E> {}

impl<F, E> Encoder<&[u8]> for F
where
    F: Fn(&[u8]) -> Result<alloc::vec::Vec<u8>, E>,
{
    type Error = E;

    fn encode(&self, value: &[u8]) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        (self)(value)
    }

    fn data_coding(&self) -> DataCoding {
        DataCoding::McSpecific
    }
}
