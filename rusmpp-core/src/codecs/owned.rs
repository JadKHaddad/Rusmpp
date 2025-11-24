use crate::{sealed::Sealed, values::DataCoding};

#[allow(private_bounds)]
pub trait Encoder<T>: Sealed {
    /// The associated error type for encoding operations.
    type Error;

    /// Encodes the input value into a new `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)` with the encoded bytes.
    /// - `Err(Self::Error)` if an encoding error occurs.
    fn encode(&self, value: T) -> Result<alloc::vec::Vec<u8>, Self::Error>;

    /// finalizes the encoded value knowing that the `header` will be prepended.
    #[allow(unused_variables)]
    fn finalize(&self, header: &[u8], encoded: alloc::vec::Vec<u8>) -> alloc::vec::Vec<u8> {
        encoded
    }

    /// The corresponding data coding for the encoded value.
    fn data_coding(&self) -> DataCoding;

    /// TODO: document this
    fn tolerance(&self) -> usize {
        0
    }
}
