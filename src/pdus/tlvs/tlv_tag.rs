use crate::io::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TLVTag {
    DestAddrSubunit = 0x0005,
    DestNetworkType = 0x0006,
    DestBearerType = 0x0007,
    DestTelematicsId = 0x0008,
    SourceAddrSubunit = 0x000D,
    SourceNetworkType = 0x000E,
    SourceBearerType = 0x000F,
    SourceTelematicsId = 0x0010,
    QosTimeToLive = 0x0017,
    PayloadType = 0x0019,
    AdditionalStatusInfoText = 0x001D,
    ReceiptedMessageId = 0x001E,
    MsMsgWaitFacilities = 0x0030,
    PrivacyIndicator = 0x0201,
    SourceSubaddress = 0x0202,
    DestSubaddress = 0x0203,
    UserMessageReference = 0x0204,
    UserResponseCode = 0x0205,
    SourcePort = 0x020A,
    DestPort = 0x020B,
    SarMsgRefNum = 0x020C,
    LanguageIndicator = 0x020D,
    SarTotalSegments = 0x020E,
    SarSegmentSeqnum = 0x020F,
    ScInterfaceVersion = 0x0210,
    CallbackNumPresInd = 0x0302,
    CallbackNumAtag = 0x0303,
    NumberOfMessages = 0x0304,
    CallbackNum = 0x0381,
    DpfResult = 0x0420,
    SetDpf = 0x0421,
    MsAvailabilityStatus = 0x0422,
    NetworkErrorCode = 0x0423,
    MessagePayload = 0x0424,
    DeliveryFailureReason = 0x0425,
    MoreMessagesToSend = 0x0426,
    MessageState = 0x0427,
    CongestionState = 0x0428,
    UssdServiceOp = 0x0501,
    BroadcastChannelIndicator = 0x0600,
    BroadcastContentType = 0x0601,
    BroadcastContentTypeInfo = 0x0602,
    BroadcastMessageClass = 0x0603,
    BroadcastRepNum = 0x0604,
    BroadcastFrequencyInterval = 0x0605,
    BroadcastAreaIdentifier = 0x0606,
    BroadcastErrorStatus = 0x0607,
    BroadcastAreaSuccess = 0x0608,
    BroadcastEndTime = 0x0609,
    BroadcastServiceGroup = 0x060A,
    BillingIdentification = 0x060B,
    SourceNetworkId = 0x060D,
    DestNetworkId = 0x060E,
    SourceNodeId = 0x060F,
    DestNodeId = 0x0610,
    DestAddrNpResolution = 0x0611,
    DestAddrNpInformation = 0x0612,
    DestAddrNpCountry = 0x0613,
    DisplayTime = 0x1201,
    SmsSignal = 0x1203,
    MsValidity = 0x1204,
    AlertOnMessageDelivery = 0x130C,
    ItsReplyType = 0x1380,
    ItsSessionInfo = 0x1383,
    Other(u16),
}

impl From<u16> for TLVTag {
    fn from(value: u16) -> Self {
        match value {
            0x0005 => TLVTag::DestAddrSubunit,
            0x0006 => TLVTag::DestNetworkType,
            0x0007 => TLVTag::DestBearerType,
            0x0008 => TLVTag::DestTelematicsId,
            0x000D => TLVTag::SourceAddrSubunit,
            0x000E => TLVTag::SourceNetworkType,
            0x000F => TLVTag::SourceBearerType,
            0x0010 => TLVTag::SourceTelematicsId,
            0x0017 => TLVTag::QosTimeToLive,
            0x0019 => TLVTag::PayloadType,
            0x001D => TLVTag::AdditionalStatusInfoText,
            0x001E => TLVTag::ReceiptedMessageId,
            0x0030 => TLVTag::MsMsgWaitFacilities,
            0x0201 => TLVTag::PrivacyIndicator,
            0x0202 => TLVTag::SourceSubaddress,
            0x0203 => TLVTag::DestSubaddress,
            0x0204 => TLVTag::UserMessageReference,
            0x0205 => TLVTag::UserResponseCode,
            0x020A => TLVTag::SourcePort,
            0x020B => TLVTag::DestPort,
            0x020C => TLVTag::SarMsgRefNum,
            0x020D => TLVTag::LanguageIndicator,
            0x020E => TLVTag::SarTotalSegments,
            0x020F => TLVTag::SarSegmentSeqnum,
            0x0210 => TLVTag::ScInterfaceVersion,
            0x0302 => TLVTag::CallbackNumPresInd,
            0x0303 => TLVTag::CallbackNumAtag,
            0x0304 => TLVTag::NumberOfMessages,
            0x0381 => TLVTag::CallbackNum,
            0x0420 => TLVTag::DpfResult,
            0x0421 => TLVTag::SetDpf,
            0x0422 => TLVTag::MsAvailabilityStatus,
            0x0423 => TLVTag::NetworkErrorCode,
            0x0424 => TLVTag::MessagePayload,
            0x0425 => TLVTag::DeliveryFailureReason,
            0x0426 => TLVTag::MoreMessagesToSend,
            0x0427 => TLVTag::MessageState,
            0x0428 => TLVTag::CongestionState,
            0x0501 => TLVTag::UssdServiceOp,
            0x0600 => TLVTag::BroadcastChannelIndicator,
            0x0601 => TLVTag::BroadcastContentType,
            0x0602 => TLVTag::BroadcastContentTypeInfo,
            0x0603 => TLVTag::BroadcastMessageClass,
            0x0604 => TLVTag::BroadcastRepNum,
            0x0605 => TLVTag::BroadcastFrequencyInterval,
            0x0606 => TLVTag::BroadcastAreaIdentifier,
            0x0607 => TLVTag::BroadcastErrorStatus,
            0x0608 => TLVTag::BroadcastAreaSuccess,
            0x0609 => TLVTag::BroadcastEndTime,
            0x060A => TLVTag::BroadcastServiceGroup,
            0x060B => TLVTag::BillingIdentification,
            0x060D => TLVTag::SourceNetworkId,
            0x060E => TLVTag::DestNetworkId,
            0x060F => TLVTag::SourceNodeId,
            0x0610 => TLVTag::DestNodeId,
            0x0611 => TLVTag::DestAddrNpResolution,
            0x0612 => TLVTag::DestAddrNpInformation,
            0x0613 => TLVTag::DestAddrNpCountry,
            0x1201 => TLVTag::DisplayTime,
            0x1203 => TLVTag::SmsSignal,
            0x1204 => TLVTag::MsValidity,
            0x130C => TLVTag::AlertOnMessageDelivery,
            0x1380 => TLVTag::ItsReplyType,
            0x1383 => TLVTag::ItsSessionInfo,
            other => TLVTag::Other(other),
        }
    }
}

