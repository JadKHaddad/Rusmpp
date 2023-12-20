use crate::{
    io::{
        length::IoLength,
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
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        let mut written = 0;

        written += self.system_id.async_io_write(buf).await?;
        written += self.password.async_io_write(buf).await?;
        written += self.system_type.async_io_write(buf).await?;
        written += self.interface_version.async_io_write(buf).await?;
        written += self.addr_ton.async_io_write(buf).await?;
        written += self.addr_npi.async_io_write(buf).await?;
        written += self.address_range.async_io_write(buf).await?;

        Ok(written)
    }
}

// #[derive(thiserror::Error, Debug)]
// pub enum IoReadError {

// }
