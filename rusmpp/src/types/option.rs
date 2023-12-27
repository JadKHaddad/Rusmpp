use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadable, IoReadError},
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
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        match self {
            Some(v) => v.async_io_write(buf).await,
            None => Ok(()),
        }
    }
}

pub async fn async_io_read_with_key_optional<T, K>(
    key: K,
    buf: &mut AsyncIoReadable,
    length: usize,
) -> Result<Option<T>, IoReadError>
where
    T: AsyncIoReadWithKeyOptional<Key = K> + Send + Sync,
{
    if length == 0 {
        return Ok(None);
    }

    T::async_io_read(key, buf, length).await
}

pub async fn async_io_read<T>(
    buf: &mut AsyncIoReadable,
    length: usize,
) -> Result<Option<T>, IoReadError>
where
    T: AsyncIoRead + Send + Sync,
{
    if length == 0 {
        return Ok(None);
    }

    Ok(Some(T::async_io_read(buf).await?))
}
