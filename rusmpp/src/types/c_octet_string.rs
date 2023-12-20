use std::str::FromStr;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Too many bytes. actual: {actual}, max: {max}")]
    TooManyBytes { actual: usize, max: usize },
    #[error("Not null terminated")]
    NotNullTerminated,
    #[error("Not ASCII")]
    NotAscii,
    #[error("Null byte found")]
    NullByteFound,
}

/// A C-Octet String is a sequence of ASCII characters
/// terminated with a NULL octet (0x00).
/// The string “Hello” would be encoded in 6 octets (5
/// characters of “Hello” and NULL octet) as follows:
///
/// 0x48656C6C6F00
///
/// Two special variants exist for use within SMPP. These
/// are C-octet String (Decimal) and C-Octet String
/// (Hexadecimal), which are used to carry decimal and
/// hexadecimal digit sequences respectively. These fields
/// are encoded the same way as any ASCII string, but are
/// specifically used to designate decimal and hexadecimal
/// numbers when presented in string format.
///
/// A Decimal C-Octet String “123456789” would be encoded
/// as follows:
///
/// 0x31323334353637383900
///
/// A Hexadecimal C-Octet String “A2F5ED278FC” would be
/// encoded as follows:
///
/// 0x413246354544323738464300
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct COctetString<const MAX: usize> {
    bytes: Vec<u8>,
}

impl<const MAX: usize> COctetString<MAX> {
    pub fn empty() -> Self {
        Self { bytes: vec![0] }
    }

    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes.as_ref().len() > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: MAX,
            });
        }

        if bytes[bytes.len() - 1] != 0 {
            return Err(Error::NotNullTerminated);
        }

        if !bytes.is_ascii() {
            return Err(Error::NotAscii);
        }

        if bytes[..bytes.len() - 1].contains(&0) {
            return Err(Error::NullByteFound);
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
    }
}

impl<const MAX: usize> Default for COctetString<MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MAX: usize> std::fmt::Debug for COctetString<MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("COctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MAX: usize> FromStr for COctetString<MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);

        Self::new(bytes)
    }
}

impl<const MAX: usize> ToString for COctetString<MAX> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes[..self.bytes.len() - 1]).to_string()
    }
}

impl<const MAX: usize> AsRef<[u8]> for COctetString<MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MAX: usize> IoLength for COctetString<MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

#[async_trait::async_trait]
impl<const MAX: usize> AsyncIoWrite for COctetString<MAX> {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        buf.write_all(&self.bytes).await?;

        Ok(self.bytes.len())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IoReadError {
    #[error("IO error: {0}")]
    IO(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("Not null terminated")]
    NotNullTerminated,
    #[error("Not ASCII")]
    NotAscii,
}

#[async_trait::async_trait]
impl<const MAX: usize> AsyncIoRead for COctetString<MAX> {
    type Error = IoReadError;

    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, Self::Error> {
        let mut bytes = Vec::with_capacity(MAX);
        let _ = buf.take(MAX as u64).read_until(0x00, &mut bytes).await?;

        if bytes.last() != Some(&0x00) {
            return Err(Self::Error::NotNullTerminated);
        }

        if !bytes.is_ascii() {
            return Err(Self::Error::NotAscii);
        }

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
            let bytes = b"Hello\0";
            let error = COctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = COctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = COctetString::<6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0o\0";
            let error = COctetString::<6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = COctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = COctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = COctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let bytes = b"\0";
            let string = COctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn too_many_bytes() {
            let string = "Hello";
            let error = COctetString::<5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn null_byte_found() {
            let string = "Hel\0o";
            let error = COctetString::<6>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let string = "Hello";
            let bytes = b"Hello\0";
            let string = COctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let string = "Hello";
            let string = COctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let string = "";
            let bytes = b"\0";
            let string = COctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let string = "";
            let string = COctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod async_io {
        use super::*;

        #[tokio::test]
        async fn not_enough_bytes() {
            let bytes = b"";
            let error = COctetString::<6>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::NotNullTerminated));
        }

        #[tokio::test]
        async fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = COctetString::<5>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::NotNullTerminated));
        }

        #[tokio::test]
        async fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = COctetString::<6>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::NotAscii));
        }

        #[tokio::test]
        async fn ok_max() {
            let bytes = b"Hello\0";
            let string = COctetString::<6>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
        }

        #[tokio::test]
        async fn ok_not_max() {
            let bytes = b"Hello\0";
            let string = COctetString::<25>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
        }

        #[tokio::test]
        async fn ok_empty_max() {
            let bytes = b"\0";
            let string = COctetString::<1>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
        }

        #[tokio::test]
        async fn ok_empty_not_max() {
            let bytes = b"\0";
            let string = COctetString::<25>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
        }

        #[tokio::test]
        async fn ok_remaining() {
            let bytes = b"Hello\0World!";
            let buf = &mut bytes.as_ref();
            let string = COctetString::<25>::async_io_read(buf).await.unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(buf, b"World!");
        }
    }
}
