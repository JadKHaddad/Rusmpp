use crate::ende::{
    decode::{DecodeError, DecodeWithLength},
    encode::{Encode, EncodeError},
    length::Length,
};

/// No fixed size [`OctetString`](struct@crate::types::octet_string::OctetString)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnyOctetString {
    #[cfg(feature = "alloc")]
    bytes: Vec<u8>,

    // TODO: what about this 255
    #[cfg(not(feature = "alloc"))]
    bytes: heapless::Vec<u8, 255>,
}

impl AnyOctetString {
    /// Create a new empty [`AnyOctetString`].
    ///
    /// Equivalent to [`AnyOctetString::empty`].
    #[inline]
    pub fn null() -> Self {
        Self::empty()
    }

    /// Create a new empty [`AnyOctetString`].
    #[inline]
    pub fn empty() -> Self {
        #[cfg(feature = "alloc")]
        return Self { bytes: vec![] };

        #[cfg(not(feature = "alloc"))]
        Self {
            bytes: heapless::Vec::new(),
        }
    }

    /// Check if an [`AnyOctetString`] is empty.
    ///
    /// An [`AnyOctetString`] is considered empty if it
    /// contains no octets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Create a new [`AnyOctetString`] from a sequence of bytes.
    #[inline]
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        let bytes = bytes.as_ref();

        #[cfg(feature = "alloc")]
        let bytes = bytes.to_vec();

        // TODO: This must return an error
        #[cfg(not(feature = "alloc"))]
        let bytes = {
            let mut heapless_bytes = heapless::Vec::<u8, 255>::new();

            heapless_bytes
                .extend_from_slice(bytes)
                .expect("bytes.len() must not be greater than 255");

            heapless_bytes
        };

        Self { bytes }
    }

    /// Convert an [`AnyOctetString`] to a &[`str`].
    #[inline]
    pub fn to_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes)
    }

    /// Get the bytes of an [`AnyOctetString`].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`AnyOctetString`] to a [`Vec`] of [`u8`].
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl core::fmt::Debug for AnyOctetString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "alloc")]
        {
            f.debug_struct("AnyOctetString")
                .field("bytes", &crate::utils::HexFormatter(&self.bytes))
                .field("string", &self.to_string())
                .finish()
        }

        #[cfg(not(feature = "alloc"))]
        f.debug_struct("AnyOctetString")
            .field("bytes", &crate::utils::HexFormatter(&self.bytes))
            .finish()
    }
}

impl Default for AnyOctetString {
    fn default() -> Self {
        Self::empty()
    }
}

impl core::str::FromStr for AnyOctetString {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.as_bytes()))
    }
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for AnyOctetString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.bytes))
    }
}

impl AsRef<[u8]> for AnyOctetString {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Length for AnyOctetString {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl Encode for AnyOctetString {
    fn encode_to<W: crate::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl DecodeWithLength for AnyOctetString {
    fn decode_from<R: crate::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        // TODO: must use length.

        #[cfg(feature = "alloc")]
        let mut bytes = vec![0; length];

        #[cfg(not(feature = "alloc"))]
        let mut bytes = heapless::Vec::<u8, 255>::new();

        reader.read_exact(&mut bytes)?;

        Ok(Self { bytes })
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
        use super::*;

        #[test]
        fn not_enough_bytes() {
            let bytes = b"";
            let error = AnyOctetString::decode_from(&mut bytes.as_ref(), 5).unwrap_err();

            assert!(matches!(error, DecodeError::IoError { .. }));
        }

        #[test]
        fn ok_all() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = AnyOctetString::decode_from(buf, 5).unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(buf, b"");
        }

        #[test]
        fn ok_partial() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = AnyOctetString::decode_from(buf, 3).unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(buf, b"lo");
        }
    }
}
