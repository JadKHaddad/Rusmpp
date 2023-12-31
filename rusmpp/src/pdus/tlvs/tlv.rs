use super::{
    tlv_tag::{
        BroadcastRequestTLVTag, BroadcastResponseTLVTag, CancelBroadcastTLVTag,
        MessageDeliveryRequestTLVTag, MessageDeliveryResponseTLVTag,
        MessageSubmissionRequestTLVTag, MessageSubmissionResponseTLVTag,
        QueryBroadcastResponseTLVTag, TLVTag,
    },
    tlv_value::{
        BroadcastRequestTLVValue, BroadcastResponseTLVValue, CancelBroadcastTLVValue,
        MessageDeliveryRequestTLVValue, MessageDeliveryResponseTLVValue,
        MessageSubmissionRequestTLVValue, MessageSubmissionResponseTLVValue,
        QueryBroadcastResponseTLVValue, TLVValue,
    },
};
use crate::io::length::IoLength;
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

/// A Tagged Length Value Field is a special composite field
/// that comprises of three parts:
/// • A 2-octet Integer (Tag)
/// The tag identifies the parameter.
///
/// • A 2-octet Integer (Length)
/// The length field indicates the length of the value
/// field in octets. Note that this length does not
/// include the length of the tag and length fields.
///
/// • An Octet String (Value)
/// The value field contains the actual data for the
/// TLV field.
///
/// The Tag identifies the parameter. The Length indicates
/// the size of the Value field in octets.
///
/// An example of a TLV is the dest_bearer_type. Its Tag is
/// 0x0007 and has a value size of 1 octet. The value 0x04
/// indicates USSD as a bearer type. In its encoded form,
/// this TLV would appear as follows:
///
/// 0x0007000104
///
/// The first 2 octets 0x0007 identifies the Tag
/// dest_bearer_type. The next two octets 0x0001 indicate
/// the 1-octet length of the value field. The value field 0x04
/// indicates USSD.
///
/// There are two types of NULL encoding for a TLV. The
/// first is a TLV that may not carry a value part. An example
/// of such a TLV is alert_on_message_delivery. This TLV is
/// typically used as an indicator only, i.e. its function is
/// driven by its very presence in the PDU. No data is
/// typically present. However it may carry up to 1 octet of
/// data if required.
///
/// Here are two examples of how this TLV can be encoded,
/// the first example carries a value, the second example
/// does not:
///
/// Tag=0x130C
/// Length=0x0001
/// Value=0x01
///
/// Encoded Format: 0x130C000101
///
/// Tag=0x130C
/// Length=0x0000
/// Value=NULL
///
/// Encoded Format: 0x130C0000
///
/// Note: Only the Tag and Length are encoded. No NULL
/// octets are specified for the zero length Value field.
///
/// If the TLV itself is not required, then it is not encoded at
/// all. The very absence of the TLV from the PDU is the
/// means by which we set the values to NULL.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct TLV {
    tag: TLVTag,
    value_length: u16,
    #[rusmpp_io_read(key=tag, length=(value_length))]
    value: Option<TLVValue>,
}

impl TLV {
    pub fn new(value: TLVValue) -> Self {
        let tag = value.tlv_tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }

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

impl From<TLVValue> for TLV {
    fn from(value: TLVValue) -> Self {
        Self::new(value)
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

    pub fn new_without_value(tag: MessageSubmissionRequestTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

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

    pub fn new_without_value(tag: MessageSubmissionResponseTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

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

    pub fn new_without_value(tag: MessageDeliveryRequestTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

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

    pub fn new_without_value(tag: MessageDeliveryResponseTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

        Self { tlv }
    }
}

impl From<MessageDeliveryResponseTLV> for TLV {
    fn from(tlv: MessageDeliveryResponseTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastRequestTLV {
    tlv: TLV,
}

impl BroadcastRequestTLV {
    pub fn new(value: BroadcastRequestTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn new_without_value(tag: BroadcastRequestTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

        Self { tlv }
    }
}

impl From<BroadcastRequestTLV> for TLV {
    fn from(tlv: BroadcastRequestTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastResponseTLV {
    tlv: TLV,
}

impl BroadcastResponseTLV {
    pub fn new(value: BroadcastResponseTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn new_without_value(tag: BroadcastResponseTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

        Self { tlv }
    }
}

impl From<BroadcastResponseTLV> for TLV {
    fn from(tlv: BroadcastResponseTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueryBroadcastResponseTLV {
    tlv: TLV,
}

impl QueryBroadcastResponseTLV {
    pub fn new(value: QueryBroadcastResponseTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn new_without_value(tag: QueryBroadcastResponseTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

        Self { tlv }
    }
}

impl From<QueryBroadcastResponseTLV> for TLV {
    fn from(tlv: QueryBroadcastResponseTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CancelBroadcastTLV {
    tlv: TLV,
}

impl CancelBroadcastTLV {
    pub fn new(value: CancelBroadcastTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn new_without_value(tag: CancelBroadcastTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

        Self { tlv }
    }
}

impl From<CancelBroadcastTLV> for TLV {
    fn from(tlv: CancelBroadcastTLV) -> Self {
        tlv.tlv
    }
}
