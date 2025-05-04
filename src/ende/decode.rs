mod error;

use crate::tri;

pub use self::error::*;

pub trait Decode: std::fmt::Debug {
    /// Decode a value from a reader
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized;

    /// Decode a value from a reader
    ///
    /// If the length is 0, return `None`
    fn length_checked_decode_from<R: std::io::Read>(
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized,
    {
        if length == 0 {
            return Ok(None);
        }

        Self::decode_from(reader).map(Some)
    }

    /// Decode a vector of values from a reader
    fn vectorized_decode_from<R: std::io::Read>(
        reader: &mut R,
        count: usize,
    ) -> Result<Vec<Self>, DecodeError>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();

        for _ in 0..count {
            let v = tri!(Self::decode_from(reader));
            vec.push(v);
        }

        Ok(vec)
    }

    /// Decode a value from a slice
    #[allow(clippy::useless_asref)]
    fn decode_from_slice(slice: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        Self::decode_from(&mut slice.as_ref())
    }
}

pub trait DecodeWithLength {
    /// Decode a value from a reader, with a specified length
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized;

    /// Decode a value from a slice, with a specified length
    #[allow(clippy::useless_asref)]
    fn decode_from_slice(slice: &[u8], length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        Self::decode_from(&mut slice.as_ref(), length)
    }
}

pub trait DecodeWithKey {
    type Key;

    /// Decode a value from a reader, using a key to determine the type
    fn decode_from<R: std::io::Read>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;

    /// Decode a value from a reader, using a key to determine the type
    ///
    /// If the length is 0, return `None`
    fn optional_length_checked_decode_from<R: std::io::Read>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized,
    {
        if length == 0 {
            return Ok(None);
        }

        Self::decode_from(key, reader, length).map(Some)
    }
}

pub trait DecodeWithKeyOptional {
    type Key;

    /// Decode a value from a reader, using a key to determine the type
    fn decode_from<R: std::io::Read>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized;
}
