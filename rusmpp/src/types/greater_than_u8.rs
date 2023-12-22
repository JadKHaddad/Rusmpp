use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, GreaterThanValueIoReadError, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Too small value. actual: {actual}, min: {min}")]
    TooSmall { actual: usize, min: usize },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GreaterThanU8<const MIN: u8> {
    value: u8,
}

impl<const MIN: u8> GreaterThanU8<MIN> {
    pub fn new(value: u8) -> Result<Self, Error> {
        if value < MIN {
            return Err(Error::TooSmall {
                actual: value as usize,
                min: MIN as usize,
            });
        }

        Ok(GreaterThanU8 { value })
    }
}

impl<const MIN: u8> Default for GreaterThanU8<MIN> {
    fn default() -> Self {
        GreaterThanU8 { value: MIN }
    }
}

impl<const MIN: u8> IoLength for GreaterThanU8<MIN> {
    fn length(&self) -> usize {
        self.value.length()
    }
}

#[async_trait::async_trait]
impl<const MIN: u8> AsyncIoWrite for GreaterThanU8<MIN> {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.value.async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl<const MIN: u8> AsyncIoRead for GreaterThanU8<MIN> {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        let value = u8::async_io_read(buf).await?;

        if value < MIN {
            return Err(IoReadError::GreaterThanValue(
                GreaterThanValueIoReadError::TooSmall {
                    actual: value as usize,
                    min: MIN as usize,
                },
            ));
        }

        Ok(GreaterThanU8 { value })
    }
}
