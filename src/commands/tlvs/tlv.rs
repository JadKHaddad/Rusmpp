use crate::encode::Length;

use super::{tlv_tag::TLVTag, tlv_value::TLVValue};

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
    pub struct TLV {
        tag: TLVTag,
        value_length: u16,
        @[key = tag, length = value_length]
        value: Option<TLVValue>,
    }
}

impl TLV {
    /// Create a new TLV with the given value
    pub fn new(value: TLVValue) -> Self {
        Self::from(value)
    }

    /// Create a new TLV without a value
    pub fn without_value(tag: TLVTag) -> Self {
        Self::from(tag)
    }

    pub fn tag(&self) -> TLVTag {
        self.tag
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

impl From<TLVValue> for TLV {
    fn from(value: TLVValue) -> Self {
        let tag = value.tlv_tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }
}

impl From<TLVTag> for TLV {
    fn from(tag: TLVTag) -> Self {
        Self {
            tag,
            value_length: 0,
            value: None,
        }
    }
}
