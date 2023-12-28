use rusmpp_macros::RusmppIo;

use rusmpp_io::{
    io::read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    types::c_octet_string::COctetString,
};

use crate::pdus::types::{npi::Npi, ton::Ton};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct QuerySm {
    pub message_id: COctetString<1, 65>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 21>,
}

impl QuerySm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for QuerySm {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            message_id: COctetString::async_io_read(buf).await?,
            source_addr_ton: Ton::async_io_read(buf).await?,
            source_addr_npi: Npi::async_io_read(buf).await?,
            source_addr: COctetString::async_io_read(buf).await?,
        })
    }
}
