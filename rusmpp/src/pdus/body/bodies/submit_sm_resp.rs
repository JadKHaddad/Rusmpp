use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubmitSmResp {
    pub message_id: COctetString<65>,
    // TODO: message_submission_response_tlvs: Vec<MessageSubmittionTLV>,
}

impl IoLength for SubmitSmResp {
    fn length(&self) -> usize {
        self.message_id.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for SubmitSmResp {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.message_id.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for SubmitSmResp {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let message_id = COctetString::async_io_read(buf).await?;

        Ok(Self { message_id })
    }
}
