use crate::{
    decode::{DecodeError, borrowed::DecodeWithLength},
    encode::{Encode, Length},
};

/// No fixed size [`OctetString`](struct@crate::types::borrowed::octet_string::OctetString).
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
pub struct AnyOctetString<'a> {
    bytes: &'a [u8],
}

impl<'a> AnyOctetString<'a> {
    /// Create a new empty [`AnyOctetString`].
    ///
    /// Equivalent to [`AnyOctetString::empty`].
    #[inline]
    pub const fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`AnyOctetString`].
    #[inline]
    pub const fn empty() -> Self {
        Self { bytes: &[] }
    }

    /// Check if an [`AnyOctetString`] is empty.
    ///
    /// An [`AnyOctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Create a new [`AnyOctetString`] from a sequence of bytes.
    #[inline]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// Convert an [`AnyOctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(self.bytes)
    }

    /// Get the bytes of an [`AnyOctetString`].
    #[inline]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

impl core::fmt::Debug for AnyOctetString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AnyOctetString")
            .field("bytes", &crate::utils::HexFormatter(self.bytes))
            .field("string", &self.to_str().unwrap_or("<invalid utf-8>"))
            .finish()
    }
}

impl Default for AnyOctetString<'_> {
    fn default() -> Self {
        Self::empty()
    }
}

impl core::fmt::Display for AnyOctetString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.to_str().unwrap_or("<invalid utf-8>"))
    }
}

impl Length for AnyOctetString<'_> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl Encode for AnyOctetString<'_> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        _ = &mut dst[..self.bytes.len()].copy_from_slice(self.bytes);

        self.bytes.len()
    }
}

impl<'a> DecodeWithLength<'a> for AnyOctetString<'a> {
    fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), DecodeError> {
        if src.len() < length {
            return Err(DecodeError::unexpected_eof());
        }

        let bytes = &src[..length];

        Ok((Self { bytes }, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::new(bytes);
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = AnyOctetString::new(bytes);
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod decode {
        use crate::decode::DecodeErrorKind;

        use super::*;

        #[test]
        fn unexpected_eof_empty() {
            let bytes = b"";
            let error = AnyOctetString::decode(bytes, 5).unwrap_err();

            assert!(matches!(error.kind(), DecodeErrorKind::UnexpectedEof));
        }

        #[test]
        fn ok_all() {
            let bytes = b"Hello";
            let (string, size) = AnyOctetString::decode(bytes, 5).unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(size, 5);
            assert_eq!(&bytes[size..], b"");
        }

        #[test]
        fn ok_partial() {
            let bytes = b"Hello";
            let (string, size) = AnyOctetString::decode(bytes, 3).unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(size, 3);
            assert_eq!(&bytes[size..], b"lo");
        }
    }
}
