use crate::{coding::UdhType, values::DataCoding};

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

    /// Max characters for no UDH, based on encoding.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` if the encoding has a known max character count.
    /// - `None` if the encoding does not have a known max character count.
    fn max_chars(&self) -> Option<usize>;

    /// Max characters for UDH, based on encoding.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` if the encoding has a known max character count with UDH.
    /// - `None` if the encoding does not have a known max character count with UDH.
    /// - `Some(0)` if the UDH length exceeds the maximum allowed bytes `140`.
    fn max_chars_with_udh(&self, udh: UdhType) -> Option<usize> {
        if udh.length() >= 140 {
            return Some(0);
        }

        Some(140 - udh.length())
    }
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

    fn max_chars(&self) -> Option<usize> {
        None
    }
}
