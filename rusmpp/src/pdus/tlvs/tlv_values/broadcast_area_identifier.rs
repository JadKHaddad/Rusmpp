use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    types::octet_string::OctetString,
};

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum BroadcastAreaFormat {
    AliasName = 0x00,
    EllipsoidArc = 0x01,
    Polygon = 0x02,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for BroadcastAreaFormat {
    fn default() -> Self {
        BroadcastAreaFormat::AliasName
    }
}

impl IoLength for BroadcastAreaFormat {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for BroadcastAreaFormat {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for BroadcastAreaFormat {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastAreaIdentifier {
    pub format: BroadcastAreaFormat,
    pub area: OctetString<100>,
}

impl BroadcastAreaIdentifier {
    pub fn new(format: BroadcastAreaFormat, area: OctetString<100>) -> Self {
        Self { format, area }
    }
}

impl IoLength for BroadcastAreaIdentifier {
    fn length(&self) -> usize {
        self.format.length() + self.area.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for BroadcastAreaIdentifier {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.format.async_io_write(buf).await?;
        self.area.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for BroadcastAreaIdentifier {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let format = BroadcastAreaFormat::async_io_read(buf).await?;
        let area = OctetString::async_io_read(buf, length - format.length() as usize).await?;

        Ok(Self { format, area })
    }
}
