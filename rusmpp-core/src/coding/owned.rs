pub trait Encode<T> {
    /// The associated error type for encoding operations.
    type Error;

    /// Encodes the input value into a new `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)` with the encoded bytes.
    /// - `Err(Self::Error)` if an encoding error occurs.
    fn encode(&self, value: T) -> Result<alloc::vec::Vec<u8>, Self::Error>;
}

/// Implements [`Encode`] for any function or closure that matches the signature.
impl<F, T, E> Encode<T> for F
where
    F: Fn(T) -> Result<alloc::vec::Vec<u8>, E>,
{
    type Error = E;

    fn encode(&self, value: T) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        self(value)
    }
}