impl From<TLVTag> for u16 {
    fn from(value: TLVTag) -> Self {
        match value {
            TLVTag::DestAddrSubunit => 0x0005,
            TLVTag::DestNetworkType => 0x0006,
            TLVTag::DestBearerType => 0x0007,
            TLVTag::DestTelematicsId => 0x0008,
            TLVTag::SourceAddrSubunit => 0x000D,
            TLVTag::SourceNetworkType => 0x000E,
            TLVTag::SourceBearerType => 0x000F,
            TLVTag::SourceTelematicsId => 0x0010,
            TLVTag::QosTimeToLive => 0x0017,
            TLVTag::PayloadType => 0x0019,
            TLVTag::AdditionalStatusInfoText => 0x001D,
            TLVTag::ReceiptedMessageId => 0x001E,
            TLVTag::MsMsgWaitFacilities => 0x0030,
            TLVTag::PrivacyIndicator => 0x0201,
            TLVTag::SourceSubaddress => 0x0202,
            TLVTag::DestSubaddress => 0x0203,
            TLVTag::UserMessageReference => 0x0204,
            TLVTag::UserResponseCode => 0x0205,
            TLVTag::SourcePort => 0x020A,
            TLVTag::DestPort => 0x020B,
            TLVTag::SarMsgRefNum => 0x020C,
            TLVTag::LanguageIndicator => 0x020D,
            TLVTag::SarTotalSegments => 0x020E,
            TLVTag::SarSegmentSeqnum => 0x020F,
            TLVTag::ScInterfaceVersion => 0x0210,
            TLVTag::CallbackNumPresInd => 0x0302,
            TLVTag::CallbackNumAtag => 0x0303,
            TLVTag::NumberOfMessages => 0x0304,
            TLVTag::CallbackNum => 0x0381,
            TLVTag::DpfResult => 0x0420,
            TLVTag::SetDpf => 0x0421,
            TLVTag::MsAvailabilityStatus => 0x0422,
            TLVTag::NetworkErrorCode => 0x0423,
            TLVTag::MessagePayload => 0x0424,
            TLVTag::DeliveryFailureReason => 0x0425,
            TLVTag::MoreMessagesToSend => 0x0426,
            TLVTag::MessageState => 0x0427,
            TLVTag::CongestionState => 0x0428,
            TLVTag::UssdServiceOp => 0x0501,
            TLVTag::BroadcastChannelIndicator => 0x0600,
            TLVTag::BroadcastContentType => 0x0601,
            TLVTag::BroadcastContentTypeInfo => 0x0602,
            TLVTag::BroadcastMessageClass => 0x0603,
            TLVTag::BroadcastRepNum => 0x0604,
            TLVTag::BroadcastFrequencyInterval => 0x0605,
            TLVTag::BroadcastAreaIdentifier => 0x0606,
            TLVTag::BroadcastErrorStatus => 0x0607,
            TLVTag::BroadcastAreaSuccess => 0x0608,
            TLVTag::BroadcastEndTime => 0x0609,
            TLVTag::BroadcastServiceGroup => 0x060A,
            TLVTag::BillingIdentification => 0x060B,
            TLVTag::SourceNetworkId => 0x060D,
            TLVTag::DestNetworkId => 0x060E,
            TLVTag::SourceNodeId => 0x060F,
            TLVTag::DestNodeId => 0x0610,
            TLVTag::DestAddrNpResolution => 0x0611,
            TLVTag::DestAddrNpInformation => 0x0612,
            TLVTag::DestAddrNpCountry => 0x0613,
            TLVTag::DisplayTime => 0x1201,
            TLVTag::SmsSignal => 0x1203,
            TLVTag::MsValidity => 0x1204,
            TLVTag::AlertOnMessageDelivery => 0x130C,
            TLVTag::ItsReplyType => 0x1380,
            TLVTag::ItsSessionInfo => 0x1383,
            TLVTag::Other(other) => other,
        }
    }
}

impl From<TLVTag> for u32 {
    fn from(value: TLVTag) -> Self {
        u16::from(value) as u32
    }
}

impl From<u32> for TLVTag {
    fn from(value: u32) -> Self {
        TLVTag::from(value as u16)
    }
}

impl Length for TLVTag {
    fn length(&self) -> usize {
        2
    }
}

impl Encode for TLVTag {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u16::from(*self).encode_to(writer)
    }
}

impl Decode for TLVTag {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u16::decode_from(reader)?);

        Ok(value)
    }
}
