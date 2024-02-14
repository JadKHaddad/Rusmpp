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

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EncodeError::IoError(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

pub trait Encode: Length {
    /// Encode a value to a writer
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError>;

    /// Encode a value to a vector
    fn encode_into_vec(&self) -> Result<Vec<u8>, EncodeError> {
        let mut buf = Vec::with_capacity(self.length());

        tri!(self.encode_to(&mut buf));

        Ok(buf)
    }
}
