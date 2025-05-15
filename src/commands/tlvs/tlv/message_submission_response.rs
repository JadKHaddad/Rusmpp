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

crate::create! {
    #[repr(u16)]
    @[skip_test]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MessageSubmissionResponseTlvTag {
        AdditionalStatusInfoText = 0x001D,
        DpfResult = 0x0420,
        NetworkErrorCode = 0x0423,
        DeliveryFailureReason = 0x0425,
        Other(u16),
    }
}

impl From<u16> for MessageSubmissionResponseTlvTag {
    fn from(tag: u16) -> Self {
        match tag {
            0x001D => MessageSubmissionResponseTlvTag::AdditionalStatusInfoText,
            0x0420 => MessageSubmissionResponseTlvTag::DpfResult,
            0x0423 => MessageSubmissionResponseTlvTag::NetworkErrorCode,
            0x0425 => MessageSubmissionResponseTlvTag::DeliveryFailureReason,
            other => MessageSubmissionResponseTlvTag::Other(other),
        }
    }
}

impl From<MessageSubmissionResponseTlvTag> for u16 {
    fn from(tag: MessageSubmissionResponseTlvTag) -> Self {
        match tag {
            MessageSubmissionResponseTlvTag::AdditionalStatusInfoText => 0x001D,
            MessageSubmissionResponseTlvTag::DpfResult => 0x0420,
            MessageSubmissionResponseTlvTag::NetworkErrorCode => 0x0423,
            MessageSubmissionResponseTlvTag::DeliveryFailureReason => 0x0425,
            MessageSubmissionResponseTlvTag::Other(other) => other,
        }
    }
}

impl From<MessageSubmissionResponseTlvTag> for TlvTag {
    fn from(tag: MessageSubmissionResponseTlvTag) -> Self {
        match tag {
            MessageSubmissionResponseTlvTag::AdditionalStatusInfoText => {
                TlvTag::AdditionalStatusInfoText
            }
            MessageSubmissionResponseTlvTag::DpfResult => TlvTag::DpfResult,
            MessageSubmissionResponseTlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode,
            MessageSubmissionResponseTlvTag::DeliveryFailureReason => TlvTag::DeliveryFailureReason,
            MessageSubmissionResponseTlvTag::Other(other) => TlvTag::Other(other),
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
