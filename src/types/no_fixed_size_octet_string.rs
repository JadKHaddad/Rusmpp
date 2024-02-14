use crate::ende::{
    decode::{DecodeError, DecodeWithLength},
    encode::{Encode, EncodeError},
    length::Length,
};

/// No fixed size [`OctetString`](struct@crate::types::octet_string::OctetString)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoFixedSizeOctetString {
    bytes: Vec<u8>,
}

impl NoFixedSizeOctetString {
    /// Create a new empty [`NoFixedSizeOctetString`]
    #[inline]
    pub fn empty() -> Self {
        Self { bytes: vec![] }
    }

    /// Check if a [`NoFixedSizeOctetString`] is empty
    ///
    /// A [`NoFixedSizeOctetString`] is considered empty if it
    /// contains no octets
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Create a new [`NoFixedSizeOctetString`] from a sequence of bytes
    #[inline]
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        let bytes = bytes.as_ref();

        Self {
            bytes: bytes.to_vec(),
        }
    }

    /// Convert a [`NoFixedSizeOctetString`] to a &[`str`]
    #[inline]
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes)
    }

    /// Get the bytes of a [`NoFixedSizeOctetString`]
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert a [`NoFixedSizeOctetString`] to a [`Vec`] of [`u8`]
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl std::fmt::Debug for NoFixedSizeOctetString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoFixedSizeOctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl Default for NoFixedSizeOctetString {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::str::FromStr for NoFixedSizeOctetString {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.as_bytes()))
    }
}

impl ToString for NoFixedSizeOctetString {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes).to_string()
    }
}

impl AsRef<[u8]> for NoFixedSizeOctetString {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Length for NoFixedSizeOctetString {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl Encode for NoFixedSizeOctetString {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl DecodeWithLength for NoFixedSizeOctetString {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut bytes = vec![0; length];
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
            let octet_string = NoFixedSizeOctetString::new(bytes);
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = NoFixedSizeOctetString::new(bytes);
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn not_enough_bytes() {
            let bytes = b"";
            let error = NoFixedSizeOctetString::decode_from(&mut bytes.as_ref(), 5).unwrap_err();

            assert!(matches!(error, DecodeError::IoError { .. }));
        }

        #[test]
        fn ok_all() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = NoFixedSizeOctetString::decode_from(buf, 5).unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(buf, b"");
        }

        #[test]
        fn ok_partial() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = NoFixedSizeOctetString::decode_from(buf, 3).unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(buf, b"lo");
        }
    }
}
