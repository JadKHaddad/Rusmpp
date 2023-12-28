use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIo, RusmppIoU8};

use rusmpp_io::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    },
    types::octet_string::OctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, RusmppIo)]
pub struct Subaddress {
    pub tag: SubaddressTag,
    pub addr: OctetString<1, 22>,
}

impl Subaddress {
    pub fn new(tag: SubaddressTag, addr: OctetString<1, 22>) -> Self {
        Self { tag, addr }
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
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU8,
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
