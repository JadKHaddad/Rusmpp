#![allow(path_statements)]

use crate::ende::{
    decode::{COctetStringDecodeError, Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

/// An Error that can occur when creating a [`COctetString`]
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
    NotNullTerminated,
    NotAscii,
    NullByteFound,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {actual}, max: {max}")
            }
            Error::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {actual}, min: {min}")
            }
            Error::NotNullTerminated => write!(f, "Not null terminated"),
            Error::NotAscii => write!(f, "Not ASCII"),
            Error::NullByteFound => write!(f, "Null byte found"),
        }
    }
}

impl core::error::Error for Error {}

/// A [`COctetString`] is a sequence of ASCII characters
/// terminated with a NULL octet (0x00).
/// The string “Hello” would be encoded in 6 octets (5
/// characters of “Hello” and NULL octet) as follows:
///
/// 0x48656C6C6F00
///
/// Two special variants exist for use within `SMPP`. These
/// are [`COctetString`] (Decimal) and [`COctetString`]
/// (Hexadecimal), which are used to carry decimal and
/// hexadecimal digit sequences respectively. These fields
/// are encoded the same way as any ASCII string, but are
/// specifically used to designate decimal and hexadecimal
/// numbers when presented in string format.
///
/// A Decimal [`COctetString`] “123456789” would be encoded
/// as follows:
///
/// 0x31323334353637383900
///
/// A Hexadecimal [`COctetString`] “A2F5ED278FC” would be
/// encoded as follows:
///
/// 0x413246354544323738464300
///
/// A NULL string “” is encoded as 0x00
///
/// # Notes
///
/// `MIN` must be greater than 0.
/// ```rust, compile_fail
/// use rusmpp::types::COctetString;
///
/// // does not compile
/// let string = COctetString::<0, 6>::new(b"Hello\0");
/// ```
/// `MIN` must be less than or equal to `MAX`.
///
/// ```rust, compile_fail
/// use rusmpp::types::COctetString;
///
/// // does not compile
/// let string = COctetString::<10, 6>::new(b"Hello\0");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct COctetString<const MIN: usize, const MAX: usize> {
    bytes: Vec<u8>,
}

impl<const MIN: usize, const MAX: usize> COctetString<MIN, MAX> {
    const _ASSERT_MIN_NON_ZERO: () = assert!(MIN > 0, "MIN must be greater than 0");
    const _ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX: () =
        assert!(MIN <= MAX, "MIN must be less than or equal to MAX");

    const _ASSERT_VALID: () = {
        Self::_ASSERT_MIN_NON_ZERO;
        Self::_ASSERT_MIN_LESS_THAN_OR_EQUAL_TO_MAX;
    };

    /// Create a new empty [`COctetString`].
    ///
    /// Equivalent to [`COctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`COctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self::_ASSERT_VALID;

        Self { bytes: vec![0] }
    }

    /// Check if a [`COctetString`] is empty.
    ///
    /// A [`COctetString`] is considered empty if it
    /// contains only a single NULL octet (0x00).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 1
    }

    /// Create a new [`COctetString`] from a sequence of bytes.
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::_ASSERT_VALID;

        let bytes = bytes.as_ref();

        if bytes[bytes.len() - 1] != 0 {
            return Err(Error::NotNullTerminated);
        }

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

        if !bytes.is_ascii() {
            return Err(Error::NotAscii);
        }

        if bytes[..bytes.len() - 1].contains(&0) {
            return Err(Error::NullByteFound);
        }

        let bytes = bytes.to_vec();

        Ok(Self { bytes })
    }

    /// Create a new [`COctetString`] from a sequence of bytes without checking the length and null termination.
    #[inline]
    pub(crate) fn new_unchecked(bytes: impl AsRef<[u8]>) -> Self {
        Self::_ASSERT_VALID;

        let bytes = bytes.as_ref().to_vec();

        Self { bytes }
    }

    /// Convert a [`COctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
    }

    /// Get the bytes of a [`COctetString`].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert a [`COctetString`] to a [`Vec`] of [`u8`].
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> Default for COctetString<MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Debug for COctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("COctetString")
            .field("bytes", &crate::utils::HexFormatter(&self.bytes))
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> core::str::FromStr for COctetString<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::_ASSERT_VALID;

        let bytes = s.as_bytes();

        if bytes.len() + 1 > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len() + 1,
                max: MAX,
            });
        }

        if bytes.len() + 1 < MIN {
            return Err(Error::TooFewBytes {
                actual: bytes.len() + 1,
                min: MIN,
            });
        }

        if !bytes.is_ascii() {
            return Err(Error::NotAscii);
        }

        if bytes[..bytes.len()].contains(&0) {
            return Err(Error::NullByteFound);
        }

        let mut bytes = bytes.to_vec();

        bytes.push(0);

        Ok(Self { bytes })
    }
}

