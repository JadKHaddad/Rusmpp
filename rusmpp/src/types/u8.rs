use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::io::{
    length::IoLength,
    read::{
        result::{IoRead, IoReadResult},
        AsyncIoRead, AsyncIoReadable,
    },
    write::{AsyncIoWritable, AsyncIoWrite},
};

const SIZE: usize = 1;

impl IoLength for u8 {
    fn length(&self) -> usize {
        SIZE
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for u8 {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        buf.write_u8(*self).await?;

        Ok(SIZE)
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for u8 {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> IoReadResult<Self> {
        Ok(IoRead {
            value: buf.read_u8().await?,
            read: SIZE,
        })
    }
}
