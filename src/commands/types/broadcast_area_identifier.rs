use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::octet_string::OctetString,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum BroadcastAreaFormat {
    #[default]
    AliasName = 0x00,
    EllipsoidArc = 0x01,
    Polygon = 0x02,
    Other(u8),
}

impl From<u8> for BroadcastAreaFormat {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BroadcastAreaFormat::AliasName,
            0x01 => BroadcastAreaFormat::EllipsoidArc,
            0x02 => BroadcastAreaFormat::Polygon,
            value => BroadcastAreaFormat::Other(value),
        }
    }
}

impl From<BroadcastAreaFormat> for u8 {
    fn from(value: BroadcastAreaFormat) -> Self {
        match value {
            BroadcastAreaFormat::AliasName => 0x00,
            BroadcastAreaFormat::EllipsoidArc => 0x01,
            BroadcastAreaFormat::Polygon => 0x02,
            BroadcastAreaFormat::Other(value) => value,
        }
    }
}

impl Length for BroadcastAreaFormat {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for BroadcastAreaFormat {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for BroadcastAreaFormat {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastAreaIdentifier {
    pub format: BroadcastAreaFormat,
    pub area: OctetString<0, 100>,
}

impl Length for BroadcastAreaIdentifier {
    fn length(&self) -> usize {
        self.format.length() + self.area.length()
    }
}

impl Encode for BroadcastAreaIdentifier {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.format.encode_to(writer));
        tri!(self.area.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for BroadcastAreaIdentifier {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let format = tri!(BroadcastAreaFormat::decode_from(reader));

        let area_length = length.saturating_sub(format.length());

        let area = tri!(OctetString::decode_from(reader, area_length));

        Ok(Self { format, area })
    }
}
