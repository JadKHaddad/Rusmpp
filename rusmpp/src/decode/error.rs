use crate::fields::SmppField;

/// An error that can occur when decoding `SMPP` values.
#[derive(Debug)]
pub struct DecodeError {
    kind: DecodeErrorKind,
    #[cfg(feature = "verbose")]
    source: Option<alloc::boxed::Box<DecodeErrorSource>>,
}

impl DecodeError {
    #[inline]
    pub const fn new(kind: DecodeErrorKind) -> Self {
        #[cfg(feature = "verbose")]
        return Self { kind, source: None };

        #[cfg(not(feature = "verbose"))]
        Self { kind }
    }

    #[inline]
    #[cold]
    #[cfg(feature = "verbose")]
    pub fn with_source(mut self, field: SmppField, error: DecodeError) -> Self {
        self.source = Some(alloc::boxed::Box::new(DecodeErrorSource { field, error }));
        self
    }

    #[inline]
    #[cold]
    #[cfg(feature = "verbose")]
    pub fn as_source(self, field: SmppField) -> DecodeError {
        DecodeError::new(self.kind).with_source(field, self)
    }

    #[inline]
    #[cfg(feature = "verbose")]
    pub fn source(&self) -> Option<&DecodeErrorSource> {
        self.source.as_deref()
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

    /// Checks recursively if the field exists in the sources tree.
    #[cfg(feature = "verbose")]
    pub fn field_exists(&self, field: SmppField) -> bool {
        if let Some(source) = &self.source {
            if source.field == field {
                return true;
            }

            return source.error.field_exists(field);
        }

        false
    }
}

/// Source of [`DecodeError`].
#[derive(Debug)]
#[cfg(feature = "verbose")]
pub struct DecodeErrorSource {
    field: SmppField,
    error: DecodeError,
}

#[cfg(feature = "verbose")]
impl DecodeErrorSource {
    pub const fn field(&self) -> SmppField {
        self.field
    }

    pub const fn error(&self) -> &DecodeError {
        &self.error
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

#[cfg(feature = "verbose")]
impl core::fmt::Display for DecodeErrorSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "field: {:?}, error: {}", self.field, self.error)
    }
}

#[cfg(feature = "verbose")]
impl core::error::Error for DecodeErrorSource {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.error)
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "verbose")]
        return match &self.source {
            Some(source) => {
                write!(f, "Decode error. kind: {}, source: [{source}]", self.kind,)
            }
            None => write!(f, "Decode error. kind: {}", self.kind),
        };

        #[cfg(not(feature = "verbose"))]
        write!(f, "Decode error. kind: {}", self.kind)
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        #[cfg(feature = "verbose")]
        return match &self.source {
            Some(source) => Some(source.as_ref()),
            None => None,
        };

        #[cfg(not(feature = "verbose"))]
        None
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        core::error::Error::source(self)
    }
}

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

pub(crate) trait DecodeErrorExt<T> {
    fn map_as_source(self, field: SmppField) -> Result<T, DecodeError>;
}

impl<T> DecodeErrorExt<T> for Result<T, DecodeError> {
    #[cold]
    fn map_as_source(self, _field: SmppField) -> Result<T, DecodeError> {
        #[cfg(feature = "verbose")]
        return self.map_err(|error| error.as_source(_field));

        #[cfg(not(feature = "verbose"))]
        self
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "verbose")]
    fn invalid_password() {
        use crate::{Command, decode::DecodeWithLength, fields::SmppField};

        // bind_transmitter bytes
        #[rustfmt::skip]
        let bytes: [u8; 46] = [
            // Header
            0x00, 0x00, 0x00, 0x2E, // Command Length (46 bytes total)
            0x00, 0x00, 0x00, 0x02, // Command ID (bind_transmitter)
            0x00, 0x00, 0x00, 0x00, // Command Status (0 - OK)
            0x00, 0x00, 0x00, 0x01, // Sequence Number (1)

            // Body
            // system_id: "SMPP3TEST\0"
            0x53, 0x4D, 0x50, 0x50, 0x33, 0x54, 0x45, 0x53, 0x54, 0x00,
            // password: "secret08\0" WRONG! not null terminated!
            0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x30, 0x38,
            // system_type: "SUBMIT1" 
            0x53, 0x55, 0x42, 0x4D, 0x49, 0x54, 0x31, 0x00,
            // interface_version
            0x50, 
            // addr_ton
            0x01, 
            // addr_npi
            0x01, 
            // addr_range
            0x00,
        ];

        let error = Command::decode(&bytes[4..], 46 - 4).unwrap_err();

        assert!(error.field_exists(SmppField::password));
    }
}
