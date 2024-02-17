use super::TLV;
use crate::{
    commands::{
        tlvs::{tlv_tag::TLVTag, tlv_value::TLVValue},
        types::{
            delivery_failure_reason::DeliveryFailureReason, dpf_result::DpfResult,
            network_error_code::NetworkErrorCode,
        },
    },
    types::c_octet_string::COctetString,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionResponseTLVTag {
    AdditionalStatusInfoText,
    DeliveryFailureReason,
    DpfResult,
    NetworkErrorCode,
}

impl From<MessageSubmissionResponseTLVTag> for TLVTag {
    fn from(value: MessageSubmissionResponseTLVTag) -> Self {
        match value {
            MessageSubmissionResponseTLVTag::AdditionalStatusInfoText => {
                TLVTag::AdditionalStatusInfoText
            }
            MessageSubmissionResponseTLVTag::DeliveryFailureReason => TLVTag::DeliveryFailureReason,
            MessageSubmissionResponseTLVTag::DpfResult => TLVTag::DpfResult,
            MessageSubmissionResponseTLVTag::NetworkErrorCode => TLVTag::NetworkErrorCode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionResponseTLVValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    DpfResult(DpfResult),
    NetworkErrorCode(NetworkErrorCode),
}

impl From<MessageSubmissionResponseTLVValue> for TLVValue {
    fn from(value: MessageSubmissionResponseTLVValue) -> Self {
        match value {
            MessageSubmissionResponseTLVValue::AdditionalStatusInfoText(value) => {
                TLVValue::AdditionalStatusInfoText(value)
            }
            MessageSubmissionResponseTLVValue::DeliveryFailureReason(value) => {
                TLVValue::DeliveryFailureReason(value)
            }
            MessageSubmissionResponseTLVValue::DpfResult(value) => TLVValue::DpfResult(value),
            MessageSubmissionResponseTLVValue::NetworkErrorCode(value) => {
                TLVValue::NetworkErrorCode(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionResponseTLV {
    tlv: TLV,
}

impl MessageSubmissionResponseTLV {
    pub fn new(value: MessageSubmissionResponseTLVValue) -> Self {
        let value = TLVValue::from(value);
        let tlv = TLV::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: MessageSubmissionResponseTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        let tlv = TLV::from(tag);

        Self { tlv }
    }
}

impl From<MessageSubmissionResponseTLVTag> for TLV {
    fn from(tag: MessageSubmissionResponseTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        TLV::from(tag)
    }
}

impl From<MessageSubmissionResponseTLVValue> for TLV {
    fn from(value: MessageSubmissionResponseTLVValue) -> Self {
        let value = TLVValue::from(value);
        TLV::from(value)
    }
}

impl From<MessageSubmissionResponseTLV> for TLV {
    fn from(tlv: MessageSubmissionResponseTLV) -> Self {
        tlv.tlv
    }
}
