/// An error that can occur when decoding a [`PduIn`](struct@crate::pdus::pdu::PduIn)
#[derive(Debug)]
pub enum DecodeError {
    IoError(std::io::Error),
    COctetStringEncodeError(COctetStringEncodeError),
    OctetStringEncodeError(OctetStringEncodeError),
    UnsupportedKey { key: u32 },
}

/// An error that can occur when decoding a [`COctetString`](struct@crate::types::c_octet_string::COctetString)
#[derive(Debug)]
pub enum COctetStringEncodeError {
    TooFewBytes { actual: usize, min: usize },
    NotAscii,
    NotNullTerminated,
}

/// An error that can occur when decoding an [`OctetString`](struct@crate::types::octet_string::OctetString)
#[derive(Debug)]
pub enum OctetStringEncodeError {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
}

impl From<std::io::Error> for DecodeError {
    fn from(e: std::io::Error) -> Self {
        DecodeError::IoError(e)
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::IoError(e) => write!(f, "I/O error: {}", e),
            DecodeError::COctetStringEncodeError(e) => write!(f, "COctetString error: {}", e),
            DecodeError::OctetStringEncodeError(e) => write!(f, "OctetString error: {}", e),
            DecodeError::UnsupportedKey { key } => write!(f, "Unsupported key: {}", key),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodeError::IoError(e) => Some(e),
            DecodeError::COctetStringEncodeError(e) => Some(e),
            DecodeError::OctetStringEncodeError(e) => Some(e),
            DecodeError::UnsupportedKey { .. } => None,
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for COctetStringEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            COctetStringEncodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {}, min: {}", actual, min)
            }
            COctetStringEncodeError::NotAscii => write!(f, "Not ASCII"),
            COctetStringEncodeError::NotNullTerminated => write!(f, "Not null terminated"),
        }
    }
}

impl std::error::Error for COctetStringEncodeError {}

impl std::fmt::Display for OctetStringEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OctetStringEncodeError::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {}, max: {}", actual, max)
            }
            OctetStringEncodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {}, min: {}", actual, min)
            }
        }
    }
}

impl std::error::Error for OctetStringEncodeError {}
