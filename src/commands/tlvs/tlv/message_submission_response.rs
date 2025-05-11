use super::Tlv;
use crate::{
    commands::{
        tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
        types::{
            delivery_failure_reason::DeliveryFailureReason, dpf_result::DpfResult,
            network_error_code::NetworkErrorCode,
        },
    },
    types::COctetString,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionResponseTlvTag {
    AdditionalStatusInfoText,
    DeliveryFailureReason,
    DpfResult,
    NetworkErrorCode,
}

impl From<MessageSubmissionResponseTlvTag> for TlvTag {
    fn from(value: MessageSubmissionResponseTlvTag) -> Self {
        match value {
            MessageSubmissionResponseTlvTag::AdditionalStatusInfoText => {
                TlvTag::AdditionalStatusInfoText
            }
            MessageSubmissionResponseTlvTag::DeliveryFailureReason => TlvTag::DeliveryFailureReason,
            MessageSubmissionResponseTlvTag::DpfResult => TlvTag::DpfResult,
            MessageSubmissionResponseTlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionResponseTlvValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    DpfResult(DpfResult),
    NetworkErrorCode(NetworkErrorCode),
}

impl From<MessageSubmissionResponseTlvValue> for TlvValue {
    fn from(value: MessageSubmissionResponseTlvValue) -> Self {
        match value {
            MessageSubmissionResponseTlvValue::AdditionalStatusInfoText(value) => {
                TlvValue::AdditionalStatusInfoText(value)
            }
            MessageSubmissionResponseTlvValue::DeliveryFailureReason(value) => {
                TlvValue::DeliveryFailureReason(value)
            }
            MessageSubmissionResponseTlvValue::DpfResult(value) => TlvValue::DpfResult(value),
            MessageSubmissionResponseTlvValue::NetworkErrorCode(value) => {
                TlvValue::NetworkErrorCode(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionResponseTlv {
    tlv: Tlv,
}

impl MessageSubmissionResponseTlv {
    pub fn new(value: MessageSubmissionResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: MessageSubmissionResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<MessageSubmissionResponseTlvTag> for Tlv {
    fn from(tag: MessageSubmissionResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<MessageSubmissionResponseTlvValue> for MessageSubmissionResponseTlv {
    fn from(value: MessageSubmissionResponseTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<MessageSubmissionResponseTlvValue> for Tlv {
    fn from(value: MessageSubmissionResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<MessageSubmissionResponseTlv> for Tlv {
    fn from(tlv: MessageSubmissionResponseTlv) -> Self {
        tlv.tlv
    }
}
