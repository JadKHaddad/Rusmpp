use super::length::IoLength;

pub type AsyncIoReadable = dyn tokio::io::AsyncBufRead + Send + Unpin;
pub type IoReadable = dyn std::io::BufRead;

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
pub trait AsyncIoReadWithKeyOptional
where
    Self: Sized + IoLength,
{
    type Key: From<u32> + Into<u32>;

    async fn async_io_read(
        key: Self::Key,
        buf: &mut AsyncIoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError>;
}

pub trait IoRead
where
    Self: Sized + IoLength,
{
    fn io_read(buf: &mut IoReadable) -> Result<Self, IoReadError>;
}

pub trait IoReadWithLength
where
    Self: Sized + IoLength,
{
    fn io_read(buf: &mut IoReadable, length: usize) -> Result<Self, IoReadError>;
}

pub trait IoReadWithKeyOptional
where
    Self: Sized + IoLength,
{
    type Key: From<u32> + Into<u32>;

    fn io_read(
        key: Self::Key,
        buf: &mut IoReadable,
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
    #[error("Unsupported key: {key}")]
    UnsupportedKey { key: u32 },
}

/// Error when reading a COctetString
#[derive(thiserror::Error, Debug)]
pub enum COctetStringIoReadError {
    #[error("Too few bytes. actual: {actual}, min: {min}")]
    TooFewBytes { actual: usize, min: usize },
    #[error("Not ASCII")]
    NotAscii,
    #[error("Not null terminated")]
    NotNullTerminated,
}

/// Error when reading an OctetString
#[derive(thiserror::Error, Debug)]
pub enum OctetStringIoReadError {
    #[error("Too many bytes. actual: {actual}, max: {max}")]
    TooManyBytes { actual: usize, max: usize },
    #[error("Too few bytes. actual: {actual}, min: {min}")]
    TooFewBytes { actual: usize, min: usize },
}
