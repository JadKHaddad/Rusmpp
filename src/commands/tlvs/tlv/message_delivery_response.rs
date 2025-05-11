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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryResponseTlvTag {
    AdditionalStatusInfoText,
    DeliveryFailureReason,
    NetworkErrorCode,
}

impl From<MessageDeliveryResponseTlvTag> for TlvTag {
    fn from(value: MessageDeliveryResponseTlvTag) -> Self {
        match value {
            MessageDeliveryResponseTlvTag::AdditionalStatusInfoText => {
                TlvTag::AdditionalStatusInfoText
            }
            MessageDeliveryResponseTlvTag::DeliveryFailureReason => TlvTag::DeliveryFailureReason,
            MessageDeliveryResponseTlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode,
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
