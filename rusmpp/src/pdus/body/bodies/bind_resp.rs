use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::tlvs::tlv::TLV,
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BindResp {
    pub system_id: COctetString<16>,
    pub sc_interface_version: Option<TLV>,
}

impl BindResp {
    pub fn new(system_id: COctetString<16>, sc_interface_version: Option<TLV>) -> Self {
        Self {
            system_id,
            sc_interface_version,
        }
    }
}

impl IoLength for BindResp {
    fn length(&self) -> usize {
        self.system_id.length() + self.sc_interface_version.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for BindResp {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.system_id.async_io_write(buf).await?;
        self.sc_interface_version.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for BindResp {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let system_id = COctetString::async_io_read(buf).await?;

        let sc_interface_version_expected_len = length.saturating_sub(system_id.length());
        let sc_interface_version = if sc_interface_version_expected_len > 0 {
            Some(TLV::async_io_read(buf).await?)
        } else {
            None
        };

        Ok(Self {
            system_id,
            sc_interface_version,
        })
    }
}
