use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadWithKey, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

use super::{tlv_tag::TLVTag, tlv_value::TLVValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TLV {
    tag: TLVTag,
    value_length: u16,
    value: Option<TLVValue>,
}

impl TLV {
    pub fn new(value: TLVValue) -> Self {
        let tag = value.tlv_tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }
}

impl IoLength for TLV {
    fn length(&self) -> usize {
        self.tag.length() + self.value_length.length() + self.value.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for TLV {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        let mut written = 0;

        written += self.tag.async_io_write(buf).await?;
        written += self.value_length.async_io_write(buf).await?;
        written += self.value.async_io_write(buf).await?;

        Ok(written)
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for TLV {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        let tag = TLVTag::async_io_read(buf).await?;
        let value_length = u16::async_io_read(buf).await?;
        let value = TLVValue::async_io_read(tag, buf, value_length as usize).await?;

        Ok(Self {
            tag,
            value_length,
            value,
        })
    }
}
