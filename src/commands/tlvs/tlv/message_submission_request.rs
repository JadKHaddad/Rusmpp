use super::Tlv;
use crate::{
    commands::{
        tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
        types::{
            addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMsgDelivery,
            bearer_type::BearerType, callback_num_pres_ind::CallbackNumPresInd,
            dest_addr_np_resolution::DestAddrNpResolution, display_time::DisplayTime,
            its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
            language_indicator::LanguageIndicator, more_messages_to_send::MoreMessagesToSend,
            ms_msg_wait_facilities::MsMsgWaitFacilities, ms_validity::MsValidity,
            network_type::NetworkType, number_of_messages::NumberOfMessages,
            payload_type::PayloadType, privacy_indicator::PrivacyIndicator, set_dpf::SetDpf,
            sub_address::Subaddress, ussd_service_op::UssdServiceOp, MessagePayload,
            UserMessageReference,
        },
    },
    types::{COctetString, OctetString},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionRequestTlvTag {
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

impl From<MessageSubmissionRequestTlvTag> for TlvTag {
    fn from(value: MessageSubmissionRequestTlvTag) -> Self {
        match value {
            MessageSubmissionRequestTlvTag::AlertOnMsgDelivery => TlvTag::AlertOnMessageDelivery,
            MessageSubmissionRequestTlvTag::BillingIdentification => TlvTag::BillingIdentification,
            MessageSubmissionRequestTlvTag::CallbackNum => TlvTag::CallbackNum,
            MessageSubmissionRequestTlvTag::CallbackNumAtag => TlvTag::CallbackNumAtag,
            MessageSubmissionRequestTlvTag::CallbackNumPresInd => TlvTag::CallbackNumPresInd,
            MessageSubmissionRequestTlvTag::DestAddrNpCountry => TlvTag::DestAddrNpCountry,
            MessageSubmissionRequestTlvTag::DestAddrNpInformation => TlvTag::DestAddrNpInformation,
            MessageSubmissionRequestTlvTag::DestAddrNpResolution => TlvTag::DestAddrNpResolution,
            MessageSubmissionRequestTlvTag::DestAddrSubunit => TlvTag::DestAddrSubunit,
            MessageSubmissionRequestTlvTag::DestBearerType => TlvTag::DestBearerType,
            MessageSubmissionRequestTlvTag::DestNetworkId => TlvTag::DestNetworkId,
            MessageSubmissionRequestTlvTag::DestNetworkType => TlvTag::DestNetworkType,
            MessageSubmissionRequestTlvTag::DestNodeId => TlvTag::DestNodeId,
            MessageSubmissionRequestTlvTag::DestSubaddress => TlvTag::DestSubaddress,
            MessageSubmissionRequestTlvTag::DestTelematicsId => TlvTag::DestTelematicsId,
            MessageSubmissionRequestTlvTag::DestPort => TlvTag::DestPort,
            MessageSubmissionRequestTlvTag::DisplayTime => TlvTag::DisplayTime,
            MessageSubmissionRequestTlvTag::ItsReplyType => TlvTag::ItsReplyType,
            MessageSubmissionRequestTlvTag::ItsSessionInfo => TlvTag::ItsSessionInfo,
            MessageSubmissionRequestTlvTag::LanguageIndicator => TlvTag::LanguageIndicator,
            MessageSubmissionRequestTlvTag::MessagePayload => TlvTag::MessagePayload,
            MessageSubmissionRequestTlvTag::MoreMessagesToSend => TlvTag::MoreMessagesToSend,
            MessageSubmissionRequestTlvTag::MsMsgWaitFacilities => TlvTag::MsMsgWaitFacilities,
            MessageSubmissionRequestTlvTag::MsValidity => TlvTag::MsValidity,
            MessageSubmissionRequestTlvTag::NumberOfMessages => TlvTag::NumberOfMessages,
            MessageSubmissionRequestTlvTag::PayloadType => TlvTag::PayloadType,
            MessageSubmissionRequestTlvTag::PrivacyIndicator => TlvTag::PrivacyIndicator,
            MessageSubmissionRequestTlvTag::QosTimeToLive => TlvTag::QosTimeToLive,
            MessageSubmissionRequestTlvTag::SarMsgRefNum => TlvTag::SarMsgRefNum,
            MessageSubmissionRequestTlvTag::SarSegmentSeqnum => TlvTag::SarSegmentSeqnum,
            MessageSubmissionRequestTlvTag::SarTotalSegments => TlvTag::SarTotalSegments,
            MessageSubmissionRequestTlvTag::SetDpf => TlvTag::SetDpf,
            MessageSubmissionRequestTlvTag::SmsSignal => TlvTag::SmsSignal,
            MessageSubmissionRequestTlvTag::SourceAddrSubunit => TlvTag::SourceAddrSubunit,
            MessageSubmissionRequestTlvTag::SourceBearerType => TlvTag::SourceBearerType,
            MessageSubmissionRequestTlvTag::SourceNetworkId => TlvTag::SourceNetworkId,
            MessageSubmissionRequestTlvTag::SourceNetworkType => TlvTag::SourceNetworkType,
            MessageSubmissionRequestTlvTag::SourceNodeId => TlvTag::SourceNodeId,
            MessageSubmissionRequestTlvTag::SourcePort => TlvTag::SourcePort,
            MessageSubmissionRequestTlvTag::SourceSubaddress => TlvTag::SourceSubaddress,
            MessageSubmissionRequestTlvTag::SourceTelematicsId => TlvTag::SourceTelematicsId,
            MessageSubmissionRequestTlvTag::UserMessageReference => TlvTag::UserMessageReference,
            MessageSubmissionRequestTlvTag::UserResponseCode => TlvTag::UserResponseCode,
            MessageSubmissionRequestTlvTag::UssdServiceOp => TlvTag::UssdServiceOp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionRequestTlvValue {
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
    MessagePayload(MessagePayload),
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
    UserMessageReference(UserMessageReference),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
}

impl From<MessageSubmissionRequestTlvValue> for TlvValue {
    fn from(value: MessageSubmissionRequestTlvValue) -> Self {
        match value {
            MessageSubmissionRequestTlvValue::AlertOnMsgDelivery(value) => {
                TlvValue::AlertOnMessageDelivery(value)
            }
            MessageSubmissionRequestTlvValue::BillingIdentification(value) => {
                TlvValue::BillingIdentification(value)
            }
            MessageSubmissionRequestTlvValue::CallbackNum(value) => TlvValue::CallbackNum(value),
            MessageSubmissionRequestTlvValue::CallbackNumAtag(value) => {
                TlvValue::CallbackNumAtag(value)
            }
            MessageSubmissionRequestTlvValue::CallbackNumPresInd(value) => {
                TlvValue::CallbackNumPresInd(value)
            }
            MessageSubmissionRequestTlvValue::DestAddrNpCountry(value) => {
                TlvValue::DestAddrNpCountry(value)
            }
            MessageSubmissionRequestTlvValue::DestAddrNpInformation(value) => {
                TlvValue::DestAddrNpInformation(value)
            }
            MessageSubmissionRequestTlvValue::DestAddrNpResolution(value) => {
                TlvValue::DestAddrNpResolution(value)
            }
            MessageSubmissionRequestTlvValue::DestAddrSubunit(value) => {
                TlvValue::DestAddrSubunit(value)
            }
            MessageSubmissionRequestTlvValue::DestBearerType(value) => {
                TlvValue::DestBearerType(value)
            }
            MessageSubmissionRequestTlvValue::DestNetworkId(value) => {
                TlvValue::DestNetworkId(value)
            }
            MessageSubmissionRequestTlvValue::DestNetworkType(value) => {
                TlvValue::DestNetworkType(value)
            }
            MessageSubmissionRequestTlvValue::DestNodeId(value) => TlvValue::DestNodeId(value),
            MessageSubmissionRequestTlvValue::DestSubaddress(value) => {
                TlvValue::DestSubaddress(value)
            }
            MessageSubmissionRequestTlvValue::DestTelematicsId(value) => {
                TlvValue::DestTelematicsId(value)
            }
            MessageSubmissionRequestTlvValue::DestPort(value) => TlvValue::DestPort(value),
            MessageSubmissionRequestTlvValue::DisplayTime(value) => TlvValue::DisplayTime(value),
            MessageSubmissionRequestTlvValue::ItsReplyType(value) => TlvValue::ItsReplyType(value),
            MessageSubmissionRequestTlvValue::ItsSessionInfo(value) => {
                TlvValue::ItsSessionInfo(value)
            }
            MessageSubmissionRequestTlvValue::LanguageIndicator(value) => {
                TlvValue::LanguageIndicator(value)
            }
            MessageSubmissionRequestTlvValue::MessagePayload(value) => {
                TlvValue::MessagePayload(value)
            }
            MessageSubmissionRequestTlvValue::MoreMessagesToSend(value) => {
                TlvValue::MoreMessagesToSend(value)
            }
            MessageSubmissionRequestTlvValue::MsMsgWaitFacilities(value) => {
                TlvValue::MsMsgWaitFacilities(value)
            }
            MessageSubmissionRequestTlvValue::MsValidity(value) => TlvValue::MsValidity(value),
            MessageSubmissionRequestTlvValue::NumberOfMessages(value) => {
                TlvValue::NumberOfMessages(value)
            }
            MessageSubmissionRequestTlvValue::PayloadType(value) => TlvValue::PayloadType(value),
            MessageSubmissionRequestTlvValue::PrivacyIndicator(value) => {
                TlvValue::PrivacyIndicator(value)
            }
            MessageSubmissionRequestTlvValue::QosTimeToLive(value) => {
                TlvValue::QosTimeToLive(value)
            }
            MessageSubmissionRequestTlvValue::SarMsgRefNum(value) => TlvValue::SarMsgRefNum(value),
            MessageSubmissionRequestTlvValue::SarSegmentSeqnum(value) => {
                TlvValue::SarSegmentSeqnum(value)
            }
            MessageSubmissionRequestTlvValue::SarTotalSegments(value) => {
                TlvValue::SarTotalSegments(value)
            }
            MessageSubmissionRequestTlvValue::SetDpf(value) => TlvValue::SetDpf(value),
            MessageSubmissionRequestTlvValue::SmsSignal(value) => TlvValue::SmsSignal(value),
            MessageSubmissionRequestTlvValue::SourceAddrSubunit(value) => {
                TlvValue::SourceAddrSubunit(value)
            }
            MessageSubmissionRequestTlvValue::SourceBearerType(value) => {
                TlvValue::SourceBearerType(value)
            }
            MessageSubmissionRequestTlvValue::SourceNetworkId(value) => {
                TlvValue::SourceNetworkId(value)
            }
            MessageSubmissionRequestTlvValue::SourceNetworkType(value) => {
                TlvValue::SourceNetworkType(value)
            }
            MessageSubmissionRequestTlvValue::SourceNodeId(value) => TlvValue::SourceNodeId(value),
            MessageSubmissionRequestTlvValue::SourcePort(value) => TlvValue::SourcePort(value),
            MessageSubmissionRequestTlvValue::SourceSubaddress(value) => {
                TlvValue::SourceSubaddress(value)
            }
            MessageSubmissionRequestTlvValue::SourceTelematicsId(value) => {
                TlvValue::SourceTelematicsId(value)
            }
            MessageSubmissionRequestTlvValue::UserMessageReference(value) => {
                TlvValue::UserMessageReference(value)
            }
            MessageSubmissionRequestTlvValue::UserResponseCode(value) => {
                TlvValue::UserResponseCode(value)
            }
            MessageSubmissionRequestTlvValue::UssdServiceOp(value) => {
                TlvValue::UssdServiceOp(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionRequestTlv {
    tlv: Tlv,
}

impl MessageSubmissionRequestTlv {
    pub fn new(value: MessageSubmissionRequestTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: MessageSubmissionRequestTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<MessageSubmissionRequestTlvTag> for Tlv {
    fn from(tag: MessageSubmissionRequestTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<MessageSubmissionRequestTlvValue> for MessageSubmissionRequestTlv {
    fn from(value: MessageSubmissionRequestTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<MessageSubmissionRequestTlvValue> for Tlv {
    fn from(value: MessageSubmissionRequestTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<MessageSubmissionRequestTlv> for Tlv {
    fn from(tlv: MessageSubmissionRequestTlv) -> Self {
        tlv.tlv
    }
}
