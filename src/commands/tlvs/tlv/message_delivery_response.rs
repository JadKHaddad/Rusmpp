use super::{TLVValue, TLV};
use crate::{
    commands::{
        tlvs::tlv_tag::TLVTag,
        types::{
            delivery_failure_reason::DeliveryFailureReason, network_error_code::NetworkErrorCode,
        },
    },
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryResponseTLVTag {
    AdditionalStatusInfoText,
    DeliveryFailureReason,
    NetworkErrorCode,
}

impl From<MessageDeliveryResponseTLVTag> for TLVTag {
    fn from(value: MessageDeliveryResponseTLVTag) -> Self {
        match value {
            MessageDeliveryResponseTLVTag::AdditionalStatusInfoText => {
                TLVTag::AdditionalStatusInfoText
            }
            MessageDeliveryResponseTLVTag::DeliveryFailureReason => TLVTag::DeliveryFailureReason,
            MessageDeliveryResponseTLVTag::NetworkErrorCode => TLVTag::NetworkErrorCode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryResponseTLVValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    NetworkErrorCode(NetworkErrorCode),
}

impl From<MessageDeliveryResponseTLVValue> for TLVValue {
    fn from(value: MessageDeliveryResponseTLVValue) -> Self {
        match value {
            MessageDeliveryResponseTLVValue::AdditionalStatusInfoText(v) => {
                TLVValue::AdditionalStatusInfoText(v)
            }
            MessageDeliveryResponseTLVValue::DeliveryFailureReason(v) => {
                TLVValue::DeliveryFailureReason(v)
            }
            MessageDeliveryResponseTLVValue::NetworkErrorCode(v) => TLVValue::NetworkErrorCode(v),
        }
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
