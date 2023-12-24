use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Outbind {
    pub system_id: COctetString<1, 16>,
    pub password: COctetString<1, 9>,
}

impl Outbind {
    pub fn new(system_id: COctetString<1, 16>, password: COctetString<1, 9>) -> Self {
        Self {
            system_id,
            password,
        }
    }
}

impl IoLength for Outbind {
    fn length(&self) -> usize {
        self.system_id.length() + self.password.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for Outbind {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.system_id.async_io_write(buf).await?;
        self.password.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for Outbind {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            system_id: COctetString::async_io_read(buf).await?,
            password: COctetString::async_io_read(buf).await?,
        })
    }
}
