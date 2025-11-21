pub trait Encode<T> {
    /// The associated error type for encoding operations.
    type Error;

    /// Encodes the input value into the provided output buffer.
    ///
    /// # Returns
    ///
    /// - `None` if the output buffer is not large enough.
    /// - `Some(Ok(usize))` with the number of bytes written to the output buffer.
    /// - `Some(Err(Self::Error))` if an encoding error occurs.
    fn encode(&self, value: T, out: &mut [u8]) -> Option<Result<usize, Self::Error>>;
}

/// Implements [`Encode`] for any function or closure that matches the signature.
impl<F, T, E> Encode<T> for F
where
    F: Fn(T, &mut [u8]) -> Option<Result<usize, E>>,
{
    type Error = E;

    fn encode(&self, value: T, out: &mut [u8]) -> Option<Result<usize, Self::Error>> {
        self(value, out)
    }
}
