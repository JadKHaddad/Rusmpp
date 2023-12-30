use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const SIZE: usize = 1;

impl IoLength for u8 {
    fn length(&self) -> usize {
        SIZE
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for u8 {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        buf.write_u8(*self).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for u8 {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(buf.read_u8().await?)
    }
}
