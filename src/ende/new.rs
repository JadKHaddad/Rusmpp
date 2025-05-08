use error::DecodeError;

pub trait Length {
    fn length(&self) -> usize;
}

pub trait Encode: Length {
    /// Encode a value to a slice
    ///
    /// Implementors are allowed to panic if the slice is not big enough to hold the encoded value. If `dst.len()` < [`Length::length`]
    fn encode(&self, dst: &mut [u8]) -> usize;
}

pub trait Decode: Sized {
    /// Decode a value from a slice
    fn decode(src: &mut [u8]) -> Result<(Self, usize), DecodeError>;
}

pub trait DecodeWithLength: Sized {
    /// Decode a slice from a reader, with a specified length
    fn decode(src: &mut [u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

pub trait DecodeWithKey: Sized {
    type Key;

    /// Decode a value from a slice, using a key to determine the type
    fn decode(key: Self::Key, src: &mut [u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

pub trait DecodeWithKeyOptional: Sized {
    type Key;

    /// Decode an optional value from a slice, using a key to determine the type
    fn decode(
        key: Self::Key,
        src: &mut [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError>;
}

pub trait EncodeExt: Encode {
    fn encode_move(&self, dst: &mut [u8], size: usize) -> usize {
        size + self.encode(&mut dst[size..])
    }
}

impl<T: Encode> EncodeExt for T {}

pub trait DecodeExt: Decode {
    fn decode_move(src: &mut [u8], size: usize) -> Result<(Self, usize), DecodeError> {
        Self::decode(&mut src[size..]).map(|(this, size_)| (this, size + size_))
    }

    /// Decode a value from a slice
    ///
    /// If the length is 0, return `None`
    fn length_checked_decode(
        src: &mut [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(src))
            .transpose()
    }

    fn length_checked_decode_move(
        src: &mut [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::length_checked_decode(&mut src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: Decode> DecodeExt for T {}

pub trait DecodeWithLengthExt: DecodeWithLength {
    fn decode_move(
        src: &mut [u8],
        length: usize,
        size: usize,
    ) -> Result<(Self, usize), DecodeError> {
        Self::decode(&mut src[size..], length).map(|(this, size_)| (this, size + size_))
    }
}

impl<T: DecodeWithLength> DecodeWithLengthExt for T {}

pub trait DecodeWithKeyExt: DecodeWithKey {
    fn decode_move(
        key: Self::Key,
        src: &mut [u8],
        length: usize,
        size: usize,
    ) -> Result<(Self, usize), DecodeError> {
        Self::decode(key, &mut src[size..], length).map(|(this, size_)| (this, size + size_))
    }

    /// Decode a value from a slice, using a key to determine the type
    ///
    /// If the length is 0, return `None`
    fn optional_length_checked_decode(
        key: Self::Key,
        src: &mut [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(key, src, length))
            .transpose()
    }

    fn optional_length_checked_decode_move(
        key: Self::Key,
        src: &mut [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::optional_length_checked_decode(key, &mut src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKey> DecodeWithKeyExt for T {}

pub trait DecodeWithKeyOptionalExt: DecodeWithKeyOptional {
    fn decode_move(
        key: Self::Key,
        src: &mut [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::decode(key, &mut src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKeyOptional> DecodeWithKeyOptionalExt for T {}

pub mod error {
    /// An error that can occur when decoding an `SMPP` value
    #[derive(Debug)]
    pub enum DecodeError {
        UnexpectedEof,
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

    impl std::fmt::Display for DecodeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DecodeError::UnexpectedEof => write!(f, "Unexpected EOF"),
                DecodeError::COctetStringDecodeError(e) => write!(f, "COctetString error: {e}"),
                DecodeError::OctetStringDecodeError(e) => write!(f, "OctetString error: {e}"),
                DecodeError::UnsupportedKey { key } => write!(f, "Unsupported key: {key}"),
            }
        }
    }

    impl std::error::Error for DecodeError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                DecodeError::UnexpectedEof => None,
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
                    write!(f, "Too few bytes. actual: {actual}, min: {min}")
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
                    write!(f, "Too many bytes. actual: {actual}, max: {max}")
                }
                OctetStringDecodeError::TooFewBytes { actual, min } => {
                    write!(f, "Too few bytes. actual: {actual}, min: {min}")
                }
            }
        }
    }

    impl std::error::Error for OctetStringDecodeError {}
}

const _: () = {
    impl<T: Length> Length for Vec<T> {
        fn length(&self) -> usize {
            self.iter().map(Length::length).sum()
        }
    }

    impl<T: Length> Length for Option<T> {
        fn length(&self) -> usize {
            match self {
                Some(value) => value.length(),
                None => 0,
            }
        }
    }

    impl<T: Encode> Encode for Option<T> {
        fn encode(&self, dst: &mut [u8]) -> usize {
            match self {
                Some(value) => value.encode(dst),
                None => 0,
            }
        }
    }

    impl<T: Encode> Encode for Vec<T> {
        fn encode(&self, dst: &mut [u8]) -> usize {
            let mut size = 0;

            for item in self {
                size += item.encode_move(dst, size);
            }

            size
        }
    }

    // TODO: I do not like this rework
    impl<T: Decode> DecodeWithLength for Vec<T> {
        fn decode(src: &mut [u8], length: usize) -> Result<(Self, usize), DecodeError> {
            let mut size = 0;

            let mut vec = Vec::with_capacity(length);

            for _ in 0..length {
                let (item, size_) = T::decode_move(src, size)?;

                size += size_;

                vec.push(item);
            }

            Ok((vec, size))
        }
    }
};
