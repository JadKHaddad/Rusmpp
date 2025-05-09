use super::tlv_tag::TLVTag;
use crate::{
    commands::types::{
        addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMsgDelivery,
        bearer_type::BearerType, broadcast_area_identifier::BroadcastAreaIdentifier,
        broadcast_area_success::BroadcastAreaSuccess,
        broadcast_channel_indicator::BroadcastChannelIndicator,
        broadcast_content_type::BroadcastContentType,
        broadcast_frequency_interval::BroadcastFrequencyInterval,
        broadcast_message_class::BroadcastMessageClass, callback_num_pres_ind::CallbackNumPresInd,
        command_status::CommandStatus, congestion_state::CongestionState,
        delivery_failure_reason::DeliveryFailureReason,
        dest_addr_np_resolution::DestAddrNpResolution, display_time::DisplayTime,
        dpf_result::DpfResult, interface_version::InterfaceVersion, its_reply_type::ItsReplyType,
        its_session_info::ItsSessionInfo, language_indicator::LanguageIndicator,
        message_state::MessageState, more_messages_to_send::MoreMessagesToSend,
        ms_availability_status::MsAvailabilityStatus, ms_msg_wait_facilities::MsMsgWaitFacilities,
        ms_validity::MsValidity, network_error_code::NetworkErrorCode, network_type::NetworkType,
        number_of_messages::NumberOfMessages, payload_type::PayloadType,
        privacy_indicator::PrivacyIndicator, set_dpf::SetDpf, sub_address::Subaddress,
        ussd_service_op::UssdServiceOp,
    },
    errors::DecodeError,
    types::{
        any_octet_string::AnyOctetString, c_octet_string::COctetString, octet_string::OctetString,
    },
    Decode, DecodeResultExt, DecodeWithKey, DecodeWithLength, Encode, Length,
};

/// See module level documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TLVValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    AlertOnMessageDelivery(AlertOnMsgDelivery),
    BillingIdentification(OctetString<0, 1024>),
    /// Identifies one or more target Broadcast Area(s) for which the
    /// status information applies.
    ///
    /// The number of instances of this parameter will be exactly equal
    /// to the number of occurrences of the broadcast_area_identifiers
    /// parameter in the corresponding broadcast_sm.
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    /// The success rate indicator, defined as the ratio of the
    /// number of BTSs that accepted the message and the total
    /// number of BTSs that should have accepted the message, for
    /// a particular broadcast_area_identifier.
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastContentTypeInfo(OctetString<0, 255>),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    /// Specifies the content type of the message.
    BroadcastContentType(BroadcastContentType),
    /// Absolute time is formatted as a 16-character string (encoded as a 17-octet C-octet String)
    /// “YYMMDDhhmmsstnnp” where:
    ///
    /// | Digits | Meaning |
    /// |--------|---------|
    /// | ‘YY’   | last two digits of the year (00-99)   |
    /// | ‘MM’   | month (01-12)                         |
    /// | ‘DD’   | day (01-31)                           |
    /// | ‘hh’   | hour (00-23)                          |
    /// | ‘mm’   | minute (00-59)                        |
    /// | ‘ss’   | second (00-59)                        |
    /// | ‘t’    | tenths of second (0-9)                |
    /// | ‘nn’   | time difference in quarter hours between local time (as expressed in the first 13 octets) and UTC (Universal Time Constant) time (00-48). |
    /// | ‘p’    | “+” Local time is in quarter hours advanced in relation to UTC time. “-” Local time is in quarter hours retarded in relation to UTC time. |
    BroadcastEndTime(OctetString<0, 17>),
    BroadcastErrorStatus(CommandStatus),
    /// This field indicates the frequency interval at which
    /// the broadcasts of a message should be repeated.
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastMessageClass(BroadcastMessageClass),
    /// This field indicates the number of repeated
    /// broadcasts of a message requested by the submitter.
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
    MessagePayload(AnyOctetString),
    /// This field indicates the current status of the broadcast message.
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
        value: AnyOctetString,
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

