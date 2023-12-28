use rusmpp_macros::RusmppIo;

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    },
    pdus::tlvs::tlv::{MessageSubmissionResponseTLV, TLV},
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct SubmitOrDataSmResp {
    message_id: COctetString<1, 65>,
    tlvs: Vec<TLV>,
}

impl SubmitOrDataSmResp {
    pub fn new(message_id: COctetString<1, 65>, tlvs: Vec<MessageSubmissionResponseTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect();

        Self { message_id, tlvs }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (COctetString<1, 65>, Vec<TLV>) {
        (self.message_id, self.tlvs)
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for SubmitOrDataSmResp {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let message_id = COctetString::async_io_read(buf).await?;
        let tlvs =
            Vec::<TLV>::async_io_read(buf, length.saturating_sub(message_id.length())).await?;

        Ok(Self { message_id, tlvs })
    }
}
