use crate::io::{
    decode::{AsyncDecode, DecodeError},
    encode::{AsyncEncode, EncodeError},
    length::Length,
};
use tokio::io::AsyncWriteExt;

/// An error that can occur when creating a [`EmptyOrFullCOctetString`]
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize },
    NotNullTerminated,
    NotAscii,
    NullByteFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {}, max: {}", actual, max)
            }
            Self::TooFewBytes { actual } => {
                write!(f, "Too few bytes. actual: {}, min: 1", actual)
            }
            Self::NotNullTerminated => write!(f, "Not null terminated"),
            Self::NotAscii => write!(f, "Not ASCII"),
            Self::NullByteFound => write!(f, "Null byte found"),
        }
    }
}

impl std::error::Error for Error {}

/// Empty or full [`COctetString`](struct@crate::types::c_octet_string::COctetString)
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyOrFullCOctetString<const N: usize> {
    bytes: Vec<u8>,
}

impl<const N: usize> EmptyOrFullCOctetString<N> {
    /// Create a new empty [`EmptyOrFullCOctetString`] String
    #[inline]
    pub fn empty() -> Self {
        Self { bytes: vec![0] }
    }

    /// Check if an [`EmptyOrFullCOctetString`] is empty
    ///
    /// An [`EmptyOrFullCOctetString`] is considered empty if it
    /// contains only a single NULL octet (0x00)
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 1
    }

    /// Create a new [`EmptyOrFullCOctetString`] from a sequence of bytes
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

    /// Convert an [`EmptyOrFullCOctetString`] to a &[`str`]
    #[inline]
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
    }

    /// Get the bytes of an [`EmptyOrFullCOctetString`]
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert an [`EmptyOrFullCOctetString`] to a [`Vec`] of [`u8`]
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
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

impl<const N: usize> std::str::FromStr for EmptyOrFullCOctetString<N> {
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

impl<const N: usize> Length for EmptyOrFullCOctetString<N> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const N: usize> AsyncEncode for EmptyOrFullCOctetString<N> {
    async fn encode_to<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes).await?;
        Ok(())
    }
}

impl<const N: usize> AsyncDecode for EmptyOrFullCOctetString<N> {
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        todo!()
    }
}
