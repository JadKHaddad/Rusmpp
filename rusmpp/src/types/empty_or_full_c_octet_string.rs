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
    #[error("Too few bytes. actual: {actual}, min: 1")]
    TooFewBytes { actual: usize },
    #[error("Not null terminated")]
    NotNullTerminated,
    #[error("Not ASCII")]
    NotAscii,
    #[error("Null byte found")]
    NullByteFound,
}

/// Empty or full COctetString
///
/// See [`COctetString`](struct@crate::types::c_octet_string::COctetString)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyOrFullCOctetString<const N: usize> {
    bytes: Vec<u8>,
}

impl<const N: usize> EmptyOrFullCOctetString<N> {
    pub fn empty() -> Self {
        Self { bytes: vec![0] }
    }

    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes[bytes.len() - 1] != 0 {
            return Err(Error::NotNullTerminated);
        }

        if !bytes.is_ascii() {
            return Err(Error::NotAscii);
        }

        if bytes[..bytes.len() - 1].contains(&0) {
            return Err(Error::NullByteFound);
        }

        if bytes.len() > 1 {
            if bytes.len() < N {
                return Err(Error::TooFewBytes {
                    actual: bytes.len(),
                });
            }

            if bytes.len() > N {
                return Err(Error::TooManyBytes {
                    actual: bytes.len(),
                    max: N,
                });
            }
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
    }
}

impl<const N: usize> std::fmt::Debug for EmptyOrFullCOctetString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const N: usize> Default for EmptyOrFullCOctetString<N> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const N: usize> FromStr for EmptyOrFullCOctetString<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);

        Self::new(bytes)
    }
}

impl<const N: usize> ToString for EmptyOrFullCOctetString<N> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes[..self.bytes.len() - 1]).to_string()
    }
}

impl<const N: usize> AsRef<[u8]> for EmptyOrFullCOctetString<N> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const N: usize> IoLength for EmptyOrFullCOctetString<N> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

#[async_trait::async_trait]
impl<const N: usize> AsyncIoWrite for EmptyOrFullCOctetString<N> {
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
    #[error("Too few bytes. actual: {actual}, min: 1")]
    TooFewBytes { actual: usize },
    #[error("Not ASCII")]
    NotAscii,
}

#[async_trait::async_trait]
impl<const N: usize> AsyncIoRead for EmptyOrFullCOctetString<N> {
    type Error = IoReadError;

    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, Self::Error> {
        let mut bytes = Vec::with_capacity(N);
        let _ = buf.take(N as u64).read_until(0x00, &mut bytes).await?;

        if bytes.last() != Some(&0x00) {
            return Err(Self::Error::NotNullTerminated);
        }

        if bytes.len() > 1 && bytes.len() < N {
            return Err(Self::Error::TooFewBytes {
                actual: bytes.len(),
            });
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
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let bytes = b"Hel\0";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 4 }));
        }

        #[test]
        fn not_null_terminated() {
            let bytes = b"Hello";
            let error = EmptyOrFullCOctetString::<5>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotNullTerminated));
        }

        #[test]
        fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = EmptyOrFullCOctetString::<6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NotAscii));
        }

        #[test]
        fn null_byte_found() {
            let bytes = b"Hel\0lo\0";
            let error = EmptyOrFullCOctetString::<6>::new(bytes).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::new(bytes).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn too_many_bytes() {
            let string = "Hello";
            let error = EmptyOrFullCOctetString::<5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooManyBytes { actual: 6, .. }));
        }

        #[test]
        fn too_few_bytes() {
            let string = "Hel";
            let error = EmptyOrFullCOctetString::<5>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::TooFewBytes { actual: 4 }));
        }

        #[test]
        fn null_byte_found() {
            let string = "Hel\0lo";
            let error = EmptyOrFullCOctetString::<6>::from_str(string).unwrap_err();
            assert!(matches!(error, Error::NullByteFound));
        }

        #[test]
        fn ok() {
            let string = "Hello";
            let bytes = b"Hello\0";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_len() {
            let string = "Hello";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 6);
            assert_eq!(string.length(), 6);
        }

        #[test]
        fn ok_empty() {
            let string = "";
            let bytes = b"\0";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes, bytes);
        }

        #[test]
        fn ok_empty_len() {
            let string = "";
            let string = EmptyOrFullCOctetString::<6>::from_str(string).unwrap();
            assert_eq!(string.bytes.len(), 1);
            assert_eq!(string.length(), 1);
        }
    }

    mod async_io {
        use super::*;

        #[tokio::test]
        async fn not_enough_bytes() {
            let bytes = b"";
            let error = EmptyOrFullCOctetString::<6>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::NotNullTerminated));
        }

        #[tokio::test]
        async fn too_many_bytes() {
            let bytes = b"Hello\0";
            let error = EmptyOrFullCOctetString::<5>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::NotNullTerminated));
        }

        #[tokio::test]
        async fn too_few_bytes() {
            let bytes = b"Hel\0";
            let error = EmptyOrFullCOctetString::<5>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::TooFewBytes { actual: 4 }));
        }

        #[tokio::test]
        async fn not_ascii() {
            let bytes = b"Hell\xF0\0";
            let error = EmptyOrFullCOctetString::<6>::async_io_read(&mut bytes.as_ref())
                .await
                .unwrap_err();

            assert!(matches!(error, IoReadError::NotAscii));
        }

        #[tokio::test]
        async fn ok() {
            let bytes = b"Hello\0World!";
            let buf = &mut bytes.as_ref();
            let string = EmptyOrFullCOctetString::<6>::async_io_read(buf)
                .await
                .unwrap();

            assert_eq!(string.bytes, b"Hello\0");
            assert_eq!(string.length(), 6);
            assert_eq!(buf, b"World!");
        }

        #[tokio::test]
        async fn ok_empty() {
            let bytes = b"\0World!";
            let buf = &mut bytes.as_ref();
            let string = EmptyOrFullCOctetString::<6>::async_io_read(buf)
                .await
                .unwrap();

            assert_eq!(string.bytes, b"\0");
            assert_eq!(string.length(), 1);
            assert_eq!(buf, b"World!");
        }
    }
}
