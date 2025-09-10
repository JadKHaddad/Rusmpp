/// An error that can occur when decoding `SMPP` values.
#[derive(Debug)]
pub struct DecodeError {
    kind: DecodeErrorKind,
}

impl DecodeError {
    #[inline]
    pub const fn new(kind: DecodeErrorKind) -> Self {
        Self { kind }
    }

    #[inline]
    pub const fn kind(&self) -> DecodeErrorKind {
        self.kind
    }

    #[inline]
    pub const fn unexpected_eof() -> Self {
        Self::new(DecodeErrorKind::UnexpectedEof)
    }

    #[inline]
    pub const fn c_octet_string_decode_error(error: COctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::COctetStringDecodeError(error))
    }

    #[inline]
    pub const fn octet_string_decode_error(error: OctetStringDecodeError) -> Self {
        Self::new(DecodeErrorKind::OctetStringDecodeError(error))
    }

    #[inline]
    pub const fn unsupported_key(key: u32) -> Self {
        Self::new(DecodeErrorKind::UnsupportedKey { key })
    }
}

/// Kind of [`DecodeError`].
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DecodeErrorKind {
    UnexpectedEof,
    COctetStringDecodeError(COctetStringDecodeError),
    OctetStringDecodeError(OctetStringDecodeError),
    UnsupportedKey { key: u32 },
}

/// An error that can occur when decoding a [`COctetString`](struct@crate::types::COctetString).
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum COctetStringDecodeError {
    TooFewBytes { actual: usize, min: usize },
    NotAscii,
    NotNullTerminated,
}

/// An error that can occur when decoding an [`OctetString`](struct@crate::types::OctetString).
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum OctetStringDecodeError {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Decode error. kind: {}", self.kind)
    }
}

impl core::error::Error for DecodeError {}

impl core::fmt::Display for DecodeErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeErrorKind::UnexpectedEof => write!(f, "Unexpected EOF"),
            DecodeErrorKind::COctetStringDecodeError(e) => write!(f, "COctetString error: {e}"),
            DecodeErrorKind::OctetStringDecodeError(e) => write!(f, "OctetString error: {e}"),
            DecodeErrorKind::UnsupportedKey { key } => write!(f, "Unsupported key: {key}"),
        }
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

pub(crate) trait DecodeResultExt<T, E> {
    fn map_decoded<F, U>(self, op: F) -> Result<(U, usize), E>
    where
        F: FnOnce(T) -> U;
}

impl<T, E> DecodeResultExt<T, E> for Result<(T, usize), E> {
    fn map_decoded<F, U>(self, op: F) -> Result<(U, usize), E>
    where
        F: FnOnce(T) -> U,
    {
        self.map(|(this, size)| (op(this), size))
    }
}
