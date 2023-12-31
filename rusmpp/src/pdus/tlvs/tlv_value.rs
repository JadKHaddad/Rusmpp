use crate::{
    pdus::types::{command_status::CommandStatus, interface_version::InterfaceVersion},
    types::{
        c_octet_string::COctetString, no_fixed_size_octet_string::NoFixedSizeOctetString,
        octet_string::OctetString,
    },
};

use super::{
    tlv_tag::TLVTag,
    tlv_values::{
        addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMsgDelivery,
        bearer_type::BearerType, broadcast_area_identifier::BroadcastAreaIdentifier,
        broadcast_area_success::BroadcastAreaSuccess,
        broadcast_channel_indicator::BroadcastChannelIndicator,
        broadcast_content_type::BroadcastContentType,
        broadcast_frequency_interval::BroadcastFrequencyInterval,
        broadcast_message_class::BroadcastMessageClass, callback_num_pres_ind::CallbackNumPresInd,
        congestion_state::CongestionState, delivery_failure_reason::DeliveryFailureReason,
        dest_addr_np_resolution::DestAddrNpResolution, display_time::DisplayTime,
        dpf_result::DpfResult, its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
        language_indicator::LanguageIndicator, message_state::MessageState,
        more_messages_to_send::MoreMessagesToSend, ms_availability_status::MsAvailabilityStatus,
        ms_msg_wait_facilities::MsMsgWaitFacilities, ms_validity::MsValidity,
        network_error_code::NetworkErrorCode, network_type::NetworkType,
        number_of_messages::NumberOfMessages, payload_type::PayloadType,
        privacy_indicator::PrivacyIndicator, set_dpf::SetDpf, subaddress::Subaddress,
        ussd_service_op::UssdServiceOp,
    },
};

mod length;
mod read;
mod write;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TLVValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    AlertOnMessageDelivery(AlertOnMsgDelivery),
    BillingIdentification(OctetString<0, 1024>),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastContentTypeInfo(OctetString<0, 255>),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    BroadcastContentType(BroadcastContentType),
    /// Absolute time is formatted as a 16-character string (encoded as a 17-octet C-octet String)
    /// “YYMMDDhhmmsstnnp” where:
    ///
    /// Digits      Meaning
    /// ‘YY’        last two digits of the year (00-99)
    /// ‘MM’        month (01-12)
    /// ‘DD’        day (01-31)
    /// ‘hh’        hour (00-23)
    /// ‘mm’        minute (00-59)
    /// ‘ss’        second (00-59)
    /// ‘t’         tenths of second (0-9)
    /// ‘nn’        Time difference in quarter hours between local
    ///             time (as expressed in the first 13 octets) and
    ///             UTC (Universal Time Constant) time (00-48).
    /// ‘p’         “+” Local time is in quarter hours advanced in
    ///             relation to UTC time.
    ///             “-” Local time is in quarter hours retarded in
    ///             relation to UTC time.
    BroadcastEndTime(OctetString<0, 17>),
    BroadcastErrorStatus(CommandStatus),
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastMessageClass(BroadcastMessageClass),
    BroadcastRepNum(u16),
    BroadcastServiceGroup(OctetString<1, 255>),
    CallbackNum(OctetString<4, 19>),
    CallbackNumAtag(OctetString<0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    CongestionState(CongestionState),
    DeliveryFailureReason(DeliveryFailureReason),
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
    DpfResult(DpfResult),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(NoFixedSizeOctetString),
    MessageState(MessageState),
    MoreMessagesToSend(MoreMessagesToSend),
    MsAvailabilityStatus(MsAvailabilityStatus),
    MsMsgWaitFacilities(MsMsgWaitFacilities),
    MsValidity(MsValidity),
    NetworkErrorCode(NetworkErrorCode),
    NumberOfMessages(NumberOfMessages),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    QosTimeToLive(u32),
    ReceiptedMessageId(COctetString<1, 65>),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    ScInterfaceVersion(InterfaceVersion),
    SetDpf(SetDpf),
    /// Encoded as per [CMT-136]
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
    Other {
        tag: TLVTag,
        value: NoFixedSizeOctetString,
    },
}

