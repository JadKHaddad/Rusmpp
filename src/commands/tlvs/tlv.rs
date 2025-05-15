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

/// Trait for types that have a TLV tag.
///
/// A Type must implement this trait to be used as a [`SingleTlv<V>`].
pub(crate) trait HasTlvTag {
    const TAG: TlvTag;
}

/// Since `TLV`s can be used in any order, we store them in a `Vec`.
/// This is a single concrete `TLV` used in `PDU`s that define a single concrete `TLV` at the end,
/// like [`BindTransmitterResp`], [`BindReceiverResp`], [`BindTransceiverResp`] and [`AlertNotification`].
///
/// The decoding of this `TLV` might fail if the given tag does not match the expected tag.
///
/// The value of this `TLV` is stored in an `Option`, since it may not be required.
/// Creating an instance of this `TLV` using a factory function will always set the value to `Some`.
/// But decoded instances may have a `None` value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct SingleTlv<V> {
    tag: TlvTag,
    value_length: u16,
    value: Option<V>,
}

impl<V: Length + HasTlvTag> SingleTlv<V> {
    /// Create a new TLV with the given value
    pub fn new(value: V) -> Self {
        let tag = V::TAG;
        let value_length = value.length() as u16;
        let value = Some(value);

        Self {
            tag,
            value_length,
            value,
        }
    }

    pub const fn tag(&self) -> TlvTag {
        self.tag
    }

    pub const fn value_length(&self) -> u16 {
        self.value_length
    }

    pub const fn value(&self) -> Option<&V> {
        self.value.as_ref()
    }
}

impl<V: HasTlvTag + Length + Default> Default for SingleTlv<V> {
    fn default() -> Self {
        Self::new(V::default())
    }
}

impl<V: Length + HasTlvTag> From<V> for SingleTlv<V> {
    fn from(value: V) -> Self {
        Self::new(value)
    }
}

impl<V: Length> Length for SingleTlv<V> {
    fn length(&self) -> usize {
        self.tag.length() + self.value_length.length() + self.value.length()
    }
}

impl<V: Encode> Encode for SingleTlv<V> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = self.tag.encode_move(dst, size);
        let size = self.value_length.encode_move(dst, size);
        self.value.encode_move(dst, size)
    }
}

impl<V: DecodeWithLength + HasTlvTag> Decode for SingleTlv<V> {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        let size = 0;

        let (tag, size) = DecodeExt::decode_move(src, size)?;

        if tag != V::TAG {
            return Err(DecodeError::UnsupportedKey {
                key: u32::from(u16::from(tag)),
            });
        };

        let (value_length, size) = DecodeExt::decode_move(src, size)?;
        let (value, size) =
            DecodeWithLengthExt::length_checked_decode_move(src, value_length as usize, size)?
                .map(|(value, size)| (Some(value), size))
                .unwrap_or((None, size));

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
