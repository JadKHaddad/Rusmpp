use crate::{
    io::{
        decode::{Decode, DecodeError, OptionalDecodeWithKey},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

use super::{tlv_tag::TLVTag, tlv_value::TLVValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TLV {
    tag: TLVTag,
    value_length: u16,
    value: Option<TLVValue>,
}

impl TLV {
    /// Create a new TLV with the given value
    pub fn new(value: TLVValue) -> Self {
        let tag = value.tlv_tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }

    /// Create a new TLV without a value
    pub fn new_without_value(tag: TLVTag) -> Self {
        Self {
            tag,
            value_length: 0,
            value: None,
        }
    }

    pub fn tag(&self) -> &TLVTag {
        &self.tag
    }

    pub fn value_length(&self) -> u16 {
        self.value_length
    }

    pub fn value(&self) -> Option<&TLVValue> {
        self.value.as_ref()
    }

    pub fn into_value(self) -> Option<TLVValue> {
        self.value
    }
}

impl Length for TLV {
    fn length(&self) -> usize {
        self.tag.length() + self.value_length.length() + self.value.length()
    }
}

impl Encode for TLV {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.tag.encode_to(writer));
        tri!(self.value_length.encode_to(writer));
        tri!(self.value.encode_to(writer));

        Ok(())
    }
}

impl Decode for TLV {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let tag = tri!(TLVTag::decode_from(reader));
        let value_length = tri!(u16::decode_from(reader));

        let value = tri!(TLVValue::length_checked_decode_from(
            tag,
            reader,
            value_length as usize
        ));

        Ok(Self {
            tag,
            value_length,
            value,
        })
    }
}
