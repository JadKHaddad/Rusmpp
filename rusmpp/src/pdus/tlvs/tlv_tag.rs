use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU16;

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU16,
)]
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
    #[num_enum(catch_all)]
    Other(u16),
}

impl From<u32> for TLVTag {
    fn from(v: u32) -> Self {
        Self::from(v as u16)
    }
}

impl From<TLVTag> for u32 {
    fn from(v: TLVTag) -> Self {
        v.into()
    }
}

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
    fn from(v: MessageSubmissionRequestTLVTag) -> Self {
        match v {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageSubmissionResponseTLVTag {
    AdditionalStatusInfoText,
    DeliveryFailureReason,
    DpfResult,
    NetworkErrorCode,
}

impl From<MessageSubmissionResponseTLVTag> for TLVTag {
    fn from(v: MessageSubmissionResponseTLVTag) -> Self {
        match v {
            MessageSubmissionResponseTLVTag::AdditionalStatusInfoText => {
                TLVTag::AdditionalStatusInfoText
            }
            MessageSubmissionResponseTLVTag::DeliveryFailureReason => TLVTag::DeliveryFailureReason,
            MessageSubmissionResponseTLVTag::DpfResult => TLVTag::DpfResult,
            MessageSubmissionResponseTLVTag::NetworkErrorCode => TLVTag::NetworkErrorCode,
        }
    }
}

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
    fn from(v: MessageDeliveryRequestTLVTag) -> Self {
        match v {
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
pub enum MessageDeliveryResponseTLVTag {
    AdditionalStatusInfoText,
    DeliveryFailureReason,
    NetworkErrorCode,
}

impl From<MessageDeliveryResponseTLVTag> for TLVTag {
    fn from(v: MessageDeliveryResponseTLVTag) -> Self {
        match v {
            MessageDeliveryResponseTLVTag::AdditionalStatusInfoText => {
                TLVTag::AdditionalStatusInfoText
            }
            MessageDeliveryResponseTLVTag::DeliveryFailureReason => TLVTag::DeliveryFailureReason,
            MessageDeliveryResponseTLVTag::NetworkErrorCode => TLVTag::NetworkErrorCode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastRequestTLVTag {
    AlertOnMsgDelivery,
    BroadcastChannelIndicator,
    BroadcastContentTypeInfo,
    BroadcastMessageClass,
    BroadcastServiceGroup,
    CallbackNum,
    CallbackNumAtag,
    CallbackNumPresInd,
    DestAddrSubunit,
    DestSubaddress,
    DestPort,
    DisplayTime,
    LanguageIndicator,
    MessagePayload,
    MsValidity,
    PayloadType,
    PrivacyIndicator,
    SmsSignal,
    SourceAddrSubunit,
    SourcePort,
    SourceSubaddress,
    UserMessageReference,
}

impl From<BroadcastRequestTLVTag> for TLVTag {
    fn from(v: BroadcastRequestTLVTag) -> Self {
        match v {
            BroadcastRequestTLVTag::AlertOnMsgDelivery => TLVTag::AlertOnMessageDelivery,
            BroadcastRequestTLVTag::BroadcastChannelIndicator => TLVTag::BroadcastChannelIndicator,
            BroadcastRequestTLVTag::BroadcastContentTypeInfo => TLVTag::BroadcastContentTypeInfo,
            BroadcastRequestTLVTag::BroadcastMessageClass => TLVTag::BroadcastMessageClass,
            BroadcastRequestTLVTag::BroadcastServiceGroup => TLVTag::BroadcastServiceGroup,
            BroadcastRequestTLVTag::CallbackNum => TLVTag::CallbackNum,
            BroadcastRequestTLVTag::CallbackNumAtag => TLVTag::CallbackNumAtag,
            BroadcastRequestTLVTag::CallbackNumPresInd => TLVTag::CallbackNumPresInd,
            BroadcastRequestTLVTag::DestAddrSubunit => TLVTag::DestAddrSubunit,
            BroadcastRequestTLVTag::DestSubaddress => TLVTag::DestSubaddress,
            BroadcastRequestTLVTag::DestPort => TLVTag::DestPort,
            BroadcastRequestTLVTag::DisplayTime => TLVTag::DisplayTime,
            BroadcastRequestTLVTag::LanguageIndicator => TLVTag::LanguageIndicator,
            BroadcastRequestTLVTag::MessagePayload => TLVTag::MessagePayload,
            BroadcastRequestTLVTag::MsValidity => TLVTag::MsValidity,
            BroadcastRequestTLVTag::PayloadType => TLVTag::PayloadType,
            BroadcastRequestTLVTag::PrivacyIndicator => TLVTag::PrivacyIndicator,
            BroadcastRequestTLVTag::SmsSignal => TLVTag::SmsSignal,
            BroadcastRequestTLVTag::SourceAddrSubunit => TLVTag::SourceAddrSubunit,
            BroadcastRequestTLVTag::SourcePort => TLVTag::SourcePort,
            BroadcastRequestTLVTag::SourceSubaddress => TLVTag::SourceSubaddress,
            BroadcastRequestTLVTag::UserMessageReference => TLVTag::UserMessageReference,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastResponseTLVTag {
    BroadcastErrorStatus,
    BroadcastAreaIdentifier,
}

impl From<BroadcastResponseTLVTag> for TLVTag {
    fn from(v: BroadcastResponseTLVTag) -> Self {
        match v {
            BroadcastResponseTLVTag::BroadcastErrorStatus => TLVTag::BroadcastErrorStatus,
            BroadcastResponseTLVTag::BroadcastAreaIdentifier => TLVTag::BroadcastAreaIdentifier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueryBroadcastResponseTLVTag {
    BroadcastEndTime,
    UserMessageReference,
}

impl From<QueryBroadcastResponseTLVTag> for TLVTag {
    fn from(v: QueryBroadcastResponseTLVTag) -> Self {
        match v {
            QueryBroadcastResponseTLVTag::BroadcastEndTime => TLVTag::BroadcastEndTime,
            QueryBroadcastResponseTLVTag::UserMessageReference => TLVTag::UserMessageReference,
        }
    }
}
