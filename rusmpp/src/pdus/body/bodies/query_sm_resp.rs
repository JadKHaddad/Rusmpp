use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::tlvs::tlv_values::message_state::MessageState,
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QuerySmResp {
    pub message_id: COctetString<1, 65>,
    pub final_date: EmptyOrFullCOctetString<17>,
    pub message_state: MessageState,
    pub error_code: u8,
}

impl QuerySmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        final_date: EmptyOrFullCOctetString<17>,
        message_state: MessageState,
        error_code: u8,
    ) -> Self {
        Self {
            message_id,
            final_date,
            message_state,
            error_code,
        }
    }
}

impl IoLength for QuerySmResp {
    fn length(&self) -> usize {
        self.message_id.length()
            + self.final_date.length()
            + self.message_state.length()
            + self.error_code.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for QuerySmResp {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.message_id.async_io_write(buf).await?;
        self.final_date.async_io_write(buf).await?;
        self.message_state.async_io_write(buf).await?;
        self.error_code.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for QuerySmResp {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            message_id: COctetString::async_io_read(buf).await?,
            final_date: EmptyOrFullCOctetString::async_io_read(buf).await?,
            message_state: MessageState::async_io_read(buf).await?,
            error_code: u8::async_io_read(buf).await?,
        })
    }
}
