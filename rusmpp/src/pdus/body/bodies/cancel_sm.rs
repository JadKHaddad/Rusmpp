use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::{npi::Npi, service_type::ServiceType, ton::Ton},
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CancelSm {
    serivce_type: ServiceType,
    message_id: COctetString<1, 65>,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    dest_addr_ton: Ton,
    dest_addr_npi: Npi,
    destination_addr: COctetString<1, 21>,
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

    pub fn service_type(&self) -> &ServiceType {
        &self.serivce_type
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn source_addr_ton(&self) -> &Ton {
        &self.source_addr_ton
    }

    pub fn source_addr_npi(&self) -> &Npi {
        &self.source_addr_npi
    }

    pub fn source_addr(&self) -> &COctetString<1, 21> {
        &self.source_addr
    }

    pub fn dest_addr_ton(&self) -> &Ton {
        &self.dest_addr_ton
    }

    pub fn dest_addr_npi(&self) -> &Npi {
        &self.dest_addr_npi
    }

    pub fn destination_addr(&self) -> &COctetString<1, 21> {
        &self.destination_addr
    }

    pub fn into_parts(
        self,
    ) -> (
        ServiceType,
        COctetString<1, 65>,
        Ton,
        Npi,
        COctetString<1, 21>,
        Ton,
        Npi,
        COctetString<1, 21>,
    ) {
        (
            self.serivce_type,
            self.message_id,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.dest_addr_ton,
            self.dest_addr_npi,
            self.destination_addr,
        )
    }
}

impl IoLength for CancelSm {
    fn length(&self) -> usize {
        self.serivce_type.length()
            + self.message_id.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.dest_addr_ton.length()
            + self.dest_addr_npi.length()
            + self.destination_addr.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for CancelSm {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.serivce_type.async_io_write(buf).await?;
        self.message_id.async_io_write(buf).await?;
        self.source_addr_ton.async_io_write(buf).await?;
        self.source_addr_npi.async_io_write(buf).await?;
        self.source_addr.async_io_write(buf).await?;
        self.dest_addr_ton.async_io_write(buf).await?;
        self.dest_addr_npi.async_io_write(buf).await?;
        self.destination_addr.async_io_write(buf).await?;

        Ok(())
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
