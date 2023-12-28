use rusmpp_macros::RusmppIo;

use crate::{
    io::read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    pdus::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct Bind {
    pub system_id: COctetString<1, 16>,
    pub password: COctetString<1, 9>,
    pub system_type: COctetString<1, 13>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: COctetString<1, 41>,
}

impl Bind {
    pub fn new(
        system_id: COctetString<1, 16>,
        password: COctetString<1, 9>,
        system_type: COctetString<1, 13>,
        interface_version: InterfaceVersion,
        addr_ton: Ton,
        addr_npi: Npi,
        address_range: COctetString<1, 41>,
    ) -> Self {
        Self {
            system_id,
            password,
            system_type,
            interface_version,
            addr_ton,
            addr_npi,
            address_range,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for Bind {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            system_id: COctetString::async_io_read(buf).await?,
            password: COctetString::async_io_read(buf).await?,
            system_type: COctetString::async_io_read(buf).await?,
            interface_version: InterfaceVersion::async_io_read(buf).await?,
            addr_ton: Ton::async_io_read(buf).await?,
            addr_npi: Npi::async_io_read(buf).await?,
            address_range: COctetString::async_io_read(buf).await?,
        })
    }
}
