use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithKey, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::interface_version::InterfaceVersion,
};

use super::tlv_tag::TLVTag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TLVValue {
    ScInterfaceVersion(InterfaceVersion),
}

impl TLVValue {
    pub fn tlv_tag(&self) -> TLVTag {
        match self {
            TLVValue::ScInterfaceVersion(_) => TLVTag::ScInterfaceVersion,
        }
    }
}

impl IoLength for TLVValue {
    fn length(&self) -> usize {
        match self {
            TLVValue::ScInterfaceVersion(v) => v.length(),
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for TLVValue {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        match self {
            TLVValue::ScInterfaceVersion(v) => v.async_io_write(buf).await,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithKey for TLVValue {
    type Key = TLVTag;

    async fn async_io_read(
        key: Self::Key,
        buf: &mut AsyncIoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError> {
        if !key.has_value() {
            return Ok(None);
        }

        let read = match key {
            TLVTag::ScInterfaceVersion => {
                TLVValue::ScInterfaceVersion(InterfaceVersion::async_io_read(buf).await?)
            }
            _ => {
                return Err(IoReadError::UnknownKey {
                    key: u32::from(u16::from(key)),
                })
            }
        };

        Ok(Some(read))
    }
}
