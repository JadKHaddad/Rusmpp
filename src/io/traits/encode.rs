#[derive(Debug)]
pub enum EncodeError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for EncodeError {
    fn from(e: std::io::Error) -> Self {
        EncodeError::IoError(e)
    }
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EncodeError::IoError(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

trait AsyncEncode {
    async fn read_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, EncodeError>
    where
        Self: Sized;
}