impl TLVValue {
    pub fn tlv_tag(&self) -> TLVTag {
        match self {
            TLVValue::AdditionalStatusInfoText(_) => TLVTag::AdditionalStatusInfoText,
            TLVValue::AlertOnMessageDelivery(_) => TLVTag::AlertOnMessageDelivery,
            TLVValue::BillingIdentification(_) => TLVTag::BillingIdentification,
            TLVValue::BroadcastAreaIdentifier(_) => TLVTag::BroadcastAreaIdentifier,
            TLVValue::BroadcastAreaSuccess(_) => TLVTag::BroadcastAreaSuccess,
            TLVValue::BroadcastContentTypeInfo(_) => TLVTag::BroadcastContentTypeInfo,
            TLVValue::BroadcastChannelIndicator(_) => TLVTag::BroadcastChannelIndicator,
            TLVValue::BroadcastContentType(_) => TLVTag::BroadcastContentType,
            TLVValue::BroadcastEndTime(_) => TLVTag::BroadcastEndTime,
            TLVValue::BroadcastErrorStatus(_) => TLVTag::BroadcastErrorStatus,
            TLVValue::BroadcastFrequencyInterval(_) => TLVTag::BroadcastFrequencyInterval,
            TLVValue::BroadcastMessageClass(_) => TLVTag::BroadcastMessageClass,
            TLVValue::BroadcastRepNum(_) => TLVTag::BroadcastRepNum,
            TLVValue::BroadcastServiceGroup(_) => TLVTag::BroadcastServiceGroup,
            TLVValue::CallbackNum(_) => TLVTag::CallbackNum,
            TLVValue::CallbackNumAtag(_) => TLVTag::CallbackNumAtag,
            TLVValue::CallbackNumPresInd(_) => TLVTag::CallbackNumPresInd,
            TLVValue::CongestionState(_) => TLVTag::CongestionState,
            TLVValue::DeliveryFailureReason(_) => TLVTag::DeliveryFailureReason,
            TLVValue::DestAddrNpCountry(_) => TLVTag::DestAddrNpCountry,
            TLVValue::DestAddrNpInformation(_) => TLVTag::DestAddrNpInformation,
            TLVValue::DestAddrNpResolution(_) => TLVTag::DestAddrNpResolution,
            TLVValue::DestAddrSubunit(_) => TLVTag::DestAddrSubunit,
            TLVValue::DestBearerType(_) => TLVTag::DestBearerType,
            TLVValue::DestNetworkId(_) => TLVTag::DestNetworkId,
            TLVValue::DestNetworkType(_) => TLVTag::DestNetworkType,
            TLVValue::DestNodeId(_) => TLVTag::DestNodeId,
            TLVValue::DestSubaddress(_) => TLVTag::DestSubaddress,
            TLVValue::DestTelematicsId(_) => TLVTag::DestTelematicsId,
            TLVValue::DestPort(_) => TLVTag::DestPort,
            TLVValue::DisplayTime(_) => TLVTag::DisplayTime,
            TLVValue::DpfResult(_) => TLVTag::DpfResult,
            TLVValue::ItsReplyType(_) => TLVTag::ItsReplyType,
            TLVValue::ItsSessionInfo(_) => TLVTag::ItsSessionInfo,
            TLVValue::LanguageIndicator(_) => TLVTag::LanguageIndicator,
            TLVValue::MessagePayload(_) => TLVTag::MessagePayload,
            TLVValue::MessageState(_) => TLVTag::MessageState,
            TLVValue::MoreMessagesToSend(_) => TLVTag::MoreMessagesToSend,
            TLVValue::MsAvailabilityStatus(_) => TLVTag::MsAvailabilityStatus,
            TLVValue::MsMsgWaitFacilities(_) => TLVTag::MsMsgWaitFacilities,
            TLVValue::MsValidity(_) => TLVTag::MsValidity,
            TLVValue::NetworkErrorCode(_) => TLVTag::NetworkErrorCode,
            TLVValue::NumberOfMessages(_) => TLVTag::NumberOfMessages,
            TLVValue::PayloadType(_) => TLVTag::PayloadType,
            TLVValue::PrivacyIndicator(_) => TLVTag::PrivacyIndicator,
            TLVValue::QosTimeToLive(_) => TLVTag::QosTimeToLive,
            TLVValue::ReceiptedMessageId(_) => TLVTag::ReceiptedMessageId,
            TLVValue::SarMsgRefNum(_) => TLVTag::SarMsgRefNum,
            TLVValue::SarSegmentSeqnum(_) => TLVTag::SarSegmentSeqnum,
            TLVValue::SarTotalSegments(_) => TLVTag::SarTotalSegments,
            TLVValue::ScInterfaceVersion(_) => TLVTag::ScInterfaceVersion,
            TLVValue::SetDpf(_) => TLVTag::SetDpf,
            TLVValue::SmsSignal(_) => TLVTag::SmsSignal,
            TLVValue::SourceAddrSubunit(_) => TLVTag::SourceAddrSubunit,
            TLVValue::SourceBearerType(_) => TLVTag::SourceBearerType,
            TLVValue::SourceNetworkId(_) => TLVTag::SourceNetworkId,
            TLVValue::SourceNetworkType(_) => TLVTag::SourceNetworkType,
            TLVValue::SourceNodeId(_) => TLVTag::SourceNodeId,
            TLVValue::SourcePort(_) => TLVTag::SourcePort,
            TLVValue::SourceSubaddress(_) => TLVTag::SourceSubaddress,
            TLVValue::SourceTelematicsId(_) => TLVTag::SourceTelematicsId,
            TLVValue::UserMessageReference(_) => TLVTag::UserMessageReference,
            TLVValue::UserResponseCode(_) => TLVTag::UserResponseCode,
            TLVValue::UssdServiceOp(_) => TLVTag::UssdServiceOp,
            TLVValue::Other { tag, .. } => *tag,
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
    MessagePayload(NoFixedSizeOctetString),
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
    fn from(v: MessageSubmissionRequestTLVValue) -> Self {
        match v {
            MessageSubmissionRequestTLVValue::AlertOnMsgDelivery(v) => {
                TLVValue::AlertOnMessageDelivery(v)
            }
            MessageSubmissionRequestTLVValue::BillingIdentification(v) => {
                TLVValue::BillingIdentification(v)
            }
            MessageSubmissionRequestTLVValue::CallbackNum(v) => TLVValue::CallbackNum(v),
            MessageSubmissionRequestTLVValue::CallbackNumAtag(v) => TLVValue::CallbackNumAtag(v),
            MessageSubmissionRequestTLVValue::CallbackNumPresInd(v) => {
                TLVValue::CallbackNumPresInd(v)
            }
            MessageSubmissionRequestTLVValue::DestAddrNpCountry(v) => {
                TLVValue::DestAddrNpCountry(v)
            }
            MessageSubmissionRequestTLVValue::DestAddrNpInformation(v) => {
                TLVValue::DestAddrNpInformation(v)
            }
            MessageSubmissionRequestTLVValue::DestAddrNpResolution(v) => {
                TLVValue::DestAddrNpResolution(v)
            }
            MessageSubmissionRequestTLVValue::DestAddrSubunit(v) => TLVValue::DestAddrSubunit(v),
            MessageSubmissionRequestTLVValue::DestBearerType(v) => TLVValue::DestBearerType(v),
            MessageSubmissionRequestTLVValue::DestNetworkId(v) => TLVValue::DestNetworkId(v),
            MessageSubmissionRequestTLVValue::DestNetworkType(v) => TLVValue::DestNetworkType(v),
            MessageSubmissionRequestTLVValue::DestNodeId(v) => TLVValue::DestNodeId(v),
            MessageSubmissionRequestTLVValue::DestSubaddress(v) => TLVValue::DestSubaddress(v),
            MessageSubmissionRequestTLVValue::DestTelematicsId(v) => TLVValue::DestTelematicsId(v),
            MessageSubmissionRequestTLVValue::DestPort(v) => TLVValue::DestPort(v),
            MessageSubmissionRequestTLVValue::DisplayTime(v) => TLVValue::DisplayTime(v),
            MessageSubmissionRequestTLVValue::ItsReplyType(v) => TLVValue::ItsReplyType(v),
            MessageSubmissionRequestTLVValue::ItsSessionInfo(v) => TLVValue::ItsSessionInfo(v),
            MessageSubmissionRequestTLVValue::LanguageIndicator(v) => {
                TLVValue::LanguageIndicator(v)
            }
            MessageSubmissionRequestTLVValue::MessagePayload(v) => TLVValue::MessagePayload(v),
            MessageSubmissionRequestTLVValue::MoreMessagesToSend(v) => {
                TLVValue::MoreMessagesToSend(v)
            }
            MessageSubmissionRequestTLVValue::MsMsgWaitFacilities(v) => {
                TLVValue::MsMsgWaitFacilities(v)
            }
            MessageSubmissionRequestTLVValue::MsValidity(v) => TLVValue::MsValidity(v),
            MessageSubmissionRequestTLVValue::NumberOfMessages(v) => TLVValue::NumberOfMessages(v),
            MessageSubmissionRequestTLVValue::PayloadType(v) => TLVValue::PayloadType(v),
            MessageSubmissionRequestTLVValue::PrivacyIndicator(v) => TLVValue::PrivacyIndicator(v),
            MessageSubmissionRequestTLVValue::QosTimeToLive(v) => TLVValue::QosTimeToLive(v),
            MessageSubmissionRequestTLVValue::SarMsgRefNum(v) => TLVValue::SarMsgRefNum(v),
            MessageSubmissionRequestTLVValue::SarSegmentSeqnum(v) => TLVValue::SarSegmentSeqnum(v),
            MessageSubmissionRequestTLVValue::SarTotalSegments(v) => TLVValue::SarTotalSegments(v),
            MessageSubmissionRequestTLVValue::SetDpf(v) => TLVValue::SetDpf(v),
            MessageSubmissionRequestTLVValue::SmsSignal(v) => TLVValue::SmsSignal(v),
            MessageSubmissionRequestTLVValue::SourceAddrSubunit(v) => {
                TLVValue::SourceAddrSubunit(v)
            }
            MessageSubmissionRequestTLVValue::SourceBearerType(v) => TLVValue::SourceBearerType(v),
            MessageSubmissionRequestTLVValue::SourceNetworkId(v) => TLVValue::SourceNetworkId(v),
            MessageSubmissionRequestTLVValue::SourceNetworkType(v) => {
                TLVValue::SourceNetworkType(v)
            }
            MessageSubmissionRequestTLVValue::SourceNodeId(v) => TLVValue::SourceNodeId(v),
            MessageSubmissionRequestTLVValue::SourcePort(v) => TLVValue::SourcePort(v),
            MessageSubmissionRequestTLVValue::SourceSubaddress(v) => TLVValue::SourceSubaddress(v),
            MessageSubmissionRequestTLVValue::SourceTelematicsId(v) => {
                TLVValue::SourceTelematicsId(v)
            }
            MessageSubmissionRequestTLVValue::UserMessageReference(v) => {
                TLVValue::UserMessageReference(v)
            }
            MessageSubmissionRequestTLVValue::UserResponseCode(v) => TLVValue::UserResponseCode(v),
            MessageSubmissionRequestTLVValue::UssdServiceOp(v) => TLVValue::UssdServiceOp(v),
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
    fn from(v: MessageSubmissionResponseTLVValue) -> Self {
        match v {
            MessageSubmissionResponseTLVValue::AdditionalStatusInfoText(v) => {
                TLVValue::AdditionalStatusInfoText(v)
            }
            MessageSubmissionResponseTLVValue::DeliveryFailureReason(v) => {
                TLVValue::DeliveryFailureReason(v)
            }
            MessageSubmissionResponseTLVValue::DpfResult(v) => TLVValue::DpfResult(v),
            MessageSubmissionResponseTLVValue::NetworkErrorCode(v) => TLVValue::NetworkErrorCode(v),
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
    fn from(v: MessageDeliveryRequestTLVValue) -> Self {
        match v {
            MessageDeliveryRequestTLVValue::CallbackNum(v) => TLVValue::CallbackNum(v),
            MessageDeliveryRequestTLVValue::CallbackNumAtag(v) => TLVValue::CallbackNumAtag(v),
            MessageDeliveryRequestTLVValue::CallbackNumPresInd(v) => {
                TLVValue::CallbackNumPresInd(v)
            }
            MessageDeliveryRequestTLVValue::DestAddrNpCountry(v) => TLVValue::DestAddrNpCountry(v),
            MessageDeliveryRequestTLVValue::DestAddrNpInformation(v) => {
                TLVValue::DestAddrNpInformation(v)
            }
            MessageDeliveryRequestTLVValue::DestAddrNpResolution(v) => {
                TLVValue::DestAddrNpResolution(v)
            }
            MessageDeliveryRequestTLVValue::DestAddrSubunit(v) => TLVValue::DestAddrSubunit(v),
            MessageDeliveryRequestTLVValue::DestNetworkId(v) => TLVValue::DestNetworkId(v),
            MessageDeliveryRequestTLVValue::DestNodeId(v) => TLVValue::DestNodeId(v),
            MessageDeliveryRequestTLVValue::DestSubaddress(v) => TLVValue::DestSubaddress(v),
            MessageDeliveryRequestTLVValue::DestPort(v) => TLVValue::DestPort(v),
            MessageDeliveryRequestTLVValue::DpfResult(v) => TLVValue::DpfResult(v),
            MessageDeliveryRequestTLVValue::ItsReplyType(v) => TLVValue::ItsReplyType(v),
            MessageDeliveryRequestTLVValue::ItsSessionInfo(v) => TLVValue::ItsSessionInfo(v),
            MessageDeliveryRequestTLVValue::LanguageIndicator(v) => TLVValue::LanguageIndicator(v),
            MessageDeliveryRequestTLVValue::MessagePayload(v) => TLVValue::MessagePayload(v),
            MessageDeliveryRequestTLVValue::MessageState(v) => TLVValue::MessageState(v),
            MessageDeliveryRequestTLVValue::NetworkErrorCode(v) => TLVValue::NetworkErrorCode(v),
            MessageDeliveryRequestTLVValue::PayloadType(v) => TLVValue::PayloadType(v),
            MessageDeliveryRequestTLVValue::PrivacyIndicator(v) => TLVValue::PrivacyIndicator(v),
            MessageDeliveryRequestTLVValue::ReceiptedMessageId(v) => {
                TLVValue::ReceiptedMessageId(v)
            }
            MessageDeliveryRequestTLVValue::SarMsgRefNum(v) => TLVValue::SarMsgRefNum(v),
            MessageDeliveryRequestTLVValue::SarSegmentSeqnum(v) => TLVValue::SarSegmentSeqnum(v),
            MessageDeliveryRequestTLVValue::SarTotalSegments(v) => TLVValue::SarTotalSegments(v),
            MessageDeliveryRequestTLVValue::SourceAddrSubunit(v) => TLVValue::SourceAddrSubunit(v),
            MessageDeliveryRequestTLVValue::SourceNetworkId(v) => TLVValue::SourceNetworkId(v),
            MessageDeliveryRequestTLVValue::SourceNodeId(v) => TLVValue::SourceNodeId(v),
            MessageDeliveryRequestTLVValue::SourcePort(v) => TLVValue::SourcePort(v),
            MessageDeliveryRequestTLVValue::SourceSubaddress(v) => TLVValue::SourceSubaddress(v),
            MessageDeliveryRequestTLVValue::UserMessageReference(v) => {
                TLVValue::UserMessageReference(v)
            }
            MessageDeliveryRequestTLVValue::UserResponseCode(v) => TLVValue::UserResponseCode(v),
            MessageDeliveryRequestTLVValue::UssdServiceOp(v) => TLVValue::UssdServiceOp(v),
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
    fn from(v: MessageDeliveryResponseTLVValue) -> Self {
        match v {
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
pub enum BroadcastRequestTLVValue {
    AlertOnMsgDelivery(AlertOnMsgDelivery),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    BroadcastContentTypeInfo(OctetString<0, 255>),
    BroadcastMessageClass(BroadcastMessageClass),
    BroadcastServiceGroup(OctetString<1, 255>),
    CallbackNum(OctetString<4, 19>),
    CallbackNumAtag(OctetString<0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrSubunit(AddrSubunit),
    DestSubaddress(Subaddress),
    DestPort(u16),
    DisplayTime(DisplayTime),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(NoFixedSizeOctetString),
    MsValidity(MsValidity),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    UserMessageReference(u16),
}

impl From<BroadcastRequestTLVValue> for TLVValue {
    fn from(v: BroadcastRequestTLVValue) -> Self {
        match v {
            BroadcastRequestTLVValue::AlertOnMsgDelivery(v) => TLVValue::AlertOnMessageDelivery(v),
            BroadcastRequestTLVValue::BroadcastChannelIndicator(v) => {
                TLVValue::BroadcastChannelIndicator(v)
            }
            BroadcastRequestTLVValue::BroadcastContentTypeInfo(v) => {
                TLVValue::BroadcastContentTypeInfo(v)
            }
            BroadcastRequestTLVValue::BroadcastMessageClass(v) => {
                TLVValue::BroadcastMessageClass(v)
            }
            BroadcastRequestTLVValue::BroadcastServiceGroup(v) => {
                TLVValue::BroadcastServiceGroup(v)
            }
            BroadcastRequestTLVValue::CallbackNum(v) => TLVValue::CallbackNum(v),
            BroadcastRequestTLVValue::CallbackNumAtag(v) => TLVValue::CallbackNumAtag(v),
            BroadcastRequestTLVValue::CallbackNumPresInd(v) => TLVValue::CallbackNumPresInd(v),
            BroadcastRequestTLVValue::DestAddrSubunit(v) => TLVValue::DestAddrSubunit(v),
            BroadcastRequestTLVValue::DestSubaddress(v) => TLVValue::DestSubaddress(v),
            BroadcastRequestTLVValue::DestPort(v) => TLVValue::DestPort(v),
            BroadcastRequestTLVValue::DisplayTime(v) => TLVValue::DisplayTime(v),
            BroadcastRequestTLVValue::LanguageIndicator(v) => TLVValue::LanguageIndicator(v),
            BroadcastRequestTLVValue::MessagePayload(v) => TLVValue::MessagePayload(v),
            BroadcastRequestTLVValue::MsValidity(v) => TLVValue::MsValidity(v),
            BroadcastRequestTLVValue::PayloadType(v) => TLVValue::PayloadType(v),
            BroadcastRequestTLVValue::PrivacyIndicator(v) => TLVValue::PrivacyIndicator(v),
            BroadcastRequestTLVValue::SmsSignal(v) => TLVValue::SmsSignal(v),
            BroadcastRequestTLVValue::SourceAddrSubunit(v) => TLVValue::SourceAddrSubunit(v),
            BroadcastRequestTLVValue::SourcePort(v) => TLVValue::SourcePort(v),
            BroadcastRequestTLVValue::SourceSubaddress(v) => TLVValue::SourceSubaddress(v),
            BroadcastRequestTLVValue::UserMessageReference(v) => TLVValue::UserMessageReference(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastResponseTLVValue {
    BroadcastErrorStatus(CommandStatus),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
}

impl From<BroadcastResponseTLVValue> for TLVValue {
    fn from(v: BroadcastResponseTLVValue) -> Self {
        match v {
            BroadcastResponseTLVValue::BroadcastErrorStatus(v) => TLVValue::BroadcastErrorStatus(v),
            BroadcastResponseTLVValue::BroadcastAreaIdentifier(v) => {
                TLVValue::BroadcastAreaIdentifier(v)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueryBroadcastResponseTLVValue {
    BroadcastEndTime(OctetString<0, 17>),
    UserMessageReference(u16),
}

impl From<QueryBroadcastResponseTLVValue> for TLVValue {
    fn from(v: QueryBroadcastResponseTLVValue) -> Self {
        match v {
            QueryBroadcastResponseTLVValue::BroadcastEndTime(v) => TLVValue::BroadcastEndTime(v),
            QueryBroadcastResponseTLVValue::UserMessageReference(v) => {
                TLVValue::UserMessageReference(v)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancelBroadcastTLVValue {
    BroadcastContentType(BroadcastContentType),
    UserMessageReference(u16),
}

impl From<CancelBroadcastTLVValue> for TLVValue {
    fn from(v: CancelBroadcastTLVValue) -> Self {
        match v {
            CancelBroadcastTLVValue::BroadcastContentType(v) => TLVValue::BroadcastContentType(v),
            CancelBroadcastTLVValue::UserMessageReference(v) => TLVValue::UserMessageReference(v),
        }
    }
}
