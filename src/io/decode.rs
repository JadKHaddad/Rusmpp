mod error;

pub use self::error::DecodeError;

pub trait AsyncDecode {
    /// Decode a value from a reader
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait AsyncDecodeWithLength {
    /// Decode a value from a reader, with a specified length
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
        length: usize,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait AsyncDecodeWithKey {
    type Key: From<u32> + Into<u32>;

    /// Decode a value from a reader, using a key to determine the type
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized;
}
