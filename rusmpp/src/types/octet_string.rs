use std::str::FromStr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::io::{
    length::IoLength,
    read::{AsyncIoReadWithLength, AsyncIoReadable, IoReadError, OctetStringIoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Too many bytes. actual: {actual}, max: {max}")]
    TooManyBytes { actual: usize, max: usize },
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
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OctetString<const MAX: usize> {
    bytes: Vec<u8>,
}

impl<const MAX: usize> OctetString<MAX> {
    pub fn empty() -> Self {
        Self { bytes: vec![0] }
    }

    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes.len() > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: MAX,
            });
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes)
    }
}

impl<const MAX: usize> std::fmt::Debug for OctetString<MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MAX: usize> Default for OctetString<MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MAX: usize> FromStr for OctetString<MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.as_bytes())
    }
}

impl<const MAX: usize> ToString for OctetString<MAX> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes).to_string()
    }
}

impl<const MAX: usize> AsRef<[u8]> for OctetString<MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MAX: usize> IoLength for OctetString<MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

#[async_trait::async_trait]
impl<const MAX: usize> AsyncIoWrite for OctetString<MAX> {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        buf.write_all(&self.bytes).await?;

        Ok(self.bytes.len())
    }
}

#[async_trait::async_trait]
impl<const MAX: usize> AsyncIoReadWithLength for OctetString<MAX> {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        if length > MAX {
            return Err(IoReadError::OctetStringIoReadError(
                OctetStringIoReadError::TooManyBytes {
                    actual: length,
                    max: MAX,
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
            let error = OctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 13, .. }));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<13>::new(bytes).unwrap();
            assert_eq!(octet_string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0World!\0";
            let octet_string = OctetString::<13>::new(bytes).unwrap();
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
            let error = OctetString::<6>::async_io_read(&mut bytes.as_ref(), 5)
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::IO { .. }));
        }

        #[tokio::test]
        async fn too_many_bytes() {
            let bytes = b"Hello";
            let error = OctetString::<5>::async_io_read(&mut bytes.as_ref(), 15)
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
        async fn ok_all() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = OctetString::<5>::async_io_read(buf, 5).await.unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(buf, b"");
        }

        #[tokio::test]
        async fn ok_partial() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = OctetString::<5>::async_io_read(buf, 3).await.unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(buf, b"lo");
        }
    }
}