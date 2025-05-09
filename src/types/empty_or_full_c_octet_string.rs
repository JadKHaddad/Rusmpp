#![allow(path_statements)]

use crate::{
    errors::{COctetStringDecodeError, DecodeError},
    Decode, Encode, Length,
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
///
/// # Notes
///
/// `N` must be greater than 0.
/// ```rust, compile_fail
/// use rusmpp::types::EmptyOrFullCOctetString;
///
/// // does not compile
/// let string = EmptyOrFullCOctetString::<0>::new(b"Hello\0");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyOrFullCOctetString<const N: usize> {
    bytes: Vec<u8>,
}

impl<const N: usize> EmptyOrFullCOctetString<N> {
    const _ASSERT_NON_ZERO: () = assert!(N > 0, "N must be greater than 0");

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
        Self::_ASSERT_NON_ZERO;

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
        Self::_ASSERT_NON_ZERO;

        let bytes = bytes.as_ref();

        if bytes[bytes.len() - 1] != 0 {
            return Err(Error::NotNullTerminated);
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

        if !bytes.is_ascii() {
            return Err(Error::NotAscii);
        }

        if bytes[..bytes.len() - 1].contains(&0) {
            return Err(Error::NullByteFound);
        }

        let bytes = bytes.to_vec();

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
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const N: usize> core::fmt::Debug for EmptyOrFullCOctetString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("COctetString")
            .field("bytes", &crate::utils::HexFormatter(&self.bytes))
            .field("string", &self.to_string())
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
        Self::_ASSERT_NON_ZERO;

        let bytes = s.as_bytes();

        if bytes.len() + 1 > 1 {
            if bytes.len() + 1 < N {
                return Err(Error::TooFewBytes {
                    actual: bytes.len() + 1,
                });
            }

            if bytes.len() + 1 > N {
                return Err(Error::TooManyBytes {
                    actual: bytes.len() + 1,
                    max: N,
                });
            }
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
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.bytes.len()].copy_from_slice(&self.bytes);

        self.bytes.len()
    }
}

impl<const N: usize> Decode for EmptyOrFullCOctetString<N> {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        Self::_ASSERT_NON_ZERO;

        let mut bytes = Vec::with_capacity(N);

        for i in 0..N {
            if i >= src.len() {
                return Err(DecodeError::UnexpectedEof);
            }

            let byte = src[i];

            bytes.push(byte);

            if byte == 0 {
                let len = i + 1;

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

                return Ok((Self { bytes }, len));
            }
        }

        Err(DecodeError::COctetStringDecodeError(
            COctetStringDecodeError::NotNullTerminated,
        ))
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
            let error = EmptyOrFullCOctetString::<7>::new(bytes).unwrap_err();
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
            let error = EmptyOrFullCOctetString::<7>::from_str(string).unwrap_err();
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

    mod to_str {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.to_str().unwrap(), "Hello");
            assert_eq!(string.to_string(), "Hello");
        }
    }

    // TODO: restore
    // mod decode {
    //     use super::*;

    //     #[test]
    //     fn not_null_terminated_empty() {
    //         let bytes = b"";
    //         let error = EmptyOrFullCOctetString::<6>::decode_from(&mut bytes.as_ref()).unwrap_err();

    //         assert!(matches!(
    //             error,
    //             DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
    //         ));
    //     }

    //     #[test]
    //     fn not_null_terminated_empty_not_empty() {
    //         let bytes = b"Hi";
    //         let error = EmptyOrFullCOctetString::<6>::decode_from(&mut bytes.as_ref()).unwrap_err();

    //         assert!(matches!(
    //             error,
    //             DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated)
    //         ));
    //     }

    //     #[test]
    //     fn too_many_bytes() {
    //         let bytes = b"Hello\0";
    //         let error = EmptyOrFullCOctetString::<5>::decode_from(&mut bytes.as_ref()).unwrap_err();

    //         assert!(matches!(
    //             error,
    //             DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotNullTerminated,)
    //         ));
    //     }

    //     #[test]
    //     fn too_few_bytes() {
    //         let bytes = b"Hel\0";
    //         let error = EmptyOrFullCOctetString::<5>::decode_from(&mut bytes.as_ref()).unwrap_err();

    //         assert!(matches!(
    //             error,
    //             DecodeError::COctetStringDecodeError(COctetStringDecodeError::TooFewBytes {
    //                 actual: 4,
    //                 ..
    //             },)
    //         ));
    //     }

    //     #[test]
    //     fn not_ascii() {
    //         let bytes = b"Hell\xF0\0";
    //         let error = EmptyOrFullCOctetString::<6>::decode_from(&mut bytes.as_ref()).unwrap_err();

    //         assert!(matches!(
    //             error,
    //             DecodeError::COctetStringDecodeError(COctetStringDecodeError::NotAscii,)
    //         ));
    //     }

    //     #[test]
    //     fn ok() {
    //         let bytes = b"Hello\0World!";
    //         let buf = &mut bytes.as_ref();
    //         let string = EmptyOrFullCOctetString::<6>::decode_from(buf).unwrap();

    //         assert_eq!(string.bytes, b"Hello\0");
    //         assert_eq!(string.length(), 6);
    //         assert_eq!(buf, b"World!");
    //     }

    //     #[test]
    //     fn ok_empty() {
    //         let bytes = b"\0World!";
    //         let buf = &mut bytes.as_ref();
    //         let string = EmptyOrFullCOctetString::<6>::decode_from(buf).unwrap();

    //         assert_eq!(string.bytes, b"\0");
    //         assert_eq!(string.length(), 1);
    //         assert_eq!(buf, b"World!");
    //     }
    // }
}
