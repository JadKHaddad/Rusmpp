use crate::tri;

use super::length::Length;

#[derive(Debug)]
pub enum EncodeError {
    IoError(crate::io::Error),
}

impl From<crate::io::Error> for EncodeError {
    fn from(e: crate::io::Error) -> Self {
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
    fn encode_to<W: crate::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError>;

    #[cfg(feature = "alloc")]
    /// Encode a value into a vector
    fn encode_into_vec(&self) -> Result<::alloc::vec::Vec<u8>, EncodeError> {
        let mut buf = ::alloc::vec::Vec::with_capacity(self.length());

        tri!(self.encode_to(&mut buf));

        Ok(buf)
    }
}
