use rusmpp_macros::RusmppIo;

use rusmpp_io::io::read::{AsyncIoRead, AsyncIoReadable, IoReadError};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct ItsSessionInfo {
    pub session_number: u8,
    pub sequence_number: u8,
}

impl ItsSessionInfo {
    pub fn new(session_number: u8, sequence_number: u8) -> Self {
        Self {
            session_number,
            sequence_number,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for ItsSessionInfo {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            session_number: u8::async_io_read(buf).await?,
            sequence_number: u8::async_io_read(buf).await?,
        })
    }
}
