use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

impl<T> IoLength for Vec<T>
where
    T: IoLength,
{
    fn length(&self) -> usize {
        self.iter().map(|v| v.length()).sum()
    }
}

#[async_trait::async_trait]
impl<T> AsyncIoWrite for Vec<T>
where
    T: AsyncIoWrite + Send + Sync,
{
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        for v in self {
            v.async_io_write(buf).await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl<T> AsyncIoReadWithLength for Vec<T>
where
    T: AsyncIoRead + Send + Sync,
{
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let mut vec = Vec::new();
        let mut remaining_length = length;

        while remaining_length > 0 {
            let v = T::async_io_read(buf).await?;
            remaining_length -= v.length();
            vec.push(v);
        }

        Ok(vec)
    }
}
