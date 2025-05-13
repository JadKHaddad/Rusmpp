//! Traits for decoding `SMPP` values.

/// Trait for decoding `SMPP` values from a slice.
///
/// # Implementation
///
/// ```rust
/// use rusmpp::decode::{Decode, DecodeError};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// impl Decode for Foo {
///     fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = u8::decode(&src[index..])?;
///         let index = index + size;
///
///         let (b, size) = u16::decode(&src[index..])?;
///         let index = index + size;
///
///         let (c, size) = u32::decode(&src[index..])?;
///         let index = index + size;
///
///         Ok((Foo { a, b, c }, index))
///     }
/// }
///
/// let buf = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
///
/// let expected = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: 0x04050607,
/// };
///
/// let (foo, size) = Foo::decode(buf).unwrap();
///
/// assert_eq!(size, 7);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[size..], &[0x08]);
/// ```
pub trait Decode: Sized {
    /// Decode a value from a slice.
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding `SMPP` values from a slice with a specified length.
///
/// # Implementation
///
/// ```rust
/// use rusmpp::{
///     decode::{Decode, DecodeError, DecodeWithLength},
///     types::AnyOctetString,
/// };
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: AnyOctetString,
/// }
///
/// impl DecodeWithLength for Foo {
///     fn decode(src: &[u8], length: usize) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = u8::decode(&src[index..])?;
///         let index = index + size;
///
///         let (b, size) = u16::decode(&src[index..])?;
///         let index = index + size;
///
///         let (c, size) = AnyOctetString::decode(&src[index..], length - index)?;
///         let index = index + size;
///
///         Ok((Foo { a, b, c }, index))
///     }
/// }
///
/// // Received over the wire
/// let length = 8;
///
/// let buf = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
///
/// let expected = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: AnyOctetString::new([0x04, 0x05, 0x06, 0x07, 0x08]),
/// };
///
/// let (foo, size) = Foo::decode(buf, length).unwrap();
///
/// assert_eq!(size, 8);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[size..], &[0x09]);
/// ```
pub trait DecodeWithLength: Sized {
    /// Decode a slice from a slice, with a specified length
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding `SMPP` values from a slice with a specified key and length.
///
/// # Implementation
///
/// ```rust
/// use rusmpp::{
///     decode::{Decode, DecodeError, DecodeWithKey, DecodeWithLength},
///     types::AnyOctetString,
/// };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo {
///     A(u16),
///     B(AnyOctetString),
/// }
///
/// impl DecodeWithKey for Foo {
///     type Key = u32;
///
///     fn decode(key: Self::Key, src: &[u8], length: usize) -> Result<(Self, usize), DecodeError> {
///         match key {
///             0x01020304 => {
///                 let (a, size) = u16::decode(src)?;
///
///                 Ok((Foo::A(a), size))
///             }
///             0x04030201 => {
///                 let (b, size) = AnyOctetString::decode(src, length)?;
///
///                 Ok((Foo::B(b), size))
///             }
///             _ => Err(DecodeError::UnsupportedKey { key }),
///         }
///     }
/// }
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is A
/// let buf = &[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, // Value
///     0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = u32::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index).unwrap();
/// let index = index + size;
///
/// let expected = Foo::A(0x0506);
///
/// assert_eq!(size, 2);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is B
/// let buf = &[
///     0x04, 0x03, 0x02, 0x01, // Key
///     0x05, 0x06, 0x07, 0x08, // Value
///     0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = u32::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index).unwrap();
/// let index = index + size;
///
/// let expected = Foo::B(AnyOctetString::new([0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKey: Sized {
    type Key;

