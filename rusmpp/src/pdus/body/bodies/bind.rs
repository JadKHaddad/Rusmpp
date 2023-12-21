use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bind {
    pub system_id: COctetString<16>,
    pub password: COctetString<9>,
    pub system_type: COctetString<13>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: COctetString<41>,
}

impl IoLength for Bind {
    fn length(&self) -> usize {
        self.system_id.length()
            + self.password.length()
            + self.system_type.length()
            + self.interface_version.length()
            + self.addr_ton.length()
            + self.addr_npi.length()
            + self.address_range.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for Bind {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.system_id.async_io_write(buf).await?;
        self.password.async_io_write(buf).await?;
        self.system_type.async_io_write(buf).await?;
        self.interface_version.async_io_write(buf).await?;
        self.addr_ton.async_io_write(buf).await?;
        self.addr_npi.async_io_write(buf).await?;
        self.address_range.async_io_write(buf).await?;

        Ok(())
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
