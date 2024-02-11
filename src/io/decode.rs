mod error;

pub use self::error::DecodeError;

pub trait Decode {
    /// Decode a value from a reader
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait DecodeWithLength {
    /// Decode a value from a reader, with a specified length
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait DecodeWithKey {
    type Key: From<u32> + Into<u32>;

    /// Decode a value from a reader, using a key to determine the type
    fn decode_from<R: std::io::Read>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized;
}
