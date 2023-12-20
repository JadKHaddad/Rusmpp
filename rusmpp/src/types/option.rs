use crate::io::{
    length::IoLength,
    write::{AsyncIoWritable, AsyncIoWrite},
};

impl<T> IoLength for Option<T>
where
    T: IoLength,
{
    fn length(&self) -> usize {
        match self {
            Some(v) => v.length(),
            None => 0,
        }
    }
}

#[async_trait::async_trait]
impl<T> AsyncIoWrite for Option<T>
where
    T: AsyncIoWrite + Send + Sync,
{
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        match self {
            Some(v) => v.async_io_write(buf).await,
            None => Ok(0),
        }
    }
}
