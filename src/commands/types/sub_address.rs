// TODO manual implementation

use crate::{
    ende::decode::{DecodeError, DecodeWithLength},
    impl_length_encode, tri,
    types::{octet_string::OctetString, u8::EndeU8},
};

impl_length_encode! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub struct Subaddress {
        pub tag: SubaddressTag,
        pub addr: OctetString<1, 22>,
    }
}

impl Subaddress {
    pub fn new(tag: SubaddressTag, addr: OctetString<1, 22>) -> Self {
        Self { tag, addr }
    }
}

impl DecodeWithLength for Subaddress {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let tag = SubaddressTag::decode_from(reader)?;

        let addr_length = length.saturating_sub(tag.length());

        let addr = tri!(OctetString::decode_from(reader, addr_length));

        Ok(Self { tag, addr })
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum SubaddressTag {
    #[default]
    NsapEven = 0b10000000,
    NsapOdd = 0b10001000,
    UserSpecified = 0b10100000,
    Other(u8),
}

impl From<u8> for SubaddressTag {
    fn from(value: u8) -> Self {
        match value {
            0b10000000 => SubaddressTag::NsapEven,
            0b10001000 => SubaddressTag::NsapOdd,
            0b10100000 => SubaddressTag::UserSpecified,
            value => SubaddressTag::Other(value),
        }
    }
}

impl From<SubaddressTag> for u8 {
    fn from(value: SubaddressTag) -> Self {
        match value {
            SubaddressTag::NsapEven => 0b10000000,
            SubaddressTag::NsapOdd => 0b10001000,
            SubaddressTag::UserSpecified => 0b10100000,
            SubaddressTag::Other(value) => value,
        }
    }
}

impl EndeU8 for SubaddressTag {}
