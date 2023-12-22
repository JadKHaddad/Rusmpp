use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[repr(u16)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
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

impl IoLength for TLVTag {
    fn length(&self) -> usize {
        u16::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for TLVTag {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u16::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for TLVTag {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u16::async_io_read(buf).await.map(Self::from)
    }
}
