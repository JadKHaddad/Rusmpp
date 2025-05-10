pub trait Decode: Sized {
    /// Decode a value from a slice
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError>;
}

pub trait DecodeWithLength: Sized {
    /// Decode a slice from a reader, with a specified length
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

pub trait DecodeWithKey: Sized {
    type Key;

    /// Decode a value from a slice, using a key to determine the type
    fn decode(key: Self::Key, src: &[u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

pub trait DecodeWithKeyOptional: Sized {
    type Key;

    /// Decode an optional value from a slice, using a key to determine the type
    fn decode(
        key: Self::Key,
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError>;
}

/// An error that can occur when decoding an `SMPP` value
#[derive(Debug)]
pub enum DecodeError {
    UnexpectedEof,
    COctetStringDecodeError(COctetStringDecodeError),
    OctetStringDecodeError(OctetStringDecodeError),
    UnsupportedKey { key: u32 },
}

/// An error that can occur when decoding a [`COctetString`](struct@crate::types::COctetString)
#[derive(Debug)]
pub enum COctetStringDecodeError {
    TooFewBytes { actual: usize, min: usize },
    NotAscii,
    NotNullTerminated,
}

/// An error that can occur when decoding an [`OctetString`](struct@crate::types::OctetString)
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

pub(crate) trait DecodeExt: Decode {
    fn decode_move(src: &[u8], size: usize) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..]).map(|(this, size_)| (this, size + size_))
    }

    // TODO: test this
    /// Decode a vector of values from a slice with a specified count
    fn counted(src: &[u8], count: usize) -> Result<(Vec<Self>, usize), DecodeError> {
        let mut size = 0;

        let mut vec = Vec::with_capacity(count);

        for _ in 0..count {
            let (item, size_) = Self::decode(&src[size..])?;

            size += size_;

            vec.push(item);
        }

        Ok((vec, size))
    }

    // TODO: test this
    fn counted_move(
        src: &[u8],
        count: usize,
        size: usize,
    ) -> Result<(Vec<Self>, usize), DecodeError> {
        Self::counted(&src[size..], count).map(|(vec, size_)| (vec, size + size_))
    }

    /// Decode a value from a slice
    ///
    /// If the length is 0, return `None`
    fn length_checked_decode(
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(src))
            .transpose()
    }

    fn length_checked_decode_move(
        src: &[u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::length_checked_decode(&src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: Decode> DecodeExt for T {}

pub(crate) trait DecodeWithLengthExt: DecodeWithLength {
    fn decode_move(src: &[u8], length: usize, size: usize) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..], length).map(|(this, size_)| (this, size + size_))
    }
}

impl<T: DecodeWithLength> DecodeWithLengthExt for T {}

pub(crate) trait DecodeWithKeyExt: DecodeWithKey {
    // TODO: unused
    // fn decode_move(
    //     key: Self::Key,
    //     src: &mut [u8],
    //     length: usize,
    //     size: usize,
    // ) -> Result<(Self, usize), DecodeError> {
    //     Self::decode(key, &mut src[size..], length).map(|(this, size_)| (this, size + size_))
    // }

    /// Decode a value from a slice, using a key to determine the type
    ///
    /// If the length is 0, return `None`
    fn optional_length_checked_decode(
        key: Self::Key,
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(key, src, length))
            .transpose()
    }

    fn optional_length_checked_decode_move(
        key: Self::Key,
        src: &[u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::optional_length_checked_decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKey> DecodeWithKeyExt for T {}

pub(crate) trait DecodeWithKeyOptionalExt: DecodeWithKeyOptional {
    fn decode_move(
        key: Self::Key,
        src: &[u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<T: DecodeWithKeyOptional> DecodeWithKeyOptionalExt for T {}

const _: () = {
    // TODO: test this
    impl<T: Decode> DecodeWithLength for Vec<T> {
        fn decode(src: &[u8], length: usize) -> Result<(Self, usize), DecodeError> {
            let mut size = 0;

            let mut vec = Vec::with_capacity(length);

            for _ in 0..length {
                let (item, size_) = T::decode(&src[size..])?;

                size += size_;

                vec.push(item);
            }

            Ok((vec, size))
        }
    }
};
