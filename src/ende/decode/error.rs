/// An error that can occur when decoding a [`Command`](struct@crate::commands::command::Command)
#[derive(Debug)]
pub enum DecodeError {
    IoError(crate::io::Error),
    COctetStringDecodeError(COctetStringDecodeError),
    OctetStringDecodeError(OctetStringDecodeError),
    VecCapacityError(VecCapacityError),
    UnsupportedKey { key: u32 },
}

/// An error that can occur when decoding a [`COctetString`](struct@crate::types::c_octet_string::COctetString)
#[derive(Debug)]
pub enum COctetStringDecodeError {
    TooFewBytes { actual: usize, min: usize },
    NotAscii,
    NotNullTerminated,
}

/// An error that can occur when decoding an [`OctetString`](struct@crate::types::octet_string::OctetString)
#[derive(Debug)]
pub enum OctetStringDecodeError {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
}

#[derive(Debug)]
pub struct VecCapacityError {
    pub capacity: usize,
}

impl From<crate::io::Error> for DecodeError {
    fn from(e: crate::io::Error) -> Self {
        DecodeError::IoError(e)
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::IoError(e) => write!(f, "I/O error: {e}"),
            DecodeError::COctetStringDecodeError(e) => write!(f, "COctetString error: {e}"),
            DecodeError::OctetStringDecodeError(e) => write!(f, "OctetString error: {e}"),
            DecodeError::VecCapacityError(e) => write!(f, "Vector capacity error: {e}"),
            DecodeError::UnsupportedKey { key } => write!(f, "Unsupported key: {key}"),
        }
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            DecodeError::IoError(e) => Some(e),
            DecodeError::COctetStringDecodeError(e) => Some(e),
            DecodeError::OctetStringDecodeError(e) => Some(e),
            DecodeError::VecCapacityError(e) => Some(e),
            DecodeError::UnsupportedKey { .. } => None,
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

impl core::fmt::Display for COctetStringDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            COctetStringDecodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
            COctetStringDecodeError::NotAscii => write!(f, "Not ASCII"),
            COctetStringDecodeError::NotNullTerminated => write!(f, "Not null terminated"),
        }
    }
}

impl core::error::Error for COctetStringDecodeError {}

impl core::fmt::Display for OctetStringDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OctetStringDecodeError::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {actual}, max: {max}")
            }
            OctetStringDecodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
        }
    }
}

impl core::error::Error for OctetStringDecodeError {}

impl core::fmt::Display for VecCapacityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vector capacity error. capacity: {}", self.capacity)
    }
}

impl core::error::Error for VecCapacityError {}
