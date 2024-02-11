#[derive(Debug)]
pub enum WriteToError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for WriteToError {
    fn from(e: std::io::Error) -> Self {
        WriteToError::IoError(e)
    }
}

impl std::fmt::Display for WriteToError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteToError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for WriteToError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WriteToError::IoError(e) => Some(e),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

trait AsyncWriteTo {
    fn size(&self) -> usize;

    async fn write_to<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), WriteToError>;
}
