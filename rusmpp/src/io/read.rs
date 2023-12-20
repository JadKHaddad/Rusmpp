use self::result::IoReadResult;

pub type AsyncIoReadable = dyn tokio::io::AsyncBufRead + Send + Unpin;

#[async_trait::async_trait]
pub trait AsyncIoRead
where
    Self: Sized,
{
    async fn async_io_read(buf: &mut AsyncIoReadable) -> IoReadResult<Self>;
}

pub mod result {
    use super::error::IoReadError;

    pub type IoReadResult<T> = Result<IoRead<T>, IoReadError>;

    #[derive(Debug, Clone)]
    pub struct IoRead<T> {
        pub value: T,
        pub read: usize,
    }

    impl<T> IoRead<T> {
        pub fn into_parts(self) -> (T, usize) {
            (self.value, self.read)
        }
    }
}

pub mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum IoReadError {
        #[error("IO error: {0}")]
        IO(#[from] std::io::Error),
        #[error("COctetString error: {0}")]
        COctetStringError(#[source] COctetStringError),
        #[error("OctetString error: {0}")]
        OctetStringError(#[source] OctetStringError),
        #[error("Unknown key: {key}")]
        UnknownKey { key: u32 },
    }

    /// Error when reading a COctetString
    #[derive(thiserror::Error, Debug)]
    pub enum COctetStringError {
        #[error("Too many bytes")]
        TooManyBytes,
        #[error("Too few bytes")]
        TooFewBytes,
        #[error("Not ASCII")]
        NotAscii,
        #[error("Not null terminated")]
        NotNullTerminated,
    }

    /// Error when reading an OctetString
    #[derive(thiserror::Error, Debug)]
    pub enum OctetStringError {
        #[error("Too many bytes")]
        TooManyBytes,
    }
}