    /// Decode a value from a slice, using a key to determine the type.
    fn decode(key: Self::Key, src: &[u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding optional `SMPP` values from a slice with a specified key and length.
///
/// # Implementation
///
/// ```rust
/// use rusmpp::{
///     decode::{Decode, DecodeError, DecodeWithKeyOptional, DecodeWithLength},
///     types::AnyOctetString,
/// };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo {
///     A,
///     B(u16),
///     C(AnyOctetString),
/// }
///
/// impl DecodeWithKeyOptional for Foo {
///     type Key = u32;
///
///     fn decode(
///         key: Self::Key,
///         src: &[u8],
///         length: usize,
///     ) -> Result<Option<(Self, usize)>, DecodeError> {
///         if length == 0 {
///             match key {
///                 0x00000000 => return Ok(Some((Foo::A, 0))),
///                 _ => return Ok(None),
///             }
///         }
///
///         match key {
///             0x01020304 => {
///                 let (a, size) = u16::decode(src)?;
///
///                 Ok(Some((Foo::B(a), size)))
///             }
///             0x04030201 => {
///                 let (b, size) = AnyOctetString::decode(src, length)?;
///
///                 Ok(Some((Foo::C(b), size)))
///             }
///             _ => Err(DecodeError::UnsupportedKey { key }),
///         }
///     }
/// }
///
/// // Received over the wire
/// let length = 4;
///
/// // Key is A
/// let buf = &[
///     0x00, 0x00, 0x00, 0x00, // Key
///     0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = u32::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::A;
///
/// assert_eq!(size, 0);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 4;
///
/// // Key is B, but the received length indicates no value
/// let buf = &[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = u32::decode(buf).unwrap();
/// let index = index + size;
///
/// let value = Foo::decode(key, &buf[index..], length - index).unwrap();
///
/// assert!(value.is_none());
/// assert_eq!(&buf[index..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is B
/// let buf = &[
///     0x01, 0x02, 0x03, 0x04, // Key
///     0x05, 0x06, // Value
///     0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = u32::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::B(0x0506);
///
/// assert_eq!(size, 2);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x07, 0x08, 0x09, 0x0A, 0x0B]);
///
/// // Received over the wire
/// let length = 8;
///
/// // Key is C
/// let buf = &[
///     0x04, 0x03, 0x02, 0x01, // Key
///     0x05, 0x06, 0x07, 0x08, // Value
///     0x09, 0x0A, 0x0B, // Rest
/// ];
///
/// let index = 0;
///
/// let (key, size) = u32::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::C(AnyOctetString::new([0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKeyOptional: Sized {
    type Key;

    /// Decode an optional value from a slice, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError>;
}

/// An error that can occur when decoding `SMPP` values.
#[derive(Debug)]
pub enum DecodeError {
    UnexpectedEof,
    COctetStringDecodeError(COctetStringDecodeError),
    OctetStringDecodeError(OctetStringDecodeError),
    UnsupportedKey { key: u32 },
}

/// An error that can occur when decoding a [`COctetString`](struct@crate::types::COctetString).
#[derive(Debug)]
pub enum COctetStringDecodeError {
    TooFewBytes { actual: usize, min: usize },
    NotAscii,
    NotNullTerminated,
}

/// An error that can occur when decoding an [`OctetString`](struct@crate::types::OctetString).
#[derive(Debug)]
pub enum OctetStringDecodeError {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::UnexpectedEof => write!(f, "Unexpected EOF"),
            DecodeError::COctetStringDecodeError(e) => write!(f, "COctetString error: {e}"),
            DecodeError::OctetStringDecodeError(e) => write!(f, "OctetString error: {e}"),
            DecodeError::UnsupportedKey { key } => write!(f, "Unsupported key: {key}"),
        }
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            DecodeError::UnexpectedEof => None,
            DecodeError::COctetStringDecodeError(e) => Some(e),
            DecodeError::OctetStringDecodeError(e) => Some(e),
            DecodeError::UnsupportedKey { .. } => None,
        }
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
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

pub(crate) trait DecodeExt: Decode {
    fn decode_move(src: &[u8], size: usize) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..]).map(|(this, size_)| (this, size + size_))
    }

    // TODO: test this
    /// Decode a vector of values from a slice with a specified count.
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

    /// Decode a value from a slice.
    ///
    /// If the length is 0, return `None`.
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
    /// Decode a value from a slice, using a key to determine the type.
    ///
    /// If the length is 0, return `None`.
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

// TODO: add tests for the implementation
