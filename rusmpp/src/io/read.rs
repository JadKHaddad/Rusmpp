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
        pub fn into_value(self) -> T {
            self.value
        }

        pub fn into_parts(self) -> (T, usize) {
            (self.value, self.read)
        }
    }
}

pub mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum IoReadError {
        #[error("IO error: {0}")]
        IO(
            #[from]
            #[source]
            std::io::Error,
        ),
        #[error("COctetString error: {0}")]
        COctetStringError(
            #[from]
            #[source]
            IoCOctetStringError,
        ),
        #[error("OctetString error: {0}")]
        OctetStringError(
            #[from]
            #[source]
            IoOctetStringError,
        ),
        #[error("Unknown key: {key}")]
        UnknownKey { key: u32 },
    }

    /// Error when reading a COctetString
    #[derive(thiserror::Error, Debug)]
    pub enum IoCOctetStringError {
        #[error("Not ASCII")]
        NotAscii,
        #[error("Not null terminated")]
        NotNullTerminated,
    }

    /// Error when reading an OctetString
    #[derive(thiserror::Error, Debug)]
    pub enum IoOctetStringError {
        #[error("Too many bytes")]
        TooManyBytes,
    }
}
