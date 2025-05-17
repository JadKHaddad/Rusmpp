//! Traits for encoding `SMPP` values.

/// Trait for determining the length of `SMPP` values.
///
/// # Implementation
///
/// ```rust
/// use rusmpp::encode::{Encode, Length};
///
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// impl Length for Foo {
///     fn length(&self) -> usize {
///         self.a.length() + self.b.length() + self.c.length()
///     }
/// }
///
/// let foo = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: 0x04050607,
/// };
///
///
/// assert_eq!(foo.length(), 7);
/// ```
pub trait Length {
    fn length(&self) -> usize;
}

/// Trait for encoding `SMPP` values into a slice.
///
/// # Implementation
///
/// ```rust
/// use rusmpp::encode::{Encode, Length};
///
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: u32,
/// }
///
/// impl Length for Foo {
///     fn length(&self) -> usize {
///         self.a.length() + self.b.length() + self.c.length()
///     }
/// }
///
/// impl Encode for Foo {
///     fn encode(&self, dst: &mut [u8]) -> usize {
///         let mut size = 0;
///
///         size += self.a.encode(&mut dst[size..]);
///         size += self.b.encode(&mut dst[size..]);
///         size += self.c.encode(&mut dst[size..]);
///
///         size
///     }
/// }
///
/// let foo = Foo {
///     a: 0x01,
///     b: 0x0203,
///     c: 0x04050607,
/// };
///
/// let buf = &mut [0u8; 1024];
///
/// assert!(buf.len() >= foo.length());
///
/// let size = foo.encode(buf);
///
/// let expected = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
///
/// assert_eq!(size, 7);
/// assert_eq!(&buf[..size], expected);
/// ```
pub trait Encode: Length {
    /// Encode a value to a slice
    ///
    /// Implementors are allowed to panic if the slice is not big enough to hold the encoded value. If `dst.len()` < [`Length::length`]
    fn encode(&self, dst: &mut [u8]) -> usize;
}

pub(crate) trait EncodeExt: Encode {
    fn encode_move(&self, dst: &mut [u8], size: usize) -> usize {
        size + self.encode(&mut dst[size..])
    }
}

impl<T: Encode> EncodeExt for T {}

impl<T: Length> Length for Option<T> {
    fn length(&self) -> usize {
        self.as_ref().map(Length::length).unwrap_or(0)
    }
}

impl<T: Length> Length for alloc::vec::Vec<T> {
    fn length(&self) -> usize {
        self.iter().map(Length::length).sum()
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        self.as_ref().map(|item| item.encode(dst)).unwrap_or(0)
    }
}

impl<T: Encode> Encode for alloc::vec::Vec<T> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        self.iter()
            .fold(0, |acc, item| acc + item.encode(&mut dst[acc..]))
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{AnyOctetString, COctetString, EmptyOrFullCOctetString, OctetString};

    use super::*;

    #[test]
    fn length_option() {
        let value: Option<u8> = Some(0u8);
        assert_eq!(value.length(), 1);

        let value: Option<u8> = None;
        assert_eq!(value.length(), 0);
    }

    #[test]
    fn length_vec() {
        let values: alloc::vec::Vec<u8> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(values.length(), 10);

        let values: alloc::vec::Vec<u16> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(values.length(), 20);

        let values: alloc::vec::Vec<u32> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(values.length(), 40);

        let values = alloc::vec![AnyOctetString::new(b"Hello"), AnyOctetString::new(b"World")];
        assert_eq!(values.length(), 10);

        let values = alloc::vec![
            COctetString::<1, 6>::new(b"Hello\0").unwrap(),
            COctetString::<1, 6>::new(b"World\0").unwrap(),
        ];
        assert_eq!(values.length(), 12);

        let values = alloc::vec![
            EmptyOrFullCOctetString::<6>::new(b"Hello\0").unwrap(),
            EmptyOrFullCOctetString::<6>::new(b"World\0").unwrap(),
        ];
        assert_eq!(values.length(), 12);

        let values = alloc::vec![
            OctetString::<0, 5>::new(b"Hello").unwrap(),
            OctetString::<0, 5>::new(b"World").unwrap(),
        ];
        assert_eq!(values.length(), 10);
    }

    #[test]
    fn encode_option() {
        let buf = &mut [0u8; 1024];

        let value: Option<u8> = Some(0u8);
        assert!(buf.len() >= value.length());

        let size = value.encode(buf);

        assert_eq!(size, 1);
        assert_eq!(&buf[..size], &[0]);

        let value: Option<u8> = None;
        assert!(buf.len() >= value.length());

        let size = value.encode(buf);

        assert_eq!(size, 0);
    }

    #[test]
    fn encode_vec() {
        let buf = &mut [0u8; 1024];

        let values: alloc::vec::Vec<u8> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(buf.len() >= values.length());

        let size = values.encode(buf);

        assert_eq!(size, 10);
        assert_eq!(&buf[..size], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let values: alloc::vec::Vec<u16> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(buf.len() >= values.length());

        let size = values.encode(buf);

        assert_eq!(size, 20);
        assert_eq!(
            &buf[..size],
            &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9]
        );

        let values: alloc::vec::Vec<u32> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(buf.len() >= values.length());

        let size = values.encode(buf);
        assert_eq!(size, 40);

        assert_eq!(
            &buf[..size],
            &[
                0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6,
                0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9
            ]
        );

        let values = alloc::vec![AnyOctetString::new(b"Hello"), AnyOctetString::new(b"World")];
        assert!(buf.len() >= values.length());

        let size = values.encode(buf);

        assert_eq!(size, 10);
        assert_eq!(&buf[..size], b"HelloWorld");

        let values = alloc::vec![
            COctetString::<1, 6>::new(b"Hello\0").unwrap(),
            COctetString::<1, 6>::new(b"World\0").unwrap(),
        ];
        assert!(buf.len() >= values.length());

        let size = values.encode(buf);

        assert_eq!(size, 12);
        assert_eq!(&buf[..size], b"Hello\0World\0");

        let values = alloc::vec![
            EmptyOrFullCOctetString::<6>::new(b"Hello\0").unwrap(),
            EmptyOrFullCOctetString::<6>::new(b"World\0").unwrap(),
        ];
        assert!(buf.len() >= values.length());

        let size = values.encode(buf);

        assert_eq!(size, 12);
        assert_eq!(&buf[..size], b"Hello\0World\0");

        let values = alloc::vec![
            OctetString::<0, 5>::new(b"Hello").unwrap(),
            OctetString::<0, 5>::new(b"World").unwrap(),
        ];
        assert!(buf.len() >= values.length());

        let size = values.encode(buf);

        assert_eq!(size, 10);
        assert_eq!(&buf[..size], b"HelloWorld");
    }
}
