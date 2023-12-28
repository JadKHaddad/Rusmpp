use rusmpp_macros::RusmppIo;

use rusmpp_io::{
    io::read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
};

use crate::pdus::tlvs::tlv_values::message_state::MessageState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
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