impl Length for TLVValue {
    fn length(&self) -> usize {
        match self {
            TLVValue::AdditionalStatusInfoText(value) => value.length(),
            TLVValue::AlertOnMessageDelivery(value) => value.length(),
            TLVValue::BillingIdentification(value) => value.length(),
            TLVValue::BroadcastAreaIdentifier(value) => value.length(),
            TLVValue::BroadcastAreaSuccess(value) => value.length(),
            TLVValue::BroadcastContentTypeInfo(value) => value.length(),
            TLVValue::BroadcastChannelIndicator(value) => value.length(),
            TLVValue::BroadcastContentType(value) => value.length(),
            TLVValue::BroadcastEndTime(value) => value.length(),
            TLVValue::BroadcastErrorStatus(value) => value.length(),
            TLVValue::BroadcastFrequencyInterval(value) => value.length(),
            TLVValue::BroadcastMessageClass(value) => value.length(),
            TLVValue::BroadcastRepNum(value) => value.length(),
            TLVValue::BroadcastServiceGroup(value) => value.length(),
            TLVValue::CallbackNum(value) => value.length(),
            TLVValue::CallbackNumAtag(value) => value.length(),
            TLVValue::CallbackNumPresInd(value) => value.length(),
            TLVValue::CongestionState(value) => value.length(),
            TLVValue::DeliveryFailureReason(value) => value.length(),
            TLVValue::DestAddrNpCountry(value) => value.length(),
            TLVValue::DestAddrNpInformation(value) => value.length(),
            TLVValue::DestAddrNpResolution(value) => value.length(),
            TLVValue::DestAddrSubunit(value) => value.length(),
            TLVValue::DestBearerType(value) => value.length(),
            TLVValue::DestNetworkId(value) => value.length(),
            TLVValue::DestNetworkType(value) => value.length(),
            TLVValue::DestNodeId(value) => value.length(),
            TLVValue::DestSubaddress(value) => value.length(),
            TLVValue::DestTelematicsId(value) => value.length(),
            TLVValue::DestPort(value) => value.length(),
            TLVValue::DisplayTime(value) => value.length(),
            TLVValue::DpfResult(value) => value.length(),
            TLVValue::ItsReplyType(value) => value.length(),
            TLVValue::ItsSessionInfo(value) => value.length(),
            TLVValue::LanguageIndicator(value) => value.length(),
            TLVValue::MessagePayload(value) => value.length(),
            TLVValue::MessageState(value) => value.length(),
            TLVValue::MoreMessagesToSend(value) => value.length(),
            TLVValue::MsAvailabilityStatus(value) => value.length(),
            TLVValue::MsMsgWaitFacilities(value) => value.length(),
            TLVValue::MsValidity(value) => value.length(),
            TLVValue::NetworkErrorCode(value) => value.length(),
            TLVValue::NumberOfMessages(value) => value.length(),
            TLVValue::PayloadType(value) => value.length(),
            TLVValue::PrivacyIndicator(value) => value.length(),
            TLVValue::QosTimeToLive(value) => value.length(),
            TLVValue::ReceiptedMessageId(value) => value.length(),
            TLVValue::SarMsgRefNum(value) => value.length(),
            TLVValue::SarSegmentSeqnum(value) => value.length(),
            TLVValue::SarTotalSegments(value) => value.length(),
            TLVValue::ScInterfaceVersion(value) => value.length(),
            TLVValue::SetDpf(value) => value.length(),
            TLVValue::SmsSignal(value) => value.length(),
            TLVValue::SourceAddrSubunit(value) => value.length(),
            TLVValue::SourceBearerType(value) => value.length(),
            TLVValue::SourceNetworkId(value) => value.length(),
            TLVValue::SourceNetworkType(value) => value.length(),
            TLVValue::SourceNodeId(value) => value.length(),
            TLVValue::SourcePort(value) => value.length(),
            TLVValue::SourceSubaddress(value) => value.length(),
            TLVValue::SourceTelematicsId(value) => value.length(),
            TLVValue::UserMessageReference(value) => value.length(),
            TLVValue::UserResponseCode(value) => value.length(),
            TLVValue::UssdServiceOp(value) => value.length(),
            TLVValue::Other { value, .. } => value.length(),
        }
    }
}

