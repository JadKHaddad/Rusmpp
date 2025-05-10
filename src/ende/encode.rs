use crate::tri;

use super::length::Length;

#[derive(Debug)]
pub enum EncodeError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for EncodeError {
    fn from(e: std::io::Error) -> Self {
        EncodeError::IoError(e)
    }
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EncodeError::IoError(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl core::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            EncodeError::IoError(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

pub trait Encode: Length {
    /// Encode a value to a writer
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError>;

    /// Encode a value into a vector
    fn encode_into_vec(&self) -> Result<Vec<u8>, EncodeError> {
        let mut buf = Vec::with_capacity(self.length());

        tri!(self.encode_to(&mut buf));

        Ok(buf)
    }
}

pub trait Encode2 {
    fn encode(&self, dst: &mut [u8]) -> usize;

    fn encode_move(&self, dst: &mut [u8], size: usize) -> usize {
        size + self.encode(&mut dst[size..])
    }
}
