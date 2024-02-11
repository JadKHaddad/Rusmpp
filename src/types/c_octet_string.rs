use crate::io::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

/// An Error that can occur when creating a [`COctetString`]
#[derive(Debug)]
pub enum Error {
    TooManyBytes { actual: usize, max: usize },
    TooFewBytes { actual: usize, min: usize },
    NotNullTerminated,
    NotAscii,
    NullByteFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TooManyBytes { actual, max } => {
                write!(f, "Too many bytes. actual: {}, max: {}", actual, max)
            }
            Error::TooFewBytes { actual, min } => {
                write!(f, "Too few bytes. actual: {}, min: {}", actual, min)
            }
            Error::NotNullTerminated => write!(f, "Not null terminated"),
            Error::NotAscii => write!(f, "Not ASCII"),
            Error::NullByteFound => write!(f, "Null byte found"),
        }
    }
}

impl std::error::Error for Error {}

/// A [`COctetString`] is a sequence of ASCII characters
/// terminated with a NULL octet (0x00).
/// The string “Hello” would be encoded in 6 octets (5
/// characters of “Hello” and NULL octet) as follows:
///
/// 0x48656C6C6F00
///
/// Two special variants exist for use within SMPP. These
/// are [`COctetString`] (Decimal) and [`COctetString`]
/// (Hexadecimal), which are used to carry decimal and
/// hexadecimal digit sequences respectively. These fields
/// are encoded the same way as any ASCII string, but are
/// specifically used to designate decimal and hexadecimal
/// numbers when presented in string format.
///
/// A Decimal [`COctetString`] “123456789” would be encoded
/// as follows:
///
/// 0x31323334353637383900
///
/// A Hexadecimal [`COctetString`] “A2F5ED278FC” would be
/// encoded as follows:
///
/// 0x413246354544323738464300
///
/// A NULL string “” is encoded as 0x00
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct COctetString<const MIN: usize, const MAX: usize> {
    bytes: Vec<u8>,
}

impl<const MIN: usize, const MAX: usize> COctetString<MIN, MAX> {
    /// Create a new empty [`COctetString`] String
    #[inline]
    pub fn empty() -> Self {
        Self { bytes: vec![0] }
    }

    /// Check if a [`COctetString`] is empty
    ///
    /// A [`COctetString`] is considered empty if it
    /// contains only a single NULL octet (0x00)
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 1
    }

    /// Create a new [`COctetString`] from a sequence of bytes
    pub fn new(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes.as_ref().len() > MAX {
            return Err(Error::TooManyBytes {
                actual: bytes.len(),
                max: MAX,
            });
        }

        if bytes.as_ref().len() < MIN {
            return Err(Error::TooFewBytes {
                actual: bytes.len(),
                min: MIN,
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

    /// Convert a [`COctetString`] to a &[`str`]
    #[inline]
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes[..self.bytes.len() - 1])
    }

    /// Get the bytes of a [`COctetString`]
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert a [`COctetString`] to a [`Vec`] of [`u8`]
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> Default for COctetString<MIN, MAX> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<const MIN: usize, const MAX: usize> std::fmt::Debug for COctetString<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("COctetString")
            .field("bytes", &self.bytes)
            .field("string", &self.to_string())
            .finish()
    }
}

impl<const MIN: usize, const MAX: usize> std::str::FromStr for COctetString<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);

        Self::new(bytes)
    }
}

impl<const MIN: usize, const MAX: usize> ToString for COctetString<MIN, MAX> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes[..self.bytes.len() - 1]).to_string()
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<[u8]> for COctetString<MIN, MAX> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const MIN: usize, const MAX: usize> Length for COctetString<MIN, MAX> {
    fn length(&self) -> usize {
        self.bytes.len()
    }
}

impl<const MIN: usize, const MAX: usize> Encode for COctetString<MIN, MAX> {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}

impl<const MIN: usize, const MAX: usize> Decode for COctetString<MIN, MAX> {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        todo!()
    }
}