impl Encode for TLVValue {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            TLVValue::AdditionalStatusInfoText(value) => value.encode(dst),
            TLVValue::AlertOnMessageDelivery(value) => value.encode(dst),
            TLVValue::BillingIdentification(value) => value.encode(dst),
            TLVValue::BroadcastAreaIdentifier(value) => value.encode(dst),
            TLVValue::BroadcastAreaSuccess(value) => value.encode(dst),
            TLVValue::BroadcastContentTypeInfo(value) => value.encode(dst),
            TLVValue::BroadcastChannelIndicator(value) => value.encode(dst),
            TLVValue::BroadcastContentType(value) => value.encode(dst),
            TLVValue::BroadcastEndTime(value) => value.encode(dst),
            TLVValue::BroadcastErrorStatus(value) => value.encode(dst),
            TLVValue::BroadcastFrequencyInterval(value) => value.encode(dst),
            TLVValue::BroadcastMessageClass(value) => value.encode(dst),
            TLVValue::BroadcastRepNum(value) => value.encode(dst),
            TLVValue::BroadcastServiceGroup(value) => value.encode(dst),
            TLVValue::CallbackNum(value) => value.encode(dst),
            TLVValue::CallbackNumAtag(value) => value.encode(dst),
            TLVValue::CallbackNumPresInd(value) => value.encode(dst),
            TLVValue::CongestionState(value) => value.encode(dst),
            TLVValue::DeliveryFailureReason(value) => value.encode(dst),
            TLVValue::DestAddrNpCountry(value) => value.encode(dst),
            TLVValue::DestAddrNpInformation(value) => value.encode(dst),
            TLVValue::DestAddrNpResolution(value) => value.encode(dst),
            TLVValue::DestAddrSubunit(value) => value.encode(dst),
            TLVValue::DestBearerType(value) => value.encode(dst),
            TLVValue::DestNetworkId(value) => value.encode(dst),
            TLVValue::DestNetworkType(value) => value.encode(dst),
            TLVValue::DestNodeId(value) => value.encode(dst),
            TLVValue::DestSubaddress(value) => value.encode(dst),
            TLVValue::DestTelematicsId(value) => value.encode(dst),
            TLVValue::DestPort(value) => value.encode(dst),
            TLVValue::DisplayTime(value) => value.encode(dst),
            TLVValue::DpfResult(value) => value.encode(dst),
            TLVValue::ItsReplyType(value) => value.encode(dst),
            TLVValue::ItsSessionInfo(value) => value.encode(dst),
            TLVValue::LanguageIndicator(value) => value.encode(dst),
            TLVValue::MessagePayload(value) => value.encode(dst),
            TLVValue::MessageState(value) => value.encode(dst),
            TLVValue::MoreMessagesToSend(value) => value.encode(dst),
            TLVValue::MsAvailabilityStatus(value) => value.encode(dst),
            TLVValue::MsMsgWaitFacilities(value) => value.encode(dst),
            TLVValue::MsValidity(value) => value.encode(dst),
            TLVValue::NetworkErrorCode(value) => value.encode(dst),
            TLVValue::NumberOfMessages(value) => value.encode(dst),
            TLVValue::PayloadType(value) => value.encode(dst),
            TLVValue::PrivacyIndicator(value) => value.encode(dst),
            TLVValue::QosTimeToLive(value) => value.encode(dst),
            TLVValue::ReceiptedMessageId(value) => value.encode(dst),
            TLVValue::SarMsgRefNum(value) => value.encode(dst),
            TLVValue::SarSegmentSeqnum(value) => value.encode(dst),
            TLVValue::SarTotalSegments(value) => value.encode(dst),
            TLVValue::ScInterfaceVersion(value) => value.encode(dst),
            TLVValue::SetDpf(value) => value.encode(dst),
            TLVValue::SmsSignal(value) => value.encode(dst),
            TLVValue::SourceAddrSubunit(value) => value.encode(dst),
            TLVValue::SourceBearerType(value) => value.encode(dst),
            TLVValue::SourceNetworkId(value) => value.encode(dst),
            TLVValue::SourceNetworkType(value) => value.encode(dst),
            TLVValue::SourceNodeId(value) => value.encode(dst),
            TLVValue::SourcePort(value) => value.encode(dst),
            TLVValue::SourceSubaddress(value) => value.encode(dst),
            TLVValue::SourceTelematicsId(value) => value.encode(dst),
            TLVValue::UserMessageReference(value) => value.encode(dst),
            TLVValue::UserResponseCode(value) => value.encode(dst),
            TLVValue::UssdServiceOp(value) => value.encode(dst),
            TLVValue::Other { value, .. } => value.encode(dst),
        }
    }
}

