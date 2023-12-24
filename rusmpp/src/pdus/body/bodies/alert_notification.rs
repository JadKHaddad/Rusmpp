use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::{
        tlvs::tlv::TLV,
        types::{npi::Npi, ton::Ton},
    },
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AlertNotification {
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 65>,
    pub esme_addr_ton: Ton,
    pub esme_addr_npi: Npi,
    pub esme_addr: COctetString<1, 65>,
    pub ms_availability_status: Option<TLV>,
}

impl AlertNotification {
    pub fn new(
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 65>,
        esme_addr_ton: Ton,
        esme_addr_npi: Npi,
        esme_addr: COctetString<1, 65>,
        ms_availability_status: Option<TLV>,
    ) -> Self {
        Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status,
        }
    }
}

impl IoLength for AlertNotification {
    fn length(&self) -> usize {
        self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.esme_addr_ton.length()
            + self.esme_addr_npi.length()
            + self.esme_addr.length()
            + self.ms_availability_status.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for AlertNotification {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.source_addr_ton.async_io_write(buf).await?;
        self.source_addr_npi.async_io_write(buf).await?;
        self.source_addr.async_io_write(buf).await?;
        self.esme_addr_ton.async_io_write(buf).await?;
        self.esme_addr_npi.async_io_write(buf).await?;
        self.esme_addr.async_io_write(buf).await?;
        self.ms_availability_status.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for AlertNotification {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let source_addr_ton = Ton::async_io_read(buf).await?;
        let source_addr_npi = Npi::async_io_read(buf).await?;
        let source_addr = COctetString::async_io_read(buf).await?;
        let esme_addr_ton = Ton::async_io_read(buf).await?;
        let esme_addr_npi = Npi::async_io_read(buf).await?;
        let esme_addr = COctetString::async_io_read(buf).await?;

        let ms_availability_status_expected_len = length.saturating_sub(
            source_addr_ton.length()
                + source_addr_npi.length()
                + source_addr.length()
                + esme_addr_ton.length()
                + esme_addr_npi.length()
                + esme_addr.length(),
        );
        let ms_availability_status = if ms_availability_status_expected_len > 0 {
            Some(TLV::async_io_read(buf).await?)
        } else {
            None
        };

        Ok(Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status,
        })
    }
}