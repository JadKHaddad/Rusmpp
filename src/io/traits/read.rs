#[derive(Debug)]
pub enum ReadFromError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for ReadFromError {
    fn from(e: std::io::Error) -> Self {
        ReadFromError::IoError(e)
    }
}

impl std::fmt::Display for ReadFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadFromError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for ReadFromError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadFromError::IoError(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

trait AsyncReadFrom {
    async fn read_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, ReadFromError>
    where
        Self: Sized;
}
