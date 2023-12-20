use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::io::{
    length::IoLength,
    read::{result::IoRead, AsyncIoRead, AsyncIoReadable, IoReadResult},
    write::{AsyncIoWritable, AsyncIoWrite},
};

const SIZE: usize = 2;

impl IoLength for u16 {
    fn length(&self) -> usize {
        SIZE
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for u16 {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        buf.write_u16(*self).await?;

        Ok(SIZE)
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for u16 {
    type Error = std::io::Error;

    async fn async_io_read(buf: &mut AsyncIoReadable) -> IoReadResult<Self, Self::Error> {
        Ok(IoRead {
            value: buf.read_u16().await?,
            read: SIZE,
        })
    }
}
