use super::length::IoLength;

pub type AsyncIoReadable = dyn tokio::io::AsyncBufRead + Send + Unpin;

#[async_trait::async_trait]
pub trait AsyncIoRead
where
    Self: Sized + IoLength,
{
    type Error;

    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, Self::Error>;
}
