use crate::{
    commands::types::{
        addr_subunit::AddrSubunit, callback_num_pres_ind::CallbackNumPresInd,
        dest_addr_np_resolution::DestAddrNpResolution, dpf_result::DpfResult,
        its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
        language_indicator::LanguageIndicator, message_state::MessageState,
        network_error_code::NetworkErrorCode, payload_type::PayloadType,
        privacy_indicator::PrivacyIndicator, subaddress::Subaddress,
        ussd_service_op::UssdServiceOp,
    },
    types::{
        c_octet_string::COctetString, no_fixed_size_octet_string::NoFixedSizeOctetString,
        octet_string::OctetString,
    },
};

use super::TLVValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryRequestTLVValue {
    CallbackNum(OctetString<4, 19>),
    CallbackNumAtag(OctetString<0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrNpCountry(OctetString<1, 5>),
    DestAddrNpInformation(OctetString<0, 10>),
    DestAddrNpResolution(DestAddrNpResolution),
    DestAddrSubunit(AddrSubunit),
    DestNetworkId(COctetString<7, 66>),
    DestNodeId(OctetString<6, 6>),
    DestSubaddress(Subaddress),
    DestPort(u16),
    DpfResult(DpfResult),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(NoFixedSizeOctetString),
    MessageState(MessageState),
    NetworkErrorCode(NetworkErrorCode),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    ReceiptedMessageId(COctetString<1, 65>),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    SourceAddrSubunit(AddrSubunit),
    SourceNetworkId(COctetString<7, 66>),
    SourceNodeId(OctetString<6, 6>),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    UserMessageReference(u16),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
}

impl From<MessageDeliveryRequestTLVValue> for TLVValue {
    fn from(value: MessageDeliveryRequestTLVValue) -> Self {
        match value {
            MessageDeliveryRequestTLVValue::CallbackNum(value) => TLVValue::CallbackNum(value),
            MessageDeliveryRequestTLVValue::CallbackNumAtag(value) => {
                TLVValue::CallbackNumAtag(value)
            }
            MessageDeliveryRequestTLVValue::CallbackNumPresInd(value) => {
                TLVValue::CallbackNumPresInd(value)
            }
            MessageDeliveryRequestTLVValue::DestAddrNpCountry(value) => {
                TLVValue::DestAddrNpCountry(value)
            }
            MessageDeliveryRequestTLVValue::DestAddrNpInformation(value) => {
                TLVValue::DestAddrNpInformation(value)
            }
            MessageDeliveryRequestTLVValue::DestAddrNpResolution(value) => {
                TLVValue::DestAddrNpResolution(value)
            }
            MessageDeliveryRequestTLVValue::DestAddrSubunit(value) => {
                TLVValue::DestAddrSubunit(value)
            }
            MessageDeliveryRequestTLVValue::DestNetworkId(value) => TLVValue::DestNetworkId(value),
            MessageDeliveryRequestTLVValue::DestNodeId(value) => TLVValue::DestNodeId(value),
            MessageDeliveryRequestTLVValue::DestSubaddress(value) => {
                TLVValue::DestSubaddress(value)
            }
            MessageDeliveryRequestTLVValue::DestPort(value) => TLVValue::DestPort(value),
            MessageDeliveryRequestTLVValue::DpfResult(value) => TLVValue::DpfResult(value),
            MessageDeliveryRequestTLVValue::ItsReplyType(value) => TLVValue::ItsReplyType(value),
            MessageDeliveryRequestTLVValue::ItsSessionInfo(value) => {
                TLVValue::ItsSessionInfo(value)
            }
            MessageDeliveryRequestTLVValue::LanguageIndicator(value) => {
                TLVValue::LanguageIndicator(value)
            }
            MessageDeliveryRequestTLVValue::MessagePayload(value) => {
                TLVValue::MessagePayload(value)
            }
            MessageDeliveryRequestTLVValue::MessageState(value) => TLVValue::MessageState(value),
            MessageDeliveryRequestTLVValue::NetworkErrorCode(value) => {
                TLVValue::NetworkErrorCode(value)
            }
            MessageDeliveryRequestTLVValue::PayloadType(value) => TLVValue::PayloadType(value),
            MessageDeliveryRequestTLVValue::PrivacyIndicator(value) => {
                TLVValue::PrivacyIndicator(value)
            }
            MessageDeliveryRequestTLVValue::ReceiptedMessageId(value) => {
                TLVValue::ReceiptedMessageId(value)
            }
            MessageDeliveryRequestTLVValue::SarMsgRefNum(value) => TLVValue::SarMsgRefNum(value),
            MessageDeliveryRequestTLVValue::SarSegmentSeqnum(value) => {
                TLVValue::SarSegmentSeqnum(value)
            }
            MessageDeliveryRequestTLVValue::SarTotalSegments(value) => {
                TLVValue::SarTotalSegments(value)
            }
            MessageDeliveryRequestTLVValue::SourceAddrSubunit(value) => {
                TLVValue::SourceAddrSubunit(value)
            }
            MessageDeliveryRequestTLVValue::SourceNetworkId(value) => {
                TLVValue::SourceNetworkId(value)
            }
            MessageDeliveryRequestTLVValue::SourceNodeId(value) => TLVValue::SourceNodeId(value),
            MessageDeliveryRequestTLVValue::SourcePort(value) => TLVValue::SourcePort(value),
            MessageDeliveryRequestTLVValue::SourceSubaddress(value) => {
                TLVValue::SourceSubaddress(value)
            }
            MessageDeliveryRequestTLVValue::UserMessageReference(value) => {
                TLVValue::UserMessageReference(value)
            }
            MessageDeliveryRequestTLVValue::UserResponseCode(value) => {
                TLVValue::UserResponseCode(value)
            }
            MessageDeliveryRequestTLVValue::UssdServiceOp(value) => TLVValue::UssdServiceOp(value),
        }
    }
}
