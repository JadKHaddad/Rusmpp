use rusmpp_macros::RusmppIo;

use crate::{
    io::read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    types::c_octet_string::COctetString,
};

use super::{command_status::CommandStatus, npi::Npi, ton::Ton};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct UnsuccessSme {
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: COctetString<1, 21>,
    pub error_status_code: CommandStatus,
}

impl UnsuccessSme {
    pub fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
        error_status_code: CommandStatus,
    ) -> Self {
        Self {
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            error_status_code,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for UnsuccessSme {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            dest_addr_ton: Ton::async_io_read(buf).await?,
            dest_addr_npi: Npi::async_io_read(buf).await?,
            destination_addr: COctetString::async_io_read(buf).await?,
            error_status_code: CommandStatus::async_io_read(buf).await?,
        })
    }
}
