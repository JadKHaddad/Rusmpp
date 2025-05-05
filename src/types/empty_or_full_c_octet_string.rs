use crate::ende::{
    decode::{COctetStringDecodeError, Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

/// An error that can occur when creating a [`EmptyOrFullCOctetString`]
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize },
    NotNullTerminated,
    NotAscii,
    NullByteFound,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {actual}, max: {max}")
            }
            Self::TooFewBytes { actual } => {
                write!(f, "Too few bytes. actual: {actual}, min: 1")
            }
            Self::NotNullTerminated => write!(f, "Not null terminated"),
            Self::NotAscii => write!(f, "Not ASCII"),
            Self::NullByteFound => write!(f, "Null byte found"),
        }
    }
}

impl core::error::Error for Error {}

/// Empty or full [`COctetString`](struct@crate::types::c_octet_string::COctetString)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyOrFullCOctetString<const N: usize> {
    #[cfg(feature = "alloc")]
    bytes: Vec<u8>,

    #[cfg(not(feature = "alloc"))]
    bytes: heapless::Vec<u8, N>,
}

// TODO: what happens if N is 0?

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
        #[cfg(feature = "alloc")]
        return Self { bytes: vec![0] };

        #[cfg(not(feature = "alloc"))]
        Self {
            // TODO
            bytes: heapless::Vec::from_slice(&[0]).expect("TODO"),
        }
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

        #[cfg(feature = "alloc")]
        let bytes = bytes.to_vec();

        #[cfg(not(feature = "alloc"))]
        let bytes = {
            let mut heapless_bytes = heapless::Vec::<u8, N>::new();

            heapless_bytes
                .extend_from_slice(bytes)
                .expect("bytes.len() must not be greater than N");

            heapless_bytes
        };

        Ok(Self { bytes })
    }

    /// Convert an [`EmptyOrFullCOctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
    }

    /// Get the bytes of an [`EmptyOrFullCOctetString`].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`EmptyOrFullCOctetString`] to a [`Vec`] of [`u8`].
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const N: usize> core::fmt::Debug for EmptyOrFullCOctetString<N> {
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

impl<const N: usize> Default for EmptyOrFullCOctetString<N> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const N: usize> core::str::FromStr for EmptyOrFullCOctetString<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "alloc")]
        {
            let mut bytes = s.as_bytes().to_vec();
            bytes.push(0);

            return Self::new(bytes);
        }

        #[cfg(not(feature = "alloc"))] // TODO: fix
        {
            let mut bytes = heapless::Vec::<u8, N>::new();
            bytes
                .extend_from_slice(s.as_bytes())
                .map_err(|_| Error::TooManyBytes {
                    actual: s.as_bytes().len() + 1,
                    max: N,
                })?;

            bytes.push(0).map_err(|_| Error::TooManyBytes {
                actual: s.as_bytes().len() + 1,
                max: N,
            })?;

            return Self::new(bytes);
        }
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> core::fmt::Display for EmptyOrFullCOctetString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(
            &self.bytes[..self.bytes.len() - 1],
        ))
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
    fn encode_to<W: crate::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl<const N: usize> Decode for EmptyOrFullCOctetString<N> {
    fn decode_from<R: crate::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        #[cfg(feature = "alloc")]
        let mut bytes = Vec::with_capacity(N);

        #[cfg(not(feature = "alloc"))]
        let mut bytes = heapless::Vec::<u8, N>::new();

        let mut reader_bytes = reader.bytes();
        for _ in 0..N {
            if let Some(Ok(byte)) = reader_bytes.next() {
                #[cfg(feature = "alloc")]
                bytes.push(byte);

                #[cfg(not(feature = "alloc"))]
                bytes.push(byte).expect("Loop must exceed N");

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
        use core::str::FromStr;

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
            extern crate std;
            std::println!("error: {error:?}");
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