impl DecodeWithKey for TLVValue {
    type Key = TLVTag;

    fn decode(key: Self::Key, src: &[u8], length: usize) -> Result<(Self, usize), DecodeError> {
        let (value, size) = match key {
            TLVTag::AdditionalStatusInfoText => {
                Decode::decode(src).map_decoded(Self::AdditionalStatusInfoText)?
            }
            TLVTag::AlertOnMessageDelivery => {
                Decode::decode(src).map_decoded(Self::AlertOnMessageDelivery)?
            }
            TLVTag::BillingIdentification => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BillingIdentification)?
            }
            TLVTag::BroadcastAreaIdentifier => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastAreaIdentifier)?
            }
            TLVTag::BroadcastAreaSuccess => {
                Decode::decode(src).map_decoded(Self::BroadcastAreaSuccess)?
            }
            TLVTag::BroadcastContentTypeInfo => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastContentTypeInfo)?
            }
            TLVTag::BroadcastChannelIndicator => {
                Decode::decode(src).map_decoded(Self::BroadcastChannelIndicator)?
            }
            TLVTag::BroadcastContentType => {
                Decode::decode(src).map_decoded(Self::BroadcastContentType)?
            }
            TLVTag::BroadcastEndTime => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastEndTime)?
            }
            TLVTag::BroadcastErrorStatus => {
                Decode::decode(src).map_decoded(Self::BroadcastErrorStatus)?
            }
            TLVTag::BroadcastFrequencyInterval => {
                Decode::decode(src).map_decoded(Self::BroadcastFrequencyInterval)?
            }
            TLVTag::BroadcastMessageClass => {
                Decode::decode(src).map_decoded(Self::BroadcastMessageClass)?
            }
            TLVTag::BroadcastRepNum => Decode::decode(src).map_decoded(Self::BroadcastRepNum)?,
            TLVTag::BroadcastServiceGroup => {
                DecodeWithLength::decode(src, length).map_decoded(Self::BroadcastServiceGroup)?
            }
            TLVTag::CallbackNum => {
                DecodeWithLength::decode(src, length).map_decoded(Self::CallbackNum)?
            }
            TLVTag::CallbackNumAtag => {
                DecodeWithLength::decode(src, length).map_decoded(Self::CallbackNumAtag)?
            }
            TLVTag::CallbackNumPresInd => {
                Decode::decode(src).map_decoded(Self::CallbackNumPresInd)?
            }
            TLVTag::CongestionState => Decode::decode(src).map_decoded(Self::CongestionState)?,
            TLVTag::DeliveryFailureReason => {
                Decode::decode(src).map_decoded(Self::DeliveryFailureReason)?
            }
            TLVTag::DestAddrNpCountry => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestAddrNpCountry)?
            }
            TLVTag::DestAddrNpInformation => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestAddrNpInformation)?
            }
            TLVTag::DestAddrNpResolution => {
                Decode::decode(src).map_decoded(Self::DestAddrNpResolution)?
            }
            TLVTag::DestAddrSubunit => Decode::decode(src).map_decoded(Self::DestAddrSubunit)?,
            TLVTag::DestBearerType => Decode::decode(src).map_decoded(Self::DestBearerType)?,
            TLVTag::DestNetworkId => Decode::decode(src).map_decoded(Self::DestNetworkId)?,
            TLVTag::DestNetworkType => Decode::decode(src).map_decoded(Self::DestNetworkType)?,
            TLVTag::DestNodeId => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestNodeId)?
            }
            TLVTag::DestSubaddress => {
                DecodeWithLength::decode(src, length).map_decoded(Self::DestSubaddress)?
            }
            TLVTag::DestTelematicsId => Decode::decode(src).map_decoded(Self::DestTelematicsId)?,
            TLVTag::DestPort => Decode::decode(src).map_decoded(Self::DestPort)?,
            TLVTag::DisplayTime => Decode::decode(src).map_decoded(Self::DisplayTime)?,
            TLVTag::DpfResult => Decode::decode(src).map_decoded(Self::DpfResult)?,
            TLVTag::ItsReplyType => Decode::decode(src).map_decoded(Self::ItsReplyType)?,
            TLVTag::ItsSessionInfo => Decode::decode(src).map_decoded(Self::ItsSessionInfo)?,
            TLVTag::LanguageIndicator => {
                Decode::decode(src).map_decoded(Self::LanguageIndicator)?
            }
            TLVTag::MessagePayload => {
                DecodeWithLength::decode(src, length).map_decoded(Self::MessagePayload)?
            }
            TLVTag::MessageState => Decode::decode(src).map_decoded(Self::MessageState)?,
            TLVTag::MoreMessagesToSend => {
                Decode::decode(src).map_decoded(Self::MoreMessagesToSend)?
            }
            TLVTag::MsAvailabilityStatus => {
                Decode::decode(src).map_decoded(Self::MsAvailabilityStatus)?
            }
            TLVTag::MsMsgWaitFacilities => {
                Decode::decode(src).map_decoded(Self::MsMsgWaitFacilities)?
            }
            TLVTag::MsValidity => {
                DecodeWithLength::decode(src, length).map_decoded(Self::MsValidity)?
            }
            TLVTag::NetworkErrorCode => Decode::decode(src).map_decoded(Self::NetworkErrorCode)?,
            TLVTag::NumberOfMessages => Decode::decode(src).map_decoded(Self::NumberOfMessages)?,
            TLVTag::PayloadType => Decode::decode(src).map_decoded(Self::PayloadType)?,
            TLVTag::PrivacyIndicator => Decode::decode(src).map_decoded(Self::PrivacyIndicator)?,
            TLVTag::QosTimeToLive => Decode::decode(src).map_decoded(Self::QosTimeToLive)?,
            TLVTag::ReceiptedMessageId => {
                Decode::decode(src).map_decoded(Self::ReceiptedMessageId)?
            }
            TLVTag::SarMsgRefNum => Decode::decode(src).map_decoded(Self::SarMsgRefNum)?,
            TLVTag::SarSegmentSeqnum => Decode::decode(src).map_decoded(Self::SarSegmentSeqnum)?,
            TLVTag::SarTotalSegments => Decode::decode(src).map_decoded(Self::SarTotalSegments)?,
            TLVTag::ScInterfaceVersion => {
                Decode::decode(src).map_decoded(Self::ScInterfaceVersion)?
            }
            TLVTag::SetDpf => Decode::decode(src).map_decoded(Self::SetDpf)?,
            TLVTag::SmsSignal => Decode::decode(src).map_decoded(Self::SmsSignal)?,
            TLVTag::SourceAddrSubunit => {
                Decode::decode(src).map_decoded(Self::SourceAddrSubunit)?
            }
            TLVTag::SourceBearerType => Decode::decode(src).map_decoded(Self::SourceBearerType)?,
            TLVTag::SourceNetworkId => Decode::decode(src).map_decoded(Self::SourceNetworkId)?,
            TLVTag::SourceNetworkType => {
                Decode::decode(src).map_decoded(Self::SourceNetworkType)?
            }
            TLVTag::SourceNodeId => {
                DecodeWithLength::decode(src, length).map_decoded(Self::SourceNodeId)?
            }
            TLVTag::SourcePort => Decode::decode(src).map_decoded(Self::SourcePort)?,
            TLVTag::SourceSubaddress => {
                DecodeWithLength::decode(src, length).map_decoded(Self::SourceSubaddress)?
            }
            TLVTag::SourceTelematicsId => {
                Decode::decode(src).map_decoded(Self::SourceTelematicsId)?
            }
            TLVTag::UserMessageReference => {
                Decode::decode(src).map_decoded(Self::UserMessageReference)?
            }
            TLVTag::UserResponseCode => Decode::decode(src).map_decoded(Self::UserResponseCode)?,
            TLVTag::UssdServiceOp => Decode::decode(src).map_decoded(Self::UssdServiceOp)?,
            other => DecodeWithLength::decode(src, length)
                .map_decoded(|value| TLVValue::Other { tag: other, value })?,
        };

        Ok((value, size))
    }
}
