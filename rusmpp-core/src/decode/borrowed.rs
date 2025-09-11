//! Traits for decoding `SMPP` values with borrowed data.

use crate::decode::DecodeError;

/// Trait for decoding `SMPP` values from a slice.
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::decode::{borrowed::Decode, DecodeError};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// impl<'a> Decode<'a> for Foo {
///     fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         let (b, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         let (c, size) = Decode::decode(&src[index..])?;
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
pub trait Decode<'a>: 'a + Sized {
    /// Decode a value from a slice.
    fn decode(src: &'a [u8]) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding `SMPP` values from a slice with a specified length.
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::{
/// #     decode::{borrowed::{Decode, DecodeWithLength}, DecodeError},
/// #     types::borrowed::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo<'a> {
///     a: u8,
///     b: u16,
///     c: AnyOctetString<'a>,
/// }
///
/// impl<'a> DecodeWithLength<'a> for Foo<'a> {
///     fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
///         let index = 0;
///
///         let (a, size) = Decode::decode(&src[index..])?;
///         let index = index + size;
///
///         let (b, size) = Decode::decode(&src[index..])?;
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
///     c: AnyOctetString::new(&[0x04, 0x05, 0x06, 0x07, 0x08]),
/// };
///
/// let (foo, size) = Foo::decode(buf, length).unwrap();
///
/// assert_eq!(size, 8);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[size..], &[0x09]);
/// ```
pub trait DecodeWithLength<'a>: 'a + Sized {
    /// Decode a value from a slice, with a specified length
    fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Everything that implements [`Decode`] also implements [`DecodeWithLength`] by ignoring the length.
impl<'a, T: Decode<'a>> DecodeWithLength<'a> for T {
    fn decode(src: &'a [u8], _length: usize) -> Result<(Self, usize), DecodeError> {
        Decode::decode(src)
    }
}

/// Trait for decoding `SMPP` values from a slice with a specified key and length.
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::{
/// #     decode::{borrowed::{Decode, DecodeWithKey, DecodeWithLength}, DecodeError},
/// #     types::borrowed::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo<'a> {
///     A(u16),
///     B(AnyOctetString<'a>),
/// }
///
/// impl<'a> DecodeWithKey<'a> for Foo<'a> {
///     type Key = u32;
///
///     fn decode(key: Self::Key, src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
///         match key {
///             0x01020304 => {
///                 let (a, size) = Decode::decode(src)?;
///
///                 Ok((Foo::A(a), size))
///             }
///             0x04030201 => {
///                 let (b, size) = AnyOctetString::decode(src, length)?;
///
///                 Ok((Foo::B(b), size))
///             }
///             _ => Err(DecodeError::unsupported_key(key)),
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
/// let (key, size) = Decode::decode(buf).unwrap();
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
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index).unwrap();
/// let index = index + size;
///
/// let expected = Foo::B(AnyOctetString::new(&[0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKey<'a>: 'a + Sized {
    type Key;

    /// Decode a value from a slice, using a key to determine the type.
    fn decode(key: Self::Key, src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError>;
}

/// Trait for decoding optional `SMPP` values from a slice with a specified key and length.
///
/// # Implementation
///
/// ```rust
/// # use rusmpp_core::{
/// #     decode::{borrowed::{Decode, DecodeWithKeyOptional, DecodeWithLength}, DecodeError},
/// #     types::borrowed::AnyOctetString,
/// # };
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum Foo<'a> {
///     A,
///     B(u16),
///     C(AnyOctetString<'a>),
/// }
///
/// impl<'a> DecodeWithKeyOptional<'a> for Foo<'a> {
///     type Key = u32;
///
///     fn decode(
///         key: Self::Key,
///         src: &'a [u8],
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
///                 let (a, size) = Decode::decode(src)?;
///
///                 Ok(Some((Foo::B(a), size)))
///             }
///             0x04030201 => {
///                 let (b, size) = AnyOctetString::decode(src, length)?;
///
///                 Ok(Some((Foo::C(b), size)))
///             }
///             _ => Err(DecodeError::unsupported_key(key)),
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
/// let (key, size) = Decode::decode(buf).unwrap();
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
/// let (key, size) = Decode::decode(buf).unwrap();
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
/// let (key, size) = Decode::decode(buf).unwrap();
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
/// let (key, size) = Decode::decode(buf).unwrap();
/// let index = index + size;
///
/// let (foo, size) = Foo::decode(key, &buf[index..], length - index)
///     .unwrap()
///     .unwrap();
/// let index = index + size;
///
/// let expected = Foo::C(AnyOctetString::new(&[0x05, 0x06, 0x07, 0x08]));
///
/// assert_eq!(size, 4);
/// assert_eq!(foo, expected);
/// assert_eq!(&buf[index..], &[0x09, 0x0A, 0x0B]);
/// ```
pub trait DecodeWithKeyOptional<'a>: 'a + Sized {
    type Key;

    /// Decode an optional value from a slice, using a key to determine the type.
    fn decode(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError>;
}

#[doc(hidden)]
pub trait DecodeExt<'a>: Decode<'a> {
    fn decode_move(src: &'a [u8], size: usize) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..]).map(|(this, size_)| (this, size + size_))
    }

    /// Decode a value from a slice.
    ///
    /// If the length is 0, return `None`.
    fn length_checked_decode(
        src: &'a [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(src))
            .transpose()
    }

    fn length_checked_decode_move(
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::length_checked_decode(&src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<'a, T: Decode<'a>> DecodeExt<'a> for T {}

#[doc(hidden)]
pub trait DecodeWithLengthExt<'a>: DecodeWithLength<'a> {
    fn decode_move(
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<(Self, usize), DecodeError> {
        Self::decode(&src[size..], length).map(|(this, size_)| (this, size + size_))
    }
}

impl<'a, T: DecodeWithLength<'a>> DecodeWithLengthExt<'a> for T {}

#[doc(hidden)]
pub trait DecodeWithKeyExt<'a>: DecodeWithKey<'a> {
    /// Decode a value from a slice, using a key to determine the type.
    ///
    /// If the length is 0, return `None`.
    fn optional_length_checked_decode(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        (length > 0)
            .then_some(())
            .map(|_| Self::decode(key, src, length))
            .transpose()
    }

    fn optional_length_checked_decode_move(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::optional_length_checked_decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<'a, T: DecodeWithKey<'a>> DecodeWithKeyExt<'a> for T {}

#[doc(hidden)]
pub trait DecodeWithKeyOptionalExt<'a>: DecodeWithKeyOptional<'a> {
    fn decode_move(
        key: Self::Key,
        src: &'a [u8],
        length: usize,
        size: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        Self::decode(key, &src[size..], length)
            .map(|decoded| decoded.map(|(this, size_)| (this, size + size_)))
    }
}

impl<'a, T: DecodeWithKeyOptional<'a>> DecodeWithKeyOptionalExt<'a> for T {}
