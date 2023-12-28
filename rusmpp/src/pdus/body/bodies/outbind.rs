use rusmpp_macros::RusmppIo;

use crate::{
    io::read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
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

#[async_trait::async_trait]
impl AsyncIoRead for Outbind {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            system_id: COctetString::async_io_read(buf).await?,
            password: COctetString::async_io_read(buf).await?,
        })
    }
}
