use super::tlv_tag::TLVTag;
use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength, OptionalDecodeWithKey},
        encode::{Encode, EncodeError},
        length::Length,
    },
    pdus::types::interface_version::InterfaceVersion,
    tri,
    types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TLVValue {
    // AdditionalStatusInfoText(COctetString<1, 256>),
    // AlertOnMessageDelivery(AlertOnMsgDelivery),
    // BillingIdentification(OctetString<0, 1024>),
    // BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    // BroadcastAreaSuccess(BroadcastAreaSuccess),
    // BroadcastContentTypeInfo(OctetString<0, 255>),
    // BroadcastChannelIndicator(BroadcastChannelIndicator),
    // BroadcastContentType(BroadcastContentType),
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
    // BroadcastEndTime(OctetString<0, 17>),
    // BroadcastErrorStatus(CommandStatus),
    // BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    // BroadcastMessageClass(BroadcastMessageClass),
    // BroadcastRepNum(u16),
    // BroadcastServiceGroup(OctetString<1, 255>),
    // CallbackNum(OctetString<4, 19>),
    // CallbackNumAtag(OctetString<0, 65>),
    // CallbackNumPresInd(CallbackNumPresInd),
    // CongestionState(CongestionState),
    // DeliveryFailureReason(DeliveryFailureReason),
    // DestAddrNpCountry(OctetString<1, 5>),
    // DestAddrNpInformation(OctetString<0, 10>),
    // DestAddrNpResolution(DestAddrNpResolution),
    // DestAddrSubunit(AddrSubunit),
    // DestBearerType(BearerType),
    // DestNetworkId(COctetString<7, 66>),
    // DestNetworkType(NetworkType),
    // DestNodeId(OctetString<6, 6>),
    // DestSubaddress(Subaddress),
    // DestTelematicsId(u16),
    // DestPort(u16),
    // DisplayTime(DisplayTime),
    // DpfResult(DpfResult),
    // ItsReplyType(ItsReplyType),
    // ItsSessionInfo(ItsSessionInfo),
    // LanguageIndicator(LanguageIndicator),
    // MessagePayload(NoFixedSizeOctetString),
    // MessageState(MessageState),
    // MoreMessagesToSend(MoreMessagesToSend),
    // MsAvailabilityStatus(MsAvailabilityStatus),
    // MsMsgWaitFacilities(MsMsgWaitFacilities),
    // MsValidity(MsValidity),
    // NetworkErrorCode(NetworkErrorCode),
    // NumberOfMessages(NumberOfMessages),
    // PayloadType(PayloadType),
    // PrivacyIndicator(PrivacyIndicator),
    // QosTimeToLive(u32),
    // ReceiptedMessageId(COctetString<1, 65>),
    // SarMsgRefNum(u16),
    // SarSegmentSeqnum(u8),
    // SarTotalSegments(u8),
    ScInterfaceVersion(InterfaceVersion),
    // SetDpf(SetDpf),
    // /// Encoded as per [CMT-136]
    // SmsSignal(u16),
    // SourceAddrSubunit(AddrSubunit),
    // SourceBearerType(BearerType),
    // SourceNetworkId(COctetString<7, 66>),
    // SourceNetworkType(NetworkType),
    // SourceNodeId(OctetString<6, 6>),
    // SourcePort(u16),
    // SourceSubaddress(Subaddress),
    // SourceTelematicsId(u16),
    // UserMessageReference(u16),
    // UserResponseCode(u8),
    // UssdServiceOp(UssdServiceOp),
    Other {
        tag: TLVTag,
        value: NoFixedSizeOctetString,
    },
}

