use crate::encode::Length;

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
