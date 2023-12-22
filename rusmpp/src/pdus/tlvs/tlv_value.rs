use crate::{
    io::{
        length::IoLength,
        read::{
            AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadWithLength, AsyncIoReadable,
            IoReadError,
        },
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::interface_version::InterfaceVersion,
    types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

use super::tlv_tag::TLVTag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TLVValue {
    ScInterfaceVersion(InterfaceVersion),
    Other {
        tag: TLVTag,
        value: NoFixedSizeOctetString,
    },
}

impl TLVValue {
    pub fn tlv_tag(&self) -> TLVTag {
        match self {
            TLVValue::ScInterfaceVersion(_) => TLVTag::ScInterfaceVersion,
            TLVValue::Other { tag, .. } => *tag,
        }
    }
}

impl IoLength for TLVValue {
    fn length(&self) -> usize {
        match self {
            TLVValue::ScInterfaceVersion(v) => v.length(),
            TLVValue::Other { value, .. } => value.length(),
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for TLVValue {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        match self {
            TLVValue::ScInterfaceVersion(v) => v.async_io_write(buf).await,
            TLVValue::Other { value, .. } => value.async_io_write(buf).await,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithKeyOptional for TLVValue {
    type Key = TLVTag;

    async fn async_io_read(
        key: Self::Key,
        buf: &mut AsyncIoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError> {
        let read = match key {
            TLVTag::ScInterfaceVersion => {
                TLVValue::ScInterfaceVersion(InterfaceVersion::async_io_read(buf).await?)
            }
            TLVTag::Other(_) => TLVValue::Other {
                tag: key,
                value: NoFixedSizeOctetString::async_io_read(buf, length).await?,
            },
            _ => return Err(IoReadError::UnsupportedKey { key: key.into() }),
        };

        Ok(Some(read))
    }
}
