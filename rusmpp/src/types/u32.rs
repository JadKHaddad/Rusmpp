use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::io::{
    length::IoLength,
    read::{result::IoRead, AsyncIoRead, AsyncIoReadable, IoReadResult},
    write::{AsyncIoWritable, AsyncIoWrite},
};

const SIZE: usize = 4;

impl IoLength for u32 {
    fn length(&self) -> usize {
        SIZE
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for u32 {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        buf.write_u32(*self).await?;

        Ok(SIZE)
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for u32 {
    type Error = std::io::Error;

    async fn async_io_read(buf: &mut AsyncIoReadable) -> IoReadResult<Self, Self::Error> {
        Ok(IoRead {
            value: buf.read_u32().await?,
            read: SIZE,
        })
    }
}