impl<const MIN: usize, const MAX: usize> core::fmt::Display for COctetString<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(
            &self.bytes[..self.bytes.len() - 1],
        ))
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<[u8]> for COctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> Length for COctetString<MIN, MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for COctetString<MIN, MAX> {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl<const MIN: usize, const MAX: usize> Decode for COctetString<MIN, MAX> {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        use std::io::Read;

        Self::_ASSERT_VALID;

        let mut bytes = Vec::with_capacity(MAX);

        let mut reader_bytes = reader.bytes();

        for _ in 0..MAX {
            if let Some(Ok(byte)) = reader_bytes.next() {
                bytes.push(byte);

                if byte == 0 {
                    break;
                }
            }
        }

        if bytes.last() != Some(&0x00) {
            return Err(DecodeError::COctetStringDecodeError(
                COctetStringDecodeError::NotNullTerminated,
            ));
        }

        if bytes.len() < MIN {
            return Err(DecodeError::COctetStringDecodeError(
                COctetStringDecodeError::TooFewBytes {
                    actual: bytes.len(),
                    min: MIN,
                },
            ));
        }

        if !bytes.is_ascii() {
            return Err(DecodeError::COctetStringDecodeError(
                COctetStringDecodeError::NotAscii,
            ));
        }

        Ok(Self { bytes })
    }
}

impl<const MIN: usize, const MAX: usize> crate::ende::encode::Encode2 for COctetString<MIN, MAX> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.bytes.len()].copy_from_slice(&self.bytes);

        self.bytes.len()
    }
}

impl<const MIN: usize, const MAX: usize> crate::ende::decode::Decode2 for COctetString<MIN, MAX> {
    fn decode(src: &mut [u8]) -> Result<(Self, usize), crate::ende::decode::DecodeError2> {
        use crate::ende::decode::{COctetStringDecodeError, DecodeError2};

        Self::_ASSERT_VALID;

        if src.len() < MIN {
            return Err(DecodeError2::COctetStringDecodeError(
                COctetStringDecodeError::TooFewBytes {
                    actual: src.len(),
                    min: MIN,
                },
            ));
        }

        let mut bytes = Vec::with_capacity(MAX);

        for &byte in src.iter().take(MAX) {
            bytes.push(byte);

            if byte == 0 {
                break;
            }
        }

        if bytes.last() != Some(&0x00) {
            return Err(DecodeError2::COctetStringDecodeError(
                COctetStringDecodeError::NotNullTerminated,
            ));
        }

        if !bytes.is_ascii() {
            return Err(DecodeError2::COctetStringDecodeError(
                COctetStringDecodeError::NotAscii,
            ));
        }

        let size = bytes.len();

        Ok((Self { bytes }, size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<1, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<10, 20>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 6, .. }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = COctetString::<1, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = COctetString::<1, 6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0o\0";
            let error = COctetString::<1, 6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let bytes = b"\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod from_str {
        use core::str::FromStr;

        use super::*;

        #[test]
        fn too_many_bytes() {
            let string = "Hello";
            let error = COctetString::<1, 5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let string = "Hello";
            let error = COctetString::<10, 20>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 6, .. }));
        }

        #[test]
        fn null_byte_found() {
            let string = "Hel\0o";
            let error = COctetString::<1, 6>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn null_byte_found_at_the_end() {
            let string = "Hell\0";
            let error = COctetString::<1, 6>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let string = "Hello";
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let string = "Hello";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let string = "";
            let bytes = b"\0";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let string = "";
            let string = COctetString::<1, 6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod to_str {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::new(bytes).unwrap();
            assert_eq!(string.to_str().unwrap(), "Hello");
            assert_eq!(string.to_string(), "Hello");
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn not_null_terminated_empty() {
            let bytes = b"";
            let error = COctetString::<1, 6>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
            ));
        }

        #[test]
        fn not_null_terminated_not_empty() {
            let bytes = b"hi";
            let error = COctetString::<1, 6>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
            ));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<1, 5>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
            ));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<10, 20>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::TooFewBytes {
                    actual: 6,
                    ..
                })
            ));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = COctetString::<1, 6>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotAscii)
            ));
        }

        #[test]
        fn ok_max() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 6>::decode_from(&mut bytes.as_ref()).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_not_max() {
            let bytes = b"Hello\0";
            let string = COctetString::<1, 25>::decode_from(&mut bytes.as_ref()).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty_max() {
            let bytes = b"\0";
            let string = COctetString::<1, 1>::decode_from(&mut bytes.as_ref()).unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
        }

        #[test]
        fn ok_empty_not_max() {
            let bytes = b"\0";
            let string = COctetString::<1, 25>::decode_from(&mut bytes.as_ref()).unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
        }

        #[test]
        fn ok_remaining() {
            let bytes = b"Hello\0World!";
            let buf = &mut bytes.as_ref();
            let string = COctetString::<1, 10>::decode_from(buf).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(buf, b"World!");
        }
    }
}
