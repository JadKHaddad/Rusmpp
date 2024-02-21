use crate::ende::{
    decode::{COctetStringDecodeError, Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};
use std::io::Read;

/// An error that can occur when creating a [`EmptyOrFullCOctetString`]
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize },
    NotNullTerminated,
    NotAscii,
    NullByteFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {}, max: {}", actual, max)
            }
            Self::TooFewBytes { actual } => {
                write!(f, "Too few bytes. actual: {}, min: 1", actual)
            }
            Self::NotNullTerminated => write!(f, "Not null terminated"),
            Self::NotAscii => write!(f, "Not ASCII"),
            Self::NullByteFound => write!(f, "Null byte found"),
        }
    }
}

impl std::error::Error for Error {}

/// Empty or full [`COctetString`](struct@crate::types::c_octet_string::COctetString)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyOrFullCOctetString<const N: usize> {
    bytes: Vec<u8>,
}

impl<const N: usize> EmptyOrFullCOctetString<N> {
    /// Create a new empty [`EmptyOrFullCOctetString`].
    ///
    /// Equivalent to [`EmptyOrFullCOctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`EmptyOrFullCOctetString`].
    #[inline]
    pub fn empty() -> Self {
        Self { bytes: vec![0] }
    }

    /// Check if an [`EmptyOrFullCOctetString`] is empty.
    ///
    /// An [`EmptyOrFullCOctetString`] is considered empty if it
    /// contains only a single NULL octet (0x00).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 1
    }

    /// Create a new [`EmptyOrFullCOctetString`] from a sequence of bytes.
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes[bytes.len() - 1] != 0 {
            return Err(Error::NotNullTerminated);
        }

        if !bytes.is_ascii() {
            return Err(Error::NotAscii);
        }

        if bytes[..bytes.len() - 1].contains(&0) {
            return Err(Error::NullByteFound);
        }

        if bytes.len() > 1 {
            if bytes.len() < N {
                return Err(Error::TooFewBytes {
                    actual: bytes.len(),
                });
            }

            if bytes.len() > N {
                return Err(Error::TooManyBytes {
                    actual: bytes.len(),
                    max: N,
                });
            }
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    /// Convert an [`EmptyOrFullCOctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
    }

    /// Get the bytes of an [`EmptyOrFullCOctetString`].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`EmptyOrFullCOctetString`] to a [`Vec`] of [`u8`].
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const N: usize> std::fmt::Debug for EmptyOrFullCOctetString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const N: usize> Default for EmptyOrFullCOctetString<N> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const N: usize> std::str::FromStr for EmptyOrFullCOctetString<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);

        Self::new(bytes)
    }
}

impl<const N: usize> ToString for EmptyOrFullCOctetString<N> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes[..self.bytes.len() - 1]).to_string()
    }
}

impl<const N: usize> AsRef<[u8]> for EmptyOrFullCOctetString<N> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const N: usize> Length for EmptyOrFullCOctetString<N> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const N: usize> Encode for EmptyOrFullCOctetString<N> {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl<const N: usize> Decode for EmptyOrFullCOctetString<N> {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut bytes = Vec::with_capacity(N);

        let mut reader_bytes = reader.bytes();
        for _ in 0..N {
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

        if bytes.len() > 1 && bytes.len() < N {
            return Err(DecodeError::COctetStringDecodeError(
                COctetStringDecodeError::TooFewBytes {
                    actual: bytes.len(),
                    min: N,
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

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hel\0";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 4 }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = EmptyOrFullCOctetString::<6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0lo\0";
            let error = EmptyOrFullCOctetString::<6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod from_str {
        use std::str::FromStr;

        use super::*;

        #[test]
        fn too_many_bytes() {
            let string = "Hello";
            let error = EmptyOrFullCOctetString::<5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let string = "Hel";
            let error = EmptyOrFullCOctetString::<5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 4 }));
        }

        #[test]
        fn null_byte_found() {
            let string = "Hel\0lo";
            let error = EmptyOrFullCOctetString::<6>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let string = "Hello";
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let string = "Hello";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let string = "";
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let string = "";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn not_null_terminated_empty() {
            let bytes = b"";
            let error = EmptyOrFullCOctetString::<6>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
            ));
        }

        #[test]
        fn not_null_terminated_empty_not_empty() {
            let bytes = b"Hi";
            let error = EmptyOrFullCOctetString::<6>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
            ));
        }

        #[test]
        fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = EmptyOrFullCOctetString::<5>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated,)
            ));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hel\0";
            let error = EmptyOrFullCOctetString::<5>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::TooFewBytes {
                    actual: 4,
                    ..
                },)
            ));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = EmptyOrFullCOctetString::<6>::decode_from(&mut bytes.as_ref()).unwrap_err();

            assert!(matches!(
                error,
                DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotAscii,)
            ));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!";
            let buf = &mut bytes.as_ref();
            let string = EmptyOrFullCOctetString::<6>::decode_from(buf).unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(buf, b"World!");
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0World!";
            let buf = &mut bytes.as_ref();
            let string = EmptyOrFullCOctetString::<6>::decode_from(buf).unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(buf, b"World!");
        }
    }
}
