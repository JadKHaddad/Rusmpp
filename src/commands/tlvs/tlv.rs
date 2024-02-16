use self::{
    message_delivery_request::{MessageDeliveryRequestTLVTag, MessageDeliveryRequestTLVValue},
    message_delivery_response::{MessageDeliveryResponseTLVTag, MessageDeliveryResponseTLVValue},
    message_submission_request::{
        MessageSubmissionRequestTLVTag, MessageSubmissionRequestTLVValue,
    },
    message_submission_response::{
        MessageSubmissionResponseTLVTag, MessageSubmissionResponseTLVValue,
    },
};
use super::{tlv_tag::TLVTag, tlv_value::TLVValue};
use crate::{
    ende::{
        decode::{Decode, DecodeError, OptionalDecodeWithKey},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

pub mod message_delivery_request;
pub mod message_delivery_response;
pub mod message_submission_request;
pub mod message_submission_response;

/// See module level documentation
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
    pub fn without_value(tag: TLVTag) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionRequestTLV {
    tlv: TLV,
}

impl MessageSubmissionRequestTLV {
    pub fn new(value: MessageSubmissionRequestTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn without_value(tag: MessageSubmissionRequestTLVTag) -> Self {
        let tlv = TLV::without_value(tag.into());

        Self { tlv }
    }
}

impl From<MessageSubmissionRequestTLV> for TLV {
    fn from(tlv: MessageSubmissionRequestTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionResponseTLV {
    tlv: TLV,
}

impl MessageSubmissionResponseTLV {
    pub fn new(value: MessageSubmissionResponseTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn without_value(tag: MessageSubmissionResponseTLVTag) -> Self {
        let tlv = TLV::without_value(tag.into());

        Self { tlv }
    }
}

impl From<MessageSubmissionResponseTLV> for TLV {
    fn from(tlv: MessageSubmissionResponseTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageDeliveryRequestTLV {
    tlv: TLV,
}

impl MessageDeliveryRequestTLV {
    pub fn new(value: MessageDeliveryRequestTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn without_value(tag: MessageDeliveryRequestTLVTag) -> Self {
        let tlv = TLV::without_value(tag.into());

        Self { tlv }
    }
}

impl From<MessageDeliveryRequestTLV> for TLV {
    fn from(tlv: MessageDeliveryRequestTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageDeliveryResponseTLV {
    tlv: TLV,
}

impl MessageDeliveryResponseTLV {
    pub fn new(value: MessageDeliveryResponseTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn without_value(tag: MessageDeliveryResponseTLVTag) -> Self {
        let tlv = TLV::without_value(tag.into());

        Self { tlv }
    }
}

impl From<MessageDeliveryResponseTLV> for TLV {
    fn from(tlv: MessageDeliveryResponseTLV) -> Self {
        tlv.tlv
    }
}
