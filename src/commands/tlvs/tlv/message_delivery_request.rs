use super::{TLVValue, TLV};
use crate::{
    commands::{
        tlvs::tlv_tag::TLVTag,
        types::{
            addr_subunit::AddrSubunit, callback_num_pres_ind::CallbackNumPresInd,
            dest_addr_np_resolution::DestAddrNpResolution, dpf_result::DpfResult,
            its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
            language_indicator::LanguageIndicator, message_state::MessageState,
            network_error_code::NetworkErrorCode, payload_type::PayloadType,
            privacy_indicator::PrivacyIndicator, subaddress::Subaddress,
            ussd_service_op::UssdServiceOp,
        },
    },
    types::{
        c_octet_string::COctetString, no_fixed_size_octet_string::NoFixedSizeOctetString,
        octet_string::OctetString,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryRequestTLVTag {
    CallbackNum,
    CallbackNumAtag,
    CallbackNumPresInd,
    DestAddrNpCountry,
    DestAddrNpInformation,
    DestAddrNpResolution,
    DestAddrSubunit,
    DestNetworkId,
    DestNodeId,
    DestSubaddress,
    DestPort,
    DpfResult,
    ItsReplyType,
    ItsSessionInfo,
    LanguageIndicator,
    MessagePayload,
    MessageState,
    NetworkErrorCode,
    PayloadType,
    PrivacyIndicator,
    ReceiptedMessageId,
    SarMsgRefNum,
    SarSegmentSeqnum,
    SarTotalSegments,
    SourceAddrSubunit,
    SourceNetworkId,
    SourceNodeId,
    SourcePort,
    SourceSubaddress,
    UserMessageReference,
    UserResponseCode,
    UssdServiceOp,
}

impl From<MessageDeliveryRequestTLVTag> for TLVTag {
    fn from(value: MessageDeliveryRequestTLVTag) -> Self {
        match value {
            MessageDeliveryRequestTLVTag::CallbackNum => TLVTag::CallbackNum,
            MessageDeliveryRequestTLVTag::CallbackNumAtag => TLVTag::CallbackNumAtag,
            MessageDeliveryRequestTLVTag::CallbackNumPresInd => TLVTag::CallbackNumPresInd,
            MessageDeliveryRequestTLVTag::DestAddrNpCountry => TLVTag::DestAddrNpCountry,
            MessageDeliveryRequestTLVTag::DestAddrNpInformation => TLVTag::DestAddrNpInformation,
            MessageDeliveryRequestTLVTag::DestAddrNpResolution => TLVTag::DestAddrNpResolution,
            MessageDeliveryRequestTLVTag::DestAddrSubunit => TLVTag::DestAddrSubunit,
            MessageDeliveryRequestTLVTag::DestNetworkId => TLVTag::DestNetworkId,
            MessageDeliveryRequestTLVTag::DestNodeId => TLVTag::DestNodeId,
            MessageDeliveryRequestTLVTag::DestSubaddress => TLVTag::DestSubaddress,
            MessageDeliveryRequestTLVTag::DestPort => TLVTag::DestPort,
            MessageDeliveryRequestTLVTag::DpfResult => TLVTag::DpfResult,
            MessageDeliveryRequestTLVTag::ItsReplyType => TLVTag::ItsReplyType,
            MessageDeliveryRequestTLVTag::ItsSessionInfo => TLVTag::ItsSessionInfo,
            MessageDeliveryRequestTLVTag::LanguageIndicator => TLVTag::LanguageIndicator,
            MessageDeliveryRequestTLVTag::MessagePayload => TLVTag::MessagePayload,
            MessageDeliveryRequestTLVTag::MessageState => TLVTag::MessageState,
            MessageDeliveryRequestTLVTag::NetworkErrorCode => TLVTag::NetworkErrorCode,
            MessageDeliveryRequestTLVTag::PayloadType => TLVTag::PayloadType,
            MessageDeliveryRequestTLVTag::PrivacyIndicator => TLVTag::PrivacyIndicator,
            MessageDeliveryRequestTLVTag::ReceiptedMessageId => TLVTag::ReceiptedMessageId,
            MessageDeliveryRequestTLVTag::SarMsgRefNum => TLVTag::SarMsgRefNum,
            MessageDeliveryRequestTLVTag::SarSegmentSeqnum => TLVTag::SarSegmentSeqnum,
            MessageDeliveryRequestTLVTag::SarTotalSegments => TLVTag::SarTotalSegments,
            MessageDeliveryRequestTLVTag::SourceAddrSubunit => TLVTag::SourceAddrSubunit,
            MessageDeliveryRequestTLVTag::SourceNetworkId => TLVTag::SourceNetworkId,
            MessageDeliveryRequestTLVTag::SourceNodeId => TLVTag::SourceNodeId,
            MessageDeliveryRequestTLVTag::SourcePort => TLVTag::SourcePort,
            MessageDeliveryRequestTLVTag::SourceSubaddress => TLVTag::SourceSubaddress,
            MessageDeliveryRequestTLVTag::UserMessageReference => TLVTag::UserMessageReference,
            MessageDeliveryRequestTLVTag::UserResponseCode => TLVTag::UserResponseCode,
            MessageDeliveryRequestTLVTag::UssdServiceOp => TLVTag::UssdServiceOp,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageDeliveryRequestTLV {
    tlv: TLV,
}

impl MessageDeliveryRequestTLV {
    pub fn new(value: MessageDeliveryRequestTLVValue) -> Self {
        let value = TLVValue::from(value);
        let tlv = TLV::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: MessageDeliveryRequestTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        let tlv = TLV::from(tag);

        Self { tlv }
    }
}

impl From<MessageDeliveryRequestTLVTag> for TLV {
    fn from(tag: MessageDeliveryRequestTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        TLV::from(tag)
    }
}

impl From<MessageDeliveryRequestTLVValue> for TLV {
    fn from(value: MessageDeliveryRequestTLVValue) -> Self {
        let value = TLVValue::from(value);
        TLV::from(value)
    }
}

impl From<MessageDeliveryRequestTLV> for TLV {
    fn from(tlv: MessageDeliveryRequestTLV) -> Self {
        tlv.tlv
    }
}