impl TLVValue {
    pub fn tlv_tag(&self) -> TLVTag {
        match self {
            // TLVValue::AdditionalStatusInfoText(_) => TLVTag::AdditionalStatusInfoText,
            // TLVValue::AlertOnMessageDelivery(_) => TLVTag::AlertOnMessageDelivery,
            // TLVValue::BillingIdentification(_) => TLVTag::BillingIdentification,
            // TLVValue::BroadcastAreaIdentifier(_) => TLVTag::BroadcastAreaIdentifier,
            // TLVValue::BroadcastAreaSuccess(_) => TLVTag::BroadcastAreaSuccess,
            // TLVValue::BroadcastContentTypeInfo(_) => TLVTag::BroadcastContentTypeInfo,
            // TLVValue::BroadcastChannelIndicator(_) => TLVTag::BroadcastChannelIndicator,
            // TLVValue::BroadcastContentType(_) => TLVTag::BroadcastContentType,
            // TLVValue::BroadcastEndTime(_) => TLVTag::BroadcastEndTime,
            // TLVValue::BroadcastErrorStatus(_) => TLVTag::BroadcastErrorStatus,
            // TLVValue::BroadcastFrequencyInterval(_) => TLVTag::BroadcastFrequencyInterval,
            // TLVValue::BroadcastMessageClass(_) => TLVTag::BroadcastMessageClass,
            // TLVValue::BroadcastRepNum(_) => TLVTag::BroadcastRepNum,
            // TLVValue::BroadcastServiceGroup(_) => TLVTag::BroadcastServiceGroup,
            // TLVValue::CallbackNum(_) => TLVTag::CallbackNum,
            // TLVValue::CallbackNumAtag(_) => TLVTag::CallbackNumAtag,
            // TLVValue::CallbackNumPresInd(_) => TLVTag::CallbackNumPresInd,
            // TLVValue::CongestionState(_) => TLVTag::CongestionState,
            // TLVValue::DeliveryFailureReason(_) => TLVTag::DeliveryFailureReason,
            // TLVValue::DestAddrNpCountry(_) => TLVTag::DestAddrNpCountry,
            // TLVValue::DestAddrNpInformation(_) => TLVTag::DestAddrNpInformation,
            // TLVValue::DestAddrNpResolution(_) => TLVTag::DestAddrNpResolution,
            // TLVValue::DestAddrSubunit(_) => TLVTag::DestAddrSubunit,
            // TLVValue::DestBearerType(_) => TLVTag::DestBearerType,
            // TLVValue::DestNetworkId(_) => TLVTag::DestNetworkId,
            // TLVValue::DestNetworkType(_) => TLVTag::DestNetworkType,
            // TLVValue::DestNodeId(_) => TLVTag::DestNodeId,
            // TLVValue::DestSubaddress(_) => TLVTag::DestSubaddress,
            // TLVValue::DestTelematicsId(_) => TLVTag::DestTelematicsId,
            // TLVValue::DestPort(_) => TLVTag::DestPort,
            // TLVValue::DisplayTime(_) => TLVTag::DisplayTime,
            // TLVValue::DpfResult(_) => TLVTag::DpfResult,
            // TLVValue::ItsReplyType(_) => TLVTag::ItsReplyType,
            // TLVValue::ItsSessionInfo(_) => TLVTag::ItsSessionInfo,
            // TLVValue::LanguageIndicator(_) => TLVTag::LanguageIndicator,
            // TLVValue::MessagePayload(_) => TLVTag::MessagePayload,
            // TLVValue::MessageState(_) => TLVTag::MessageState,
            // TLVValue::MoreMessagesToSend(_) => TLVTag::MoreMessagesToSend,
            // TLVValue::MsAvailabilityStatus(_) => TLVTag::MsAvailabilityStatus,
            // TLVValue::MsMsgWaitFacilities(_) => TLVTag::MsMsgWaitFacilities,
            // TLVValue::MsValidity(_) => TLVTag::MsValidity,
            // TLVValue::NetworkErrorCode(_) => TLVTag::NetworkErrorCode,
            // TLVValue::NumberOfMessages(_) => TLVTag::NumberOfMessages,
            // TLVValue::PayloadType(_) => TLVTag::PayloadType,
            // TLVValue::PrivacyIndicator(_) => TLVTag::PrivacyIndicator,
            // TLVValue::QosTimeToLive(_) => TLVTag::QosTimeToLive,
            // TLVValue::ReceiptedMessageId(_) => TLVTag::ReceiptedMessageId,
            // TLVValue::SarMsgRefNum(_) => TLVTag::SarMsgRefNum,
            // TLVValue::SarSegmentSeqnum(_) => TLVTag::SarSegmentSeqnum,
            // TLVValue::SarTotalSegments(_) => TLVTag::SarTotalSegments,
            TLVValue::ScInterfaceVersion(_) => TLVTag::ScInterfaceVersion,
            // TLVValue::SetDpf(_) => TLVTag::SetDpf,
            // TLVValue::SmsSignal(_) => TLVTag::SmsSignal,
            // TLVValue::SourceAddrSubunit(_) => TLVTag::SourceAddrSubunit,
            // TLVValue::SourceBearerType(_) => TLVTag::SourceBearerType,
            // TLVValue::SourceNetworkId(_) => TLVTag::SourceNetworkId,
            // TLVValue::SourceNetworkType(_) => TLVTag::SourceNetworkType,
            // TLVValue::SourceNodeId(_) => TLVTag::SourceNodeId,
            // TLVValue::SourcePort(_) => TLVTag::SourcePort,
            // TLVValue::SourceSubaddress(_) => TLVTag::SourceSubaddress,
            // TLVValue::SourceTelematicsId(_) => TLVTag::SourceTelematicsId,
            // TLVValue::UserMessageReference(_) => TLVTag::UserMessageReference,
            // TLVValue::UserResponseCode(_) => TLVTag::UserResponseCode,
            // TLVValue::UssdServiceOp(_) => TLVTag::UssdServiceOp,
            TLVValue::Other { tag, .. } => *tag,
        }
    }
}

impl Length for TLVValue {
    fn length(&self) -> usize {
        match self {
            TLVValue::ScInterfaceVersion(value) => value.length(),
            TLVValue::Other { value, .. } => value.length(),
        }
    }
}

impl Encode for TLVValue {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            TLVValue::ScInterfaceVersion(value) => value.encode_to(writer),
            TLVValue::Other { value, .. } => value.encode_to(writer),
        }
    }
}

impl OptionalDecodeWithKey for TLVValue {
    type Key = TLVTag;

    fn decode_from<R: std::io::Read>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized,
    {
        let value = match key {
            TLVTag::ScInterfaceVersion => {
                TLVValue::ScInterfaceVersion(tri!(InterfaceVersion::decode_from(reader)))
            }
            TLVTag::Other(_) => {
                let value = tri!(NoFixedSizeOctetString::decode_from(reader, length));
                TLVValue::Other { tag: key, value }
            }
            _ => return Ok(None),
        };

        Ok(Some(value))
    }
}
