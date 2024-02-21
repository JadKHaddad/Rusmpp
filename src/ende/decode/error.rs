/// An error that can occur when decoding a [`Command`](struct@crate::commands::command::Command)
#[derive(Debug)]
pub enum DecodeError {
    IoError(std::io::Error),
    COctetStringDecodeError(COctetStringDecodeError),
    OctetStringDecodeError(OctetStringDecodeError),
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

impl From<std::io::Error> for DecodeError {
    fn from(e: std::io::Error) -> Self {
        DecodeError::IoError(e)
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::IoError(e) => write!(f, "I/O error: {}", e),
            DecodeError::COctetStringDecodeError(e) => write!(f, "COctetString error: {}", e),
            DecodeError::OctetStringDecodeError(e) => write!(f, "OctetString error: {}", e),
            DecodeError::UnsupportedKey { key } => write!(f, "Unsupported key: {}", key),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodeError::IoError(e) => Some(e),
            DecodeError::COctetStringDecodeError(e) => Some(e),
            DecodeError::OctetStringDecodeError(e) => Some(e),
            DecodeError::UnsupportedKey { .. } => None,
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for COctetStringDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            COctetStringDecodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {}, min: {}", actual, min)
            }
            COctetStringDecodeError::NotAscii => write!(f, "Not ASCII"),
            COctetStringDecodeError::NotNullTerminated => write!(f, "Not null terminated"),
        }
    }
}

impl std::error::Error for COctetStringDecodeError {}

impl std::fmt::Display for OctetStringDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OctetStringDecodeError::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {}, max: {}", actual, max)
            }
            OctetStringDecodeError::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {}, min: {}", actual, min)
            }
        }
    }
}

impl std::error::Error for OctetStringDecodeError {}
