use std::{convert::Infallible, str::FromStr};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::io::{
    length::IoLength,
    read::{AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};
/// No fixed size OctetString
///
/// See [`OctetString`](struct@crate::types::octet_string::OctetString)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoFixedSizeOctetString {
    bytes: Vec<u8>,
}

impl NoFixedSizeOctetString {
    pub fn empty() -> Self {
        Self { bytes: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        let bytes = bytes.as_ref();

        Self {
            bytes: bytes.to_vec(),
        }
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

impl FromStr for NoFixedSizeOctetString {
    type Err = Infallible;

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

impl IoLength for NoFixedSizeOctetString {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for NoFixedSizeOctetString {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        buf.write_all(&self.bytes).await
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for NoFixedSizeOctetString {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
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

    mod async_io {
        use crate::io::read::IoReadError;

        use super::*;

        #[tokio::test]
        async fn not_enough_bytes() {
            let bytes = b"";
            let error = NoFixedSizeOctetString::async_io_read(&mut bytes.as_ref(), 5)
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::IO { .. }));
        }

        #[tokio::test]
        async fn ok_all() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = NoFixedSizeOctetString::async_io_read(buf, 5).await.unwrap();

            assert_eq!(string.bytes, b"Hello");
            assert_eq!(string.length(), 5);
            assert_eq!(buf, b"");
        }

        #[tokio::test]
        async fn ok_partial() {
            let bytes = b"Hello";
            let buf = &mut bytes.as_ref();
            let string = NoFixedSizeOctetString::async_io_read(buf, 3).await.unwrap();

            assert_eq!(string.bytes, b"Hel");
            assert_eq!(string.length(), 3);
            assert_eq!(buf, b"lo");
        }
    }
}
