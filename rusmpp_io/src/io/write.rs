pub type AsyncIoWritable = dyn tokio::io::AsyncWrite + Send + Unpin;
pub type IoWritable = dyn std::io::Write;

#[async_trait::async_trait]
pub trait AsyncIoWrite {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()>;
}

pub trait IoWrite {
    fn io_write(&self, buf: &mut IoWritable) -> std::io::Result<()>;
}
