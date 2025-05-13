use super::{Tlv, TlvValue};
use crate::{
    commands::{
        tlvs::tlv_tag::TlvTag,
        types::{
            addr_subunit::AddrSubunit, callback_num_pres_ind::CallbackNumPresInd,
            dest_addr_np_resolution::DestAddrNpResolution, dpf_result::DpfResult,
            its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
            language_indicator::LanguageIndicator, message_state::MessageState,
            network_error_code::NetworkErrorCode, payload_type::PayloadType,
            privacy_indicator::PrivacyIndicator, sub_address::Subaddress,
            ussd_service_op::UssdServiceOp, MessagePayload, UserMessageReference,
        },
    },
    types::{COctetString, OctetString},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryRequestTlvTag {
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

impl From<MessageDeliveryRequestTlvTag> for TlvTag {
    fn from(value: MessageDeliveryRequestTlvTag) -> Self {
        match value {
            MessageDeliveryRequestTlvTag::CallbackNum => TlvTag::CallbackNum,
            MessageDeliveryRequestTlvTag::CallbackNumAtag => TlvTag::CallbackNumAtag,
            MessageDeliveryRequestTlvTag::CallbackNumPresInd => TlvTag::CallbackNumPresInd,
            MessageDeliveryRequestTlvTag::DestAddrNpCountry => TlvTag::DestAddrNpCountry,
            MessageDeliveryRequestTlvTag::DestAddrNpInformation => TlvTag::DestAddrNpInformation,
            MessageDeliveryRequestTlvTag::DestAddrNpResolution => TlvTag::DestAddrNpResolution,
            MessageDeliveryRequestTlvTag::DestAddrSubunit => TlvTag::DestAddrSubunit,
            MessageDeliveryRequestTlvTag::DestNetworkId => TlvTag::DestNetworkId,
            MessageDeliveryRequestTlvTag::DestNodeId => TlvTag::DestNodeId,
            MessageDeliveryRequestTlvTag::DestSubaddress => TlvTag::DestSubaddress,
            MessageDeliveryRequestTlvTag::DestPort => TlvTag::DestPort,
            MessageDeliveryRequestTlvTag::DpfResult => TlvTag::DpfResult,
            MessageDeliveryRequestTlvTag::ItsReplyType => TlvTag::ItsReplyType,
            MessageDeliveryRequestTlvTag::ItsSessionInfo => TlvTag::ItsSessionInfo,
            MessageDeliveryRequestTlvTag::LanguageIndicator => TlvTag::LanguageIndicator,
            MessageDeliveryRequestTlvTag::MessagePayload => TlvTag::MessagePayload,
            MessageDeliveryRequestTlvTag::MessageState => TlvTag::MessageState,
            MessageDeliveryRequestTlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode,
            MessageDeliveryRequestTlvTag::PayloadType => TlvTag::PayloadType,
            MessageDeliveryRequestTlvTag::PrivacyIndicator => TlvTag::PrivacyIndicator,
            MessageDeliveryRequestTlvTag::ReceiptedMessageId => TlvTag::ReceiptedMessageId,
            MessageDeliveryRequestTlvTag::SarMsgRefNum => TlvTag::SarMsgRefNum,
            MessageDeliveryRequestTlvTag::SarSegmentSeqnum => TlvTag::SarSegmentSeqnum,
            MessageDeliveryRequestTlvTag::SarTotalSegments => TlvTag::SarTotalSegments,
            MessageDeliveryRequestTlvTag::SourceAddrSubunit => TlvTag::SourceAddrSubunit,
            MessageDeliveryRequestTlvTag::SourceNetworkId => TlvTag::SourceNetworkId,
            MessageDeliveryRequestTlvTag::SourceNodeId => TlvTag::SourceNodeId,
            MessageDeliveryRequestTlvTag::SourcePort => TlvTag::SourcePort,
            MessageDeliveryRequestTlvTag::SourceSubaddress => TlvTag::SourceSubaddress,
            MessageDeliveryRequestTlvTag::UserMessageReference => TlvTag::UserMessageReference,
            MessageDeliveryRequestTlvTag::UserResponseCode => TlvTag::UserResponseCode,
            MessageDeliveryRequestTlvTag::UssdServiceOp => TlvTag::UssdServiceOp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryRequestTlvValue {
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
    MessagePayload(MessagePayload),
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
    UserMessageReference(UserMessageReference),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
}

impl From<MessageDeliveryRequestTlvValue> for TlvValue {
    fn from(value: MessageDeliveryRequestTlvValue) -> Self {
        match value {
            MessageDeliveryRequestTlvValue::CallbackNum(value) => TlvValue::CallbackNum(value),
            MessageDeliveryRequestTlvValue::CallbackNumAtag(value) => {
                TlvValue::CallbackNumAtag(value)
            }
            MessageDeliveryRequestTlvValue::CallbackNumPresInd(value) => {
                TlvValue::CallbackNumPresInd(value)
            }
            MessageDeliveryRequestTlvValue::DestAddrNpCountry(value) => {
                TlvValue::DestAddrNpCountry(value)
            }
            MessageDeliveryRequestTlvValue::DestAddrNpInformation(value) => {
                TlvValue::DestAddrNpInformation(value)
            }
            MessageDeliveryRequestTlvValue::DestAddrNpResolution(value) => {
                TlvValue::DestAddrNpResolution(value)
            }
            MessageDeliveryRequestTlvValue::DestAddrSubunit(value) => {
                TlvValue::DestAddrSubunit(value)
            }
            MessageDeliveryRequestTlvValue::DestNetworkId(value) => TlvValue::DestNetworkId(value),
            MessageDeliveryRequestTlvValue::DestNodeId(value) => TlvValue::DestNodeId(value),
            MessageDeliveryRequestTlvValue::DestSubaddress(value) => {
                TlvValue::DestSubaddress(value)
            }
            MessageDeliveryRequestTlvValue::DestPort(value) => TlvValue::DestPort(value),
            MessageDeliveryRequestTlvValue::DpfResult(value) => TlvValue::DpfResult(value),
            MessageDeliveryRequestTlvValue::ItsReplyType(value) => TlvValue::ItsReplyType(value),
            MessageDeliveryRequestTlvValue::ItsSessionInfo(value) => {
                TlvValue::ItsSessionInfo(value)
            }
            MessageDeliveryRequestTlvValue::LanguageIndicator(value) => {
                TlvValue::LanguageIndicator(value)
            }
            MessageDeliveryRequestTlvValue::MessagePayload(value) => {
                TlvValue::MessagePayload(value)
            }
            MessageDeliveryRequestTlvValue::MessageState(value) => TlvValue::MessageState(value),
            MessageDeliveryRequestTlvValue::NetworkErrorCode(value) => {
                TlvValue::NetworkErrorCode(value)
            }
            MessageDeliveryRequestTlvValue::PayloadType(value) => TlvValue::PayloadType(value),
            MessageDeliveryRequestTlvValue::PrivacyIndicator(value) => {
                TlvValue::PrivacyIndicator(value)
            }
            MessageDeliveryRequestTlvValue::ReceiptedMessageId(value) => {
                TlvValue::ReceiptedMessageId(value)
            }
            MessageDeliveryRequestTlvValue::SarMsgRefNum(value) => TlvValue::SarMsgRefNum(value),
            MessageDeliveryRequestTlvValue::SarSegmentSeqnum(value) => {
                TlvValue::SarSegmentSeqnum(value)
            }
            MessageDeliveryRequestTlvValue::SarTotalSegments(value) => {
                TlvValue::SarTotalSegments(value)
            }
            MessageDeliveryRequestTlvValue::SourceAddrSubunit(value) => {
                TlvValue::SourceAddrSubunit(value)
            }
            MessageDeliveryRequestTlvValue::SourceNetworkId(value) => {
                TlvValue::SourceNetworkId(value)
            }
            MessageDeliveryRequestTlvValue::SourceNodeId(value) => TlvValue::SourceNodeId(value),
            MessageDeliveryRequestTlvValue::SourcePort(value) => TlvValue::SourcePort(value),
            MessageDeliveryRequestTlvValue::SourceSubaddress(value) => {
                TlvValue::SourceSubaddress(value)
            }
            MessageDeliveryRequestTlvValue::UserMessageReference(value) => {
                TlvValue::UserMessageReference(value)
            }
            MessageDeliveryRequestTlvValue::UserResponseCode(value) => {
                TlvValue::UserResponseCode(value)
            }
            MessageDeliveryRequestTlvValue::UssdServiceOp(value) => TlvValue::UssdServiceOp(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageDeliveryRequestTlv {
    tlv: Tlv,
}

impl MessageDeliveryRequestTlv {
    pub fn new(value: MessageDeliveryRequestTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: MessageDeliveryRequestTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<MessageDeliveryRequestTlvTag> for Tlv {
    fn from(tag: MessageDeliveryRequestTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<MessageDeliveryRequestTlvValue> for MessageDeliveryRequestTlv {
    fn from(value: MessageDeliveryRequestTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<MessageDeliveryRequestTlvValue> for Tlv {
    fn from(value: MessageDeliveryRequestTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<MessageDeliveryRequestTlv> for Tlv {
    fn from(tlv: MessageDeliveryRequestTlv) -> Self {
        tlv.tlv
    }
}
