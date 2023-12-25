use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::{npi::Npi, ton::Ton},
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl IoLength for QuerySm {
    fn length(&self) -> usize {
        self.message_id.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for QuerySm {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.message_id.async_io_write(buf).await?;
        self.source_addr_ton.async_io_write(buf).await?;
        self.source_addr_npi.async_io_write(buf).await?;
        self.source_addr.async_io_write(buf).await?;

        Ok(())
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
