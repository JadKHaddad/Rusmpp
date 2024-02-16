use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
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

impl Length for Subaddress {
    fn length(&self) -> usize {
        self.tag.length() + self.addr.length()
    }
}

impl Encode for Subaddress {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.tag.encode_to(writer));
        tri!(self.addr.encode_to(writer));

        Ok(())
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

impl Length for SubaddressTag {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for SubaddressTag {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for SubaddressTag {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}