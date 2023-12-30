use crate::io::{
    length::IoLength,
    read::{
        AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoRead, IoReadError, IoReadWithLength,
        IoReadable,
    },
    write::{AsyncIoWritable, AsyncIoWrite, IoWritable, IoWrite},
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

impl<T> IoWrite for Vec<T>
where
    T: IoWrite + Send + Sync,
{
    fn io_write(&self, buf: &mut IoWritable) -> std::io::Result<()> {
        for v in self {
            v.io_write(buf)?;
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
            remaining_length = remaining_length.saturating_sub(v.length());
            vec.push(v);
        }

        Ok(vec)
    }
}

impl<T> IoReadWithLength for Vec<T>
where
    T: IoRead + Send + Sync,
{
    fn io_read(buf: &mut IoReadable, length: usize) -> Result<Self, IoReadError> {
        let mut vec = Vec::new();
        let mut remaining_length = length;

        while remaining_length > 0 {
            let v = T::io_read(buf)?;
            remaining_length = remaining_length.saturating_sub(v.length());
            vec.push(v);
        }

        Ok(vec)
    }
}

pub async fn async_read_counted<T>(
    buf: &mut AsyncIoReadable,
    count: usize,
) -> Result<Vec<T>, IoReadError>
where
    T: AsyncIoRead + Send + Sync,
{
    let mut vec = Vec::new();

    for _ in 0..count {
        let v = T::async_io_read(buf).await?;
        vec.push(v);
    }

    Ok(vec)
}

pub fn read_counted<T>(buf: &mut IoReadable, count: usize) -> Result<Vec<T>, IoReadError>
where
    T: IoRead + Send + Sync,
{
    let mut vec = Vec::new();

    for _ in 0..count {
        let v = T::io_read(buf)?;
        vec.push(v);
    }

    Ok(vec)
}
