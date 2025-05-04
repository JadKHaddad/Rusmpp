use super::TLV;
use crate::{
    commands::{
        tlvs::{tlv_tag::TLVTag, tlv_value::TLVValue},
        types::{
            addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMsgDelivery,
            bearer_type::BearerType, callback_num_pres_ind::CallbackNumPresInd,
            dest_addr_np_resolution::DestAddrNpResolution, display_time::DisplayTime,
            its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
            language_indicator::LanguageIndicator, more_messages_to_send::MoreMessagesToSend,
            ms_msg_wait_facilities::MsMsgWaitFacilities, ms_validity::MsValidity,
            network_type::NetworkType, number_of_messages::NumberOfMessages,
            payload_type::PayloadType, privacy_indicator::PrivacyIndicator, set_dpf::SetDpf,
            sub_address::Subaddress, ussd_service_op::UssdServiceOp,
        },
    },
    types::{
        any_octet_string::AnyOctetString, c_octet_string::COctetString, octet_string::OctetString,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionRequestTLVTag {
    AlertOnMsgDelivery,
    BillingIdentification,
    CallbackNum,
    CallbackNumAtag,
    CallbackNumPresInd,
    DestAddrNpCountry,
    DestAddrNpInformation,
    DestAddrNpResolution,
    DestAddrSubunit,
    DestBearerType,
    DestNetworkId,
    DestNetworkType,
    DestNodeId,
    DestSubaddress,
    DestTelematicsId,
    DestPort,
    DisplayTime,
    ItsReplyType,
    ItsSessionInfo,
    LanguageIndicator,
    MessagePayload,
    MoreMessagesToSend,
    MsMsgWaitFacilities,
    MsValidity,
    NumberOfMessages,
    PayloadType,
    PrivacyIndicator,
    QosTimeToLive,
    SarMsgRefNum,
    SarSegmentSeqnum,
    SarTotalSegments,
    SetDpf,
    SmsSignal,
    SourceAddrSubunit,
    SourceBearerType,
    SourceNetworkId,
    SourceNetworkType,
    SourceNodeId,
    SourcePort,
    SourceSubaddress,
    SourceTelematicsId,
    UserMessageReference,
    UserResponseCode,
    UssdServiceOp,
}

impl From<MessageSubmissionRequestTLVTag> for TLVTag {
    fn from(value: MessageSubmissionRequestTLVTag) -> Self {
        match value {
            MessageSubmissionRequestTLVTag::AlertOnMsgDelivery => TLVTag::AlertOnMessageDelivery,
            MessageSubmissionRequestTLVTag::BillingIdentification => TLVTag::BillingIdentification,
            MessageSubmissionRequestTLVTag::CallbackNum => TLVTag::CallbackNum,
            MessageSubmissionRequestTLVTag::CallbackNumAtag => TLVTag::CallbackNumAtag,
            MessageSubmissionRequestTLVTag::CallbackNumPresInd => TLVTag::CallbackNumPresInd,
            MessageSubmissionRequestTLVTag::DestAddrNpCountry => TLVTag::DestAddrNpCountry,
            MessageSubmissionRequestTLVTag::DestAddrNpInformation => TLVTag::DestAddrNpInformation,
            MessageSubmissionRequestTLVTag::DestAddrNpResolution => TLVTag::DestAddrNpResolution,
            MessageSubmissionRequestTLVTag::DestAddrSubunit => TLVTag::DestAddrSubunit,
            MessageSubmissionRequestTLVTag::DestBearerType => TLVTag::DestBearerType,
            MessageSubmissionRequestTLVTag::DestNetworkId => TLVTag::DestNetworkId,
            MessageSubmissionRequestTLVTag::DestNetworkType => TLVTag::DestNetworkType,
            MessageSubmissionRequestTLVTag::DestNodeId => TLVTag::DestNodeId,
            MessageSubmissionRequestTLVTag::DestSubaddress => TLVTag::DestSubaddress,
            MessageSubmissionRequestTLVTag::DestTelematicsId => TLVTag::DestTelematicsId,
            MessageSubmissionRequestTLVTag::DestPort => TLVTag::DestPort,
            MessageSubmissionRequestTLVTag::DisplayTime => TLVTag::DisplayTime,
            MessageSubmissionRequestTLVTag::ItsReplyType => TLVTag::ItsReplyType,
            MessageSubmissionRequestTLVTag::ItsSessionInfo => TLVTag::ItsSessionInfo,
            MessageSubmissionRequestTLVTag::LanguageIndicator => TLVTag::LanguageIndicator,
            MessageSubmissionRequestTLVTag::MessagePayload => TLVTag::MessagePayload,
            MessageSubmissionRequestTLVTag::MoreMessagesToSend => TLVTag::MoreMessagesToSend,
            MessageSubmissionRequestTLVTag::MsMsgWaitFacilities => TLVTag::MsMsgWaitFacilities,
            MessageSubmissionRequestTLVTag::MsValidity => TLVTag::MsValidity,
            MessageSubmissionRequestTLVTag::NumberOfMessages => TLVTag::NumberOfMessages,
            MessageSubmissionRequestTLVTag::PayloadType => TLVTag::PayloadType,
            MessageSubmissionRequestTLVTag::PrivacyIndicator => TLVTag::PrivacyIndicator,
            MessageSubmissionRequestTLVTag::QosTimeToLive => TLVTag::QosTimeToLive,
            MessageSubmissionRequestTLVTag::SarMsgRefNum => TLVTag::SarMsgRefNum,
            MessageSubmissionRequestTLVTag::SarSegmentSeqnum => TLVTag::SarSegmentSeqnum,
            MessageSubmissionRequestTLVTag::SarTotalSegments => TLVTag::SarTotalSegments,
            MessageSubmissionRequestTLVTag::SetDpf => TLVTag::SetDpf,
            MessageSubmissionRequestTLVTag::SmsSignal => TLVTag::SmsSignal,
            MessageSubmissionRequestTLVTag::SourceAddrSubunit => TLVTag::SourceAddrSubunit,
            MessageSubmissionRequestTLVTag::SourceBearerType => TLVTag::SourceBearerType,
            MessageSubmissionRequestTLVTag::SourceNetworkId => TLVTag::SourceNetworkId,
            MessageSubmissionRequestTLVTag::SourceNetworkType => TLVTag::SourceNetworkType,
            MessageSubmissionRequestTLVTag::SourceNodeId => TLVTag::SourceNodeId,
            MessageSubmissionRequestTLVTag::SourcePort => TLVTag::SourcePort,
            MessageSubmissionRequestTLVTag::SourceSubaddress => TLVTag::SourceSubaddress,
            MessageSubmissionRequestTLVTag::SourceTelematicsId => TLVTag::SourceTelematicsId,
            MessageSubmissionRequestTLVTag::UserMessageReference => TLVTag::UserMessageReference,
            MessageSubmissionRequestTLVTag::UserResponseCode => TLVTag::UserResponseCode,
            MessageSubmissionRequestTLVTag::UssdServiceOp => TLVTag::UssdServiceOp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionRequestTLVValue {
    AlertOnMsgDelivery(AlertOnMsgDelivery),
    BillingIdentification(OctetString<0, 1024>),
    CallbackNum(OctetString<4, 19>),
    CallbackNumAtag(OctetString<0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrNpCountry(OctetString<1, 5>),
    DestAddrNpInformation(OctetString<0, 10>),
    DestAddrNpResolution(DestAddrNpResolution),
    DestAddrSubunit(AddrSubunit),
    DestBearerType(BearerType),
    DestNetworkId(COctetString<7, 66>),
    DestNetworkType(NetworkType),
    DestNodeId(OctetString<6, 6>),
    DestSubaddress(Subaddress),
    DestTelematicsId(u16),
    DestPort(u16),
    DisplayTime(DisplayTime),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(AnyOctetString),
    MoreMessagesToSend(MoreMessagesToSend),
    MsMsgWaitFacilities(MsMsgWaitFacilities),
    MsValidity(MsValidity),
    NumberOfMessages(NumberOfMessages),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    QosTimeToLive(u32),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    SetDpf(SetDpf),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourceBearerType(BearerType),
    SourceNetworkId(COctetString<7, 66>),
    SourceNetworkType(NetworkType),
    SourceNodeId(OctetString<6, 6>),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    SourceTelematicsId(u16),
    UserMessageReference(u16),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
}

impl From<MessageSubmissionRequestTLVValue> for TLVValue {
    fn from(value: MessageSubmissionRequestTLVValue) -> Self {
        match value {
            MessageSubmissionRequestTLVValue::AlertOnMsgDelivery(value) => {
                TLVValue::AlertOnMessageDelivery(value)
            }
            MessageSubmissionRequestTLVValue::BillingIdentification(value) => {
                TLVValue::BillingIdentification(value)
            }
            MessageSubmissionRequestTLVValue::CallbackNum(value) => TLVValue::CallbackNum(value),
            MessageSubmissionRequestTLVValue::CallbackNumAtag(value) => {
                TLVValue::CallbackNumAtag(value)
            }
            MessageSubmissionRequestTLVValue::CallbackNumPresInd(value) => {
                TLVValue::CallbackNumPresInd(value)
            }
            MessageSubmissionRequestTLVValue::DestAddrNpCountry(value) => {
                TLVValue::DestAddrNpCountry(value)
            }
            MessageSubmissionRequestTLVValue::DestAddrNpInformation(value) => {
                TLVValue::DestAddrNpInformation(value)
            }
            MessageSubmissionRequestTLVValue::DestAddrNpResolution(value) => {
                TLVValue::DestAddrNpResolution(value)
            }
            MessageSubmissionRequestTLVValue::DestAddrSubunit(value) => {
                TLVValue::DestAddrSubunit(value)
            }
            MessageSubmissionRequestTLVValue::DestBearerType(value) => {
                TLVValue::DestBearerType(value)
            }
            MessageSubmissionRequestTLVValue::DestNetworkId(value) => {
                TLVValue::DestNetworkId(value)
            }
            MessageSubmissionRequestTLVValue::DestNetworkType(value) => {
                TLVValue::DestNetworkType(value)
            }
            MessageSubmissionRequestTLVValue::DestNodeId(value) => TLVValue::DestNodeId(value),
            MessageSubmissionRequestTLVValue::DestSubaddress(value) => {
                TLVValue::DestSubaddress(value)
            }
            MessageSubmissionRequestTLVValue::DestTelematicsId(value) => {
                TLVValue::DestTelematicsId(value)
            }
            MessageSubmissionRequestTLVValue::DestPort(value) => TLVValue::DestPort(value),
            MessageSubmissionRequestTLVValue::DisplayTime(value) => TLVValue::DisplayTime(value),
            MessageSubmissionRequestTLVValue::ItsReplyType(value) => TLVValue::ItsReplyType(value),
            MessageSubmissionRequestTLVValue::ItsSessionInfo(value) => {
                TLVValue::ItsSessionInfo(value)
            }
            MessageSubmissionRequestTLVValue::LanguageIndicator(value) => {
                TLVValue::LanguageIndicator(value)
            }
            MessageSubmissionRequestTLVValue::MessagePayload(value) => {
                TLVValue::MessagePayload(value)
            }
            MessageSubmissionRequestTLVValue::MoreMessagesToSend(value) => {
                TLVValue::MoreMessagesToSend(value)
            }
            MessageSubmissionRequestTLVValue::MsMsgWaitFacilities(value) => {
                TLVValue::MsMsgWaitFacilities(value)
            }
            MessageSubmissionRequestTLVValue::MsValidity(value) => TLVValue::MsValidity(value),
            MessageSubmissionRequestTLVValue::NumberOfMessages(value) => {
                TLVValue::NumberOfMessages(value)
            }
            MessageSubmissionRequestTLVValue::PayloadType(value) => TLVValue::PayloadType(value),
            MessageSubmissionRequestTLVValue::PrivacyIndicator(value) => {
                TLVValue::PrivacyIndicator(value)
            }
            MessageSubmissionRequestTLVValue::QosTimeToLive(value) => {
                TLVValue::QosTimeToLive(value)
            }
            MessageSubmissionRequestTLVValue::SarMsgRefNum(value) => TLVValue::SarMsgRefNum(value),
            MessageSubmissionRequestTLVValue::SarSegmentSeqnum(value) => {
                TLVValue::SarSegmentSeqnum(value)
            }
            MessageSubmissionRequestTLVValue::SarTotalSegments(value) => {
                TLVValue::SarTotalSegments(value)
            }
            MessageSubmissionRequestTLVValue::SetDpf(value) => TLVValue::SetDpf(value),
            MessageSubmissionRequestTLVValue::SmsSignal(value) => TLVValue::SmsSignal(value),
            MessageSubmissionRequestTLVValue::SourceAddrSubunit(value) => {
                TLVValue::SourceAddrSubunit(value)
            }
            MessageSubmissionRequestTLVValue::SourceBearerType(value) => {
                TLVValue::SourceBearerType(value)
            }
            MessageSubmissionRequestTLVValue::SourceNetworkId(value) => {
                TLVValue::SourceNetworkId(value)
            }
            MessageSubmissionRequestTLVValue::SourceNetworkType(value) => {
                TLVValue::SourceNetworkType(value)
            }
            MessageSubmissionRequestTLVValue::SourceNodeId(value) => TLVValue::SourceNodeId(value),
            MessageSubmissionRequestTLVValue::SourcePort(value) => TLVValue::SourcePort(value),
            MessageSubmissionRequestTLVValue::SourceSubaddress(value) => {
                TLVValue::SourceSubaddress(value)
            }
            MessageSubmissionRequestTLVValue::SourceTelematicsId(value) => {
                TLVValue::SourceTelematicsId(value)
            }
            MessageSubmissionRequestTLVValue::UserMessageReference(value) => {
                TLVValue::UserMessageReference(value)
            }
            MessageSubmissionRequestTLVValue::UserResponseCode(value) => {
                TLVValue::UserResponseCode(value)
            }
            MessageSubmissionRequestTLVValue::UssdServiceOp(value) => {
                TLVValue::UssdServiceOp(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionRequestTLV {
    tlv: TLV,
}

impl MessageSubmissionRequestTLV {
    pub fn new(value: MessageSubmissionRequestTLVValue) -> Self {
        let value = TLVValue::from(value);
        let tlv = TLV::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: MessageSubmissionRequestTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        let tlv = TLV::from(tag);

        Self { tlv }
    }
}

impl From<MessageSubmissionRequestTLVTag> for TLV {
    fn from(tag: MessageSubmissionRequestTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        TLV::from(tag)
    }
}

impl From<MessageSubmissionRequestTLVValue> for MessageSubmissionRequestTLV {
    fn from(value: MessageSubmissionRequestTLVValue) -> Self {
        Self::new(value)
    }
}

impl From<MessageSubmissionRequestTLVValue> for TLV {
    fn from(value: MessageSubmissionRequestTLVValue) -> Self {
        let value = TLVValue::from(value);
        TLV::from(value)
    }
}

impl From<MessageSubmissionRequestTLV> for TLV {
    fn from(tlv: MessageSubmissionRequestTLV) -> Self {
        tlv.tlv
    }
}
