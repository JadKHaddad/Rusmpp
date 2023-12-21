use super::length::IoLength;

pub type AsyncIoReadable = dyn tokio::io::AsyncBufRead + Send + Unpin;

#[async_trait::async_trait]
pub trait AsyncIoRead
where
    Self: Sized + IoLength,
{
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError>;
}

#[async_trait::async_trait]
pub trait AsyncIoReadWithLength
where
    Self: Sized + IoLength,
{
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError>;
}

#[async_trait::async_trait]
pub trait AsyncIoReadWithKey
where
    Self: Sized + IoLength,
{
    type Key;

    async fn async_io_read(
        key: Self::Key,
        buf: &mut AsyncIoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError>;
}

#[derive(thiserror::Error, Debug)]
pub enum IoReadError {
    #[error("IO error: {0}")]
    IO(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("COctetString error: {0}")]
    COctetStringIoReadError(#[source] COctetStringIoReadError),
    #[error("OctetString error: {0}")]
    OctetStringIoReadError(#[source] OctetStringIoReadError),
    #[error("Unknown key: {key}")]
    UnknownKey { key: u32 },
}

/// Error when deserializing a COctetString
#[derive(thiserror::Error, Debug)]
pub enum COctetStringIoReadError {
    #[error("Too few bytes. actual: {actual}, min: 1")]
    TooFewBytes { actual: usize },
    #[error("Not ASCII")]
    NotAscii,
    #[error("Not null terminated")]
    NotNullTerminated,
}

/// Error when deserializing a OctetString
#[derive(thiserror::Error, Debug)]
pub enum OctetStringIoReadError {
    #[error("Too many bytes. actual: {actual}, max: {max}")]
    TooManyBytes { actual: usize, max: usize },
}
