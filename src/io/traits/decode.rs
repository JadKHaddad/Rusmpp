#[derive(Debug)]
pub enum DecodeError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for DecodeError {
    fn from(e: std::io::Error) -> Self {
        DecodeError::IoError(e)
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodeError::IoError(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

pub trait AsyncDecode {
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait AsyncDecodeWithLength {
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
        length: usize,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait AsyncDecodeWithKey {
    type Key: From<u32> + Into<u32>;

    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized;
}
