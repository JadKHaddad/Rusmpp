pub type AsyncIoReadable = dyn tokio::io::AsyncBufRead + Send + Unpin;

pub type IoReadResult<T, E> = Result<self::result::IoRead<T>, E>;

#[async_trait::async_trait]
pub trait AsyncIoRead
where
    Self: Sized,
{
    type Error;

    async fn async_io_read(buf: &mut AsyncIoReadable) -> IoReadResult<Self, Self::Error>;
}

pub mod result {

    #[derive(Debug, Clone)]
    pub struct IoRead<T> {
        pub value: T,
        pub read: usize,
    }

    impl<T> IoRead<T> {
        pub fn into_value(self) -> T {
            self.value
        }

        pub fn into_size(self) -> usize {
            self.read
        }
    }

    impl<T> From<(T, usize)> for IoRead<T> {
        fn from((value, read): (T, usize)) -> Self {
            Self { value, read }
        }
    }

    impl<T> From<IoRead<T>> for (T, usize) {
        fn from(io_read: IoRead<T>) -> Self {
            (io_read.value, io_read.read)
        }
    }
}
