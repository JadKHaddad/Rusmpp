use super::{Tlv, TlvValue};
use crate::{
    commands::{
        tlvs::tlv_tag::TlvTag,
        types::{
            delivery_failure_reason::DeliveryFailureReason, network_error_code::NetworkErrorCode,
        },
    },
    types::COctetString,
};

crate::create! {
    #[repr(u16)]
    @[skip_test]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MessageDeliveryResponseTlvTag {
        AdditionalStatusInfoText = 0x001D,
        DeliveryFailureReason = 0x0425,
        NetworkErrorCode = 0x0423,
        Other(u16),
    }
}

impl From<u16> for MessageDeliveryResponseTlvTag {
    fn from(tag: u16) -> Self {
        match tag {
            0x001D => MessageDeliveryResponseTlvTag::AdditionalStatusInfoText,
            0x0425 => MessageDeliveryResponseTlvTag::DeliveryFailureReason,
            0x0423 => MessageDeliveryResponseTlvTag::NetworkErrorCode,
            other => MessageDeliveryResponseTlvTag::Other(other),
        }
    }
}

impl From<MessageDeliveryResponseTlvTag> for u16 {
    fn from(tag: MessageDeliveryResponseTlvTag) -> Self {
        match tag {
            MessageDeliveryResponseTlvTag::AdditionalStatusInfoText => 0x001D,
            MessageDeliveryResponseTlvTag::DeliveryFailureReason => 0x0425,
            MessageDeliveryResponseTlvTag::NetworkErrorCode => 0x0423,
            MessageDeliveryResponseTlvTag::Other(other) => other,
        }
    }
}

impl From<MessageDeliveryResponseTlvTag> for TlvTag {
    fn from(tag: MessageDeliveryResponseTlvTag) -> Self {
        match tag {
            MessageDeliveryResponseTlvTag::AdditionalStatusInfoText => {
                TlvTag::AdditionalStatusInfoText
            }
            MessageDeliveryResponseTlvTag::DeliveryFailureReason => TlvTag::DeliveryFailureReason,
            MessageDeliveryResponseTlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode,
            MessageDeliveryResponseTlvTag::Other(other) => TlvTag::Other(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryResponseTlvValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    NetworkErrorCode(NetworkErrorCode),
}

impl From<MessageDeliveryResponseTlvValue> for TlvValue {
    fn from(value: MessageDeliveryResponseTlvValue) -> Self {
        match value {
            MessageDeliveryResponseTlvValue::AdditionalStatusInfoText(v) => {
                TlvValue::AdditionalStatusInfoText(v)
            }
            MessageDeliveryResponseTlvValue::DeliveryFailureReason(v) => {
                TlvValue::DeliveryFailureReason(v)
            }
            MessageDeliveryResponseTlvValue::NetworkErrorCode(v) => TlvValue::NetworkErrorCode(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageDeliveryResponseTlv {
    tlv: Tlv,
}

impl MessageDeliveryResponseTlv {
    pub fn new(value: MessageDeliveryResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: MessageDeliveryResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<MessageDeliveryResponseTlvTag> for Tlv {
    fn from(tag: MessageDeliveryResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<MessageDeliveryResponseTlvValue> for MessageDeliveryResponseTlv {
    fn from(value: MessageDeliveryResponseTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<MessageDeliveryResponseTlvValue> for Tlv {
    fn from(value: MessageDeliveryResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<MessageDeliveryResponseTlv> for Tlv {
    fn from(tlv: MessageDeliveryResponseTlv) -> Self {
        tlv.tlv
    }
}
