use rusmpp_macros::RusmppIo;

use rusmpp_io::{
    io::read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    types::c_octet_string::COctetString,
};

use crate::pdus::types::{npi::Npi, service_type::ServiceType, ton::Ton};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct CancelSm {
    pub serivce_type: ServiceType,
    pub message_id: COctetString<1, 65>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 21>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: COctetString<1, 21>,
}

impl CancelSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for CancelSm {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            serivce_type: ServiceType::async_io_read(buf).await?,
            message_id: COctetString::async_io_read(buf).await?,
            source_addr_ton: Ton::async_io_read(buf).await?,
            source_addr_npi: Npi::async_io_read(buf).await?,
            source_addr: COctetString::async_io_read(buf).await?,
            dest_addr_ton: Ton::async_io_read(buf).await?,
            dest_addr_npi: Npi::async_io_read(buf).await?,
            destination_addr: COctetString::async_io_read(buf).await?,
        })
    }
}
