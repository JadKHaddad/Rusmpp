use super::no_fixed_size_octet_string::NoFixedSizeOctetString;
use crate::io::{
    length::IoLength,
    read::{AsyncIoReadWithLength, AsyncIoReadable, IoReadError, OctetStringIoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Too many bytes. actual: {actual}, max: {max}")]
    TooManyBytes { actual: usize, max: usize },
    #[error("Too few bytes. actual: {actual}, min: {min}")]
    TooFewBytes { actual: usize, min: usize },
}

/// An Octet String is a sequence of octets not necessarily
/// terminated with a NULL octet. Such fields using Octet
/// String encoding, typically represent fields that can be
/// used to encode raw binary data. In all circumstances, the
/// field will be either a fixed length field or explicit length field
/// where another field indicates the length of the Octet
/// String field. An example of this is the short_message field
/// of the submit_sm PDU that is Octet String encoded and
/// the previous message_length field specifies its length.
///
/// A NULL Octet-String is not encoded. The explicit length
/// field that indicates its length should be set to zero.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OctetString<const MIN: usize, const MAX: usize> {
    bytes: Vec<u8>,
}

impl<const MIN: usize, const MAX: usize> OctetString<MIN, MAX> {
    pub fn empty() -> Self {
        Self { bytes: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes.len() > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: MAX,
            });
        }

        if bytes.len() < MIN {
            return Err(Error::TooFewBytes {
                actual: bytes.len(),
                min: MIN,
            });
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> std::fmt::Debug for OctetString<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> Default for OctetString<MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> FromStr for OctetString<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.as_bytes())
    }
}

impl<const MIN: usize, const MAX: usize> ToString for OctetString<MIN, MAX> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes).to_string()
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<[u8]> for OctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> From<OctetString<MIN, MAX>> for NoFixedSizeOctetString {
    fn from(octet_string: OctetString<MIN, MAX>) -> Self {
        Self::new(octet_string.bytes)
    }
}

impl<const MIN: usize, const MAX: usize> IoLength for OctetString<MIN, MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

#[async_trait::async_trait]
impl<const MIN: usize, const MAX: usize> AsyncIoWrite for OctetString<MIN, MAX> {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        buf.write_all(&self.bytes).await
    }
}

#[async_trait::async_trait]
impl<const MIN: usize, const MAX: usize> AsyncIoReadWithLength for OctetString<MIN, MAX> {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        if length > MAX {
            return Err(IoReadError::OctetStringIoReadError(
                OctetStringIoReadError::TooManyBytes {
                    actual: length,
                    max: MAX,
                },
            ));
        }

        if length < MIN {
            return Err(IoReadError::OctetStringIoReadError(
                OctetStringIoReadError::TooFewBytes {
                    actual: length,
                    min: MIN,
                },
            ));
        }

        let mut bytes = vec![0; length];
        buf.read_exact(&mut bytes).await?;

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
            let bytes = b"Hello\0World!\0";
            let error = OctetString::<0, 5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 13, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hello World!";
            let error = OctetString::<15, 20>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 12, .. }));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<0, 13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes.len(), 13);
            assert_eq!(octet_string.length(), 13);
        }
    }

    mod async_io {
        use crate::io::read::IoReadError;

        use super::*;

        #[tokio::test]
        async fn not_enough_bytes() {
            let bytes = b"";
            let error = OctetString::<0, 6>::async_io_read(&mut bytes.as_ref(), 5)
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::IO { .. }));
        }

        #[tokio::test]
        async fn too_many_bytes() {
            let bytes = b"Hello";
            let error = OctetString::<0, 5>::async_io_read(&mut bytes.as_ref(), 15)
                .await
                .unwrap_err();

            assert!(matches!(
                error,
                IoReadError::OctetStringIoReadError(OctetStringIoReadError::TooManyBytes {
                    actual: 15,
                    ..
                },)
            ));
        }

        #[tokio::test]
        async fn too_few_bytes() {
            let bytes = b"Hello";
            let error = OctetString::<6, 10>::async_io_read(&mut bytes.as_ref(), 5)
                .await
                .unwrap_err();

            assert!(matches!(
                error,
                IoReadError::OctetStringIoReadError(OctetStringIoReadError::TooFewBytes {
                    actual: 5,
                    ..
                },)
            ));
        }

        #[tokio::test]
        async fn ok_all() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = OctetString::<0, 5>::async_io_read(buf, 5).await.unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(buf, b"");
        }

        #[tokio::test]
        async fn ok_partial() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = OctetString::<0, 5>::async_io_read(buf, 3).await.unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(buf, b"lo");
        }
    }
}
