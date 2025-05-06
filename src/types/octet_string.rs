#[cfg(feature = "alloc")]
use ::alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::ende::{
    decode::{DecodeError, DecodeWithLength, OctetStringDecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

/// An error that can occur when creating an [`OctetString`]
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {actual}, max: {max}")
            }
            Self::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
        }
    }
}

impl core::error::Error for Error {}

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
    #[cfg(feature = "alloc")]
    bytes: Vec<u8>,

    #[cfg(not(feature = "alloc"))]
    bytes: heapless::Vec<u8, MAX>,
}

impl<const MIN: usize, const MAX: usize> OctetString<MIN, MAX> {
    const _ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX: () =
        assert!(MIN <= MAX, "MIN must be less than or equal to MAX");

    /// Create a new empty [`OctetString`].
    ///
    /// Equivalent to [`OctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`OctetString`].
    #[inline]
    pub fn empty() -> Self {
        #[allow(path_statements)]
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;

        #[cfg(feature = "alloc")]
        return Self { bytes: Vec::new() };

        #[cfg(not(feature = "alloc"))]
        Self {
            bytes: heapless::Vec::new(),
        }
    }

    /// Check if an [`OctetString`] is empty.
    ///
    /// An [`OctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        #[allow(path_statements)]
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;

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

        #[cfg(feature = "alloc")]
        let bytes = bytes.to_vec();

        #[cfg(not(feature = "alloc"))]
        let bytes = {
            let mut heapless_bytes = heapless::Vec::<u8, MAX>::new();

            heapless_bytes
                .extend_from_slice(bytes)
                .expect("bytes.len() must not be greater than MAX");

            heapless_bytes
        };

        Ok(Self { bytes })
    }

    /// Convert an [`OctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes)
    }

    /// Get the bytes of an [`OctetString`].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`OctetString`] to a [`Vec`] of [`u8`].
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Debug for OctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "alloc")]
        {
            f.debug_struct("OctetString")
                .field("bytes", &crate::utils::HexFormatter(&self.bytes))
                .field("string", &self.to_string())
                .finish()
        }

        #[cfg(not(feature = "alloc"))]
        f.debug_struct("OctetString")
            .field("bytes", &crate::utils::HexFormatter(&self.bytes))
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> Default for OctetString<MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> core::str::FromStr for OctetString<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.as_bytes())
    }
}

#[cfg(feature = "alloc")]
impl<const MIN: usize, const MAX: usize> core::fmt::Display for OctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.bytes))
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<[u8]> for OctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(feature = "alloc")]
impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>>
    for super::any_octet_string::AnyOctetString
{
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
    fn encode_to<W: crate::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl<const MIN: usize, const MAX: usize> DecodeWithLength for OctetString<MIN, MAX> {
    fn decode_from<R: crate::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        #[allow(path_statements)]
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;

        if length > MAX {
            return Err(DecodeError::OctetStringDecodeError(
                OctetStringDecodeError::TooManyBytes {
                    actual: length,
                    max: MAX,
                },
            ));
        }

        if length < MIN {
            return Err(DecodeError::OctetStringDecodeError(
                OctetStringDecodeError::TooFewBytes {
                    actual: length,
                    min: MIN,
                },
            ));
        }

        #[cfg(feature = "alloc")]
        let bytes = {
            let mut bytes = vec![0; length];

            reader.read_exact(&mut bytes)?;

            bytes
        };

        #[cfg(not(feature = "alloc"))]
        let bytes = {
            let mut temp = [0u8; MAX];

            reader.read_exact(&mut temp[..length])?;

            let mut bytes: heapless::Vec<u8, MAX> = heapless::Vec::new();

            bytes
                .extend_from_slice(&temp[..length])
                .expect("length must not be greater than MAX");

            bytes
        };

        Ok(Self { bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0World!\0";
            let error = OctetString::<0, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 13, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello World!";
            let error = OctetString::<15, 20>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 12, .. }));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn not_enough_bytes() {
            let bytes = b"";
            let error = OctetString::<0, 6>::decode_from(&mut bytes.as_ref(), 5).unwrap_err();

            assert!(matches!(error, DecodeError::IoError { .. }));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello";
            let error = OctetString::<0, 5>::decode_from(&mut bytes.as_ref(), 15).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::OctetStringDecodeError(OctetStringDecodeError::TooManyBytes {
                    actual: 15,
                    ..
                },)
            ));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello";
            let error = OctetString::<6, 10>::decode_from(&mut bytes.as_ref(), 5).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::OctetStringDecodeError(OctetStringDecodeError::TooFewBytes {
                    actual: 5,
                    ..
                },)
            ));
        }

        #[test]
        fn ok_all() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = OctetString::<0, 5>::decode_from(buf, 5).unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(buf, b"");
        }

        #[test]
        fn ok_partial() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = OctetString::<0, 5>::decode_from(buf, 3).unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(buf, b"lo");
        }
    }
}
