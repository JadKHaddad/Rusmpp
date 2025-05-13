use crate::{
    decode::{Decode, DecodeError, DecodeExt, DecodeWithLength, DecodeWithLengthExt},
    encode::{Encode, EncodeExt, Length},
};

use super::{tlv_tag::TlvTag, tlv_value::TlvValue};

pub mod broadcast_request;
pub mod broadcast_response;
pub mod cancel_broadcast;
pub mod message_delivery_request;
pub mod message_delivery_response;
pub mod message_submission_request;
pub mod message_submission_response;
pub mod query_broadcast_response;

crate::create! {
    @[skip_test]
    /// See module level documentation
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Tlv {
        tag: TlvTag,
        value_length: u16,
        @[key = tag, length = value_length]
        value: Option<TlvValue>,
    }
}

impl Tlv {
    /// Create a new TLV with the given value
    pub fn new(value: TlvValue) -> Self {
        Self::from(value)
    }

    /// Create a new TLV without a value
    pub fn without_value(tag: TlvTag) -> Self {
        Self::from(tag)
    }

    pub fn tag(&self) -> TlvTag {
        self.tag
    }

    pub fn value_length(&self) -> u16 {
        self.value_length
    }

    pub fn value(&self) -> Option<&TlvValue> {
        self.value.as_ref()
    }

    pub fn into_value(self) -> Option<TlvValue> {
        self.value
    }
}

impl From<TlvValue> for Tlv {
    fn from(value: TlvValue) -> Self {
        let tag = value.tlv_tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }
}

impl From<TlvTag> for Tlv {
    fn from(tag: TlvTag) -> Self {
        Self {
            tag,
            value_length: 0,
            value: None,
        }
    }
}

pub(crate) trait HasTlvTag {
    fn tlv_tag() -> TlvTag;
}

/// See module level documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct KnownTlv<V> {
    tag: TlvTag,
    value_length: u16,
    value: V,
}

impl<V: Length + HasTlvTag> KnownTlv<V> {
    /// Create a new TLV with the given value
    pub fn new(value: V) -> Self {
        let tag = V::tlv_tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value,
        }
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<V: HasTlvTag + Length + Default> Default for KnownTlv<V> {
    fn default() -> Self {
        Self::new(V::default())
    }
}

impl<V: Length + HasTlvTag> From<V> for KnownTlv<V> {
    fn from(value: V) -> Self {
        Self::new(value)
    }
}

impl<V: Length> Length for KnownTlv<V> {
    fn length(&self) -> usize {
        self.tag.length() + self.value_length.length() + self.value.length()
    }
}

impl<V: Encode> Encode for KnownTlv<V> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = self.tag.encode_move(dst, size);
        let size = self.value_length.encode_move(dst, size);
        self.value.encode_move(dst, size)
    }
}

impl<V: DecodeWithLength + HasTlvTag> Decode for KnownTlv<V> {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        let size = 0;

        let (tag, size) = DecodeExt::decode_move(src, size)?;

        if tag != V::tlv_tag() {
            return Err(DecodeError::UnsupportedKey {
                key: u32::from(u16::from(tag)),
            });
        };

        let (value_length, size) = DecodeExt::decode_move(src, size)?;
        let (value, size) = DecodeWithLengthExt::decode_move(src, value_length as usize, size)?;

        Ok((
            Self {
                tag,
                value_length,
                value,
            },
            size,
        ))
    }
}

// TODO: remove the downcast stuff and use this instead. like bind_resp.
// Since DecodeWithLength now implemented for every decode. we can use DecodeWithLengthExt for all variants, while match decoding in tlv and pdu.
