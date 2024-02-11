use super::no_fixed_size_octet_string::NoFixedSizeOctetString;
use crate::io::{
    decode::{DecodeError, DecodeWithLength},
    encode::{Encode, EncodeError},
    length::Length,
};

/// An error that can occur when creating an [`OctetString`]
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {}, max: {}", actual, max)
            }
            Self::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {}, min: {}", actual, min)
            }
        }
    }
}

impl std::error::Error for Error {}

/// An [`OctetString`] is a sequence of octets not necessarily
/// terminated with a NULL octet. Such fields using Octet
/// String encoding, typically represent fields that can be
/// used to encode raw binary data. In all circumstances, the
/// field will be either a fixed length field or explicit length field
/// where another field indicates the length of the Octet
/// String field. An example of this is the short_message field
/// of the submit_sm PDU that is [`OctetString`] encoded and
/// the previous message_length field specifies its length.
///
/// A NULL [`OctetString`] is not encoded. The explicit length
/// field that indicates its length should be set to zero.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OctetString<const MIN: usize, const MAX: usize> {
    bytes: Vec<u8>,
}

impl<const MIN: usize, const MAX: usize> OctetString<MIN, MAX> {
    /// Create a new empty [`OctetString`] String
    #[inline]
    pub fn empty() -> Self {
        Self { bytes: vec![] }
    }

    /// Check if an [`OctetString`] is empty
    ///
    /// An [`OctetString`] is considered empty if it
    /// contains no octets
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes.len() > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: MAX,
            });
        }

        if bytes.len() < MIN {
            return Err(Error::TooFewBytes {
                actual: bytes.len(),
                min: MIN,
            });
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    /// Convert an [`OctetString`] to a &[`str`]
    #[inline]
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes)
    }

    /// Get the bytes of an [`OctetString`]
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`OctetString`] to a [`Vec`] of [`u8`]
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> std::fmt::Debug for OctetString<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> Default for OctetString<MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> std::str::FromStr for OctetString<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.as_bytes())
    }
}

impl<const MIN: usize, const MAX: usize> ToString for OctetString<MIN, MAX> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes).to_string()
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<[u8]> for OctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>> for NoFixedSizeOctetString {
    fn from(octet_string: OctetString<MIN, MAX>) -> Self {
        Self::new(octet_string.bytes)
    }
}

impl<const MIN: usize, const MAX: usize> Length for OctetString<MIN, MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for OctetString<MIN, MAX> {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl<const MIN: usize, const MAX: usize> DecodeWithLength for OctetString<MIN, MAX> {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        todo!()
    }
}
