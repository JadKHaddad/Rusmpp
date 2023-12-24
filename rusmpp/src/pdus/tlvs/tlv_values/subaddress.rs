use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    types::octet_string::OctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Subaddress {
    pub tag: SubaddressTag,
    pub addr: OctetString<1, 22>,
}

impl Subaddress {
    pub fn new(tag: SubaddressTag, addr: OctetString<1, 22>) -> Self {
        Self { tag, addr }
    }
}

impl IoLength for Subaddress {
    fn length(&self) -> usize {
        self.tag.length() + self.addr.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for Subaddress {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.tag.async_io_write(buf).await?;
        self.addr.async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for Subaddress {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let tag = SubaddressTag::async_io_read(buf).await?;
        let addr = OctetString::async_io_read(buf, length - tag.length() as usize).await?;

        Ok(Self { tag, addr })
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum SubaddressTag {
    NsapEven = 0b10000000,
    NsapOdd = 0b10001000,
    UserSpecified = 0b10100000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for SubaddressTag {
    fn default() -> Self {
        SubaddressTag::NsapEven
    }
}

impl IoLength for SubaddressTag {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for SubaddressTag {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for SubaddressTag {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}
