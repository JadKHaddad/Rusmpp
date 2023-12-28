pub type AsyncIoWritable = dyn tokio::io::AsyncWrite + Send + Unpin;

#[async_trait::async_trait]
pub trait AsyncIoWrite {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()>;
}
