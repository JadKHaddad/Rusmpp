use super::TLVValue;
use rusmpp_io::io::write::{AsyncIoWritable, AsyncIoWrite, IoWritable, IoWrite};

#[async_trait::async_trait]
impl AsyncIoWrite for TLVValue {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        match self {
            TLVValue::AdditionalStatusInfoText(v) => v.async_io_write(buf).await,
            TLVValue::AlertOnMessageDelivery(v) => v.async_io_write(buf).await,
            TLVValue::BillingIdentification(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastAreaIdentifier(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastAreaSuccess(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastContentTypeInfo(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastChannelIndicator(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastContentType(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastEndTime(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastErrorStatus(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastFrequencyInterval(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastMessageClass(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastRepNum(v) => v.async_io_write(buf).await,
            TLVValue::BroadcastServiceGroup(v) => v.async_io_write(buf).await,
            TLVValue::CallbackNum(v) => v.async_io_write(buf).await,
            TLVValue::CallbackNumAtag(v) => v.async_io_write(buf).await,
            TLVValue::CallbackNumPresInd(v) => v.async_io_write(buf).await,
            TLVValue::CongestionState(v) => v.async_io_write(buf).await,
            TLVValue::DeliveryFailureReason(v) => v.async_io_write(buf).await,
            TLVValue::DestAddrNpCountry(v) => v.async_io_write(buf).await,
            TLVValue::DestAddrNpInformation(v) => v.async_io_write(buf).await,
            TLVValue::DestAddrNpResolution(v) => v.async_io_write(buf).await,
            TLVValue::DestAddrSubunit(v) => v.async_io_write(buf).await,
            TLVValue::DestBearerType(v) => v.async_io_write(buf).await,
            TLVValue::DestNetworkId(v) => v.async_io_write(buf).await,
            TLVValue::DestNetworkType(v) => v.async_io_write(buf).await,
            TLVValue::DestNodeId(v) => v.async_io_write(buf).await,
            TLVValue::DestSubaddress(v) => v.async_io_write(buf).await,
            TLVValue::DestTelematicsId(v) => v.async_io_write(buf).await,
            TLVValue::DestPort(v) => v.async_io_write(buf).await,
            TLVValue::DisplayTime(v) => v.async_io_write(buf).await,
            TLVValue::DpfResult(v) => v.async_io_write(buf).await,
            TLVValue::ItsReplyType(v) => v.async_io_write(buf).await,
            TLVValue::ItsSessionInfo(v) => v.async_io_write(buf).await,
            TLVValue::LanguageIndicator(v) => v.async_io_write(buf).await,
            TLVValue::MessagePayload(v) => v.async_io_write(buf).await,
            TLVValue::MessageState(v) => v.async_io_write(buf).await,
            TLVValue::MoreMessagesToSend(v) => v.async_io_write(buf).await,
            TLVValue::MsAvailabilityStatus(v) => v.async_io_write(buf).await,
            TLVValue::MsMsgWaitFacilities(v) => v.async_io_write(buf).await,
            TLVValue::MsValidity(v) => v.async_io_write(buf).await,
            TLVValue::NetworkErrorCode(v) => v.async_io_write(buf).await,
            TLVValue::NumberOfMessages(v) => v.async_io_write(buf).await,
            TLVValue::PayloadType(v) => v.async_io_write(buf).await,
            TLVValue::PrivacyIndicator(v) => v.async_io_write(buf).await,
            TLVValue::QosTimeToLive(v) => v.async_io_write(buf).await,
            TLVValue::ReceiptedMessageId(v) => v.async_io_write(buf).await,
            TLVValue::SarMsgRefNum(v) => v.async_io_write(buf).await,
            TLVValue::SarSegmentSeqnum(v) => v.async_io_write(buf).await,
            TLVValue::SarTotalSegments(v) => v.async_io_write(buf).await,
            TLVValue::ScInterfaceVersion(v) => v.async_io_write(buf).await,
            TLVValue::SetDpf(v) => v.async_io_write(buf).await,
            TLVValue::SmsSignal(v) => v.async_io_write(buf).await,
            TLVValue::SourceAddrSubunit(v) => v.async_io_write(buf).await,
            TLVValue::SourceBearerType(v) => v.async_io_write(buf).await,
            TLVValue::SourceNetworkId(v) => v.async_io_write(buf).await,
            TLVValue::SourceNetworkType(v) => v.async_io_write(buf).await,
            TLVValue::SourceNodeId(v) => v.async_io_write(buf).await,
            TLVValue::SourcePort(v) => v.async_io_write(buf).await,
            TLVValue::SourceSubaddress(v) => v.async_io_write(buf).await,
            TLVValue::SourceTelematicsId(v) => v.async_io_write(buf).await,
            TLVValue::UserMessageReference(v) => v.async_io_write(buf).await,
            TLVValue::UserResponseCode(v) => v.async_io_write(buf).await,
            TLVValue::UssdServiceOp(v) => v.async_io_write(buf).await,
            TLVValue::Other { value, .. } => value.async_io_write(buf).await,
        }
    }
}

impl IoWrite for TLVValue {
    fn io_write(&self, buf: &mut IoWritable) -> std::io::Result<()> {
        match self {
            TLVValue::AdditionalStatusInfoText(v) => v.io_write(buf),
            TLVValue::AlertOnMessageDelivery(v) => v.io_write(buf),
            TLVValue::BillingIdentification(v) => v.io_write(buf),
            TLVValue::BroadcastAreaIdentifier(v) => v.io_write(buf),
            TLVValue::BroadcastAreaSuccess(v) => v.io_write(buf),
            TLVValue::BroadcastContentTypeInfo(v) => v.io_write(buf),
            TLVValue::BroadcastChannelIndicator(v) => v.io_write(buf),
            TLVValue::BroadcastContentType(v) => v.io_write(buf),
            TLVValue::BroadcastEndTime(v) => v.io_write(buf),
            TLVValue::BroadcastErrorStatus(v) => v.io_write(buf),
            TLVValue::BroadcastFrequencyInterval(v) => v.io_write(buf),
            TLVValue::BroadcastMessageClass(v) => v.io_write(buf),
            TLVValue::BroadcastRepNum(v) => v.io_write(buf),
            TLVValue::BroadcastServiceGroup(v) => v.io_write(buf),
            TLVValue::CallbackNum(v) => v.io_write(buf),
            TLVValue::CallbackNumAtag(v) => v.io_write(buf),
            TLVValue::CallbackNumPresInd(v) => v.io_write(buf),
            TLVValue::CongestionState(v) => v.io_write(buf),
            TLVValue::DeliveryFailureReason(v) => v.io_write(buf),
            TLVValue::DestAddrNpCountry(v) => v.io_write(buf),
            TLVValue::DestAddrNpInformation(v) => v.io_write(buf),
            TLVValue::DestAddrNpResolution(v) => v.io_write(buf),
            TLVValue::DestAddrSubunit(v) => v.io_write(buf),
            TLVValue::DestBearerType(v) => v.io_write(buf),
            TLVValue::DestNetworkId(v) => v.io_write(buf),
            TLVValue::DestNetworkType(v) => v.io_write(buf),
            TLVValue::DestNodeId(v) => v.io_write(buf),
            TLVValue::DestSubaddress(v) => v.io_write(buf),
            TLVValue::DestTelematicsId(v) => v.io_write(buf),
            TLVValue::DestPort(v) => v.io_write(buf),
            TLVValue::DisplayTime(v) => v.io_write(buf),
            TLVValue::DpfResult(v) => v.io_write(buf),
            TLVValue::ItsReplyType(v) => v.io_write(buf),
            TLVValue::ItsSessionInfo(v) => v.io_write(buf),
            TLVValue::LanguageIndicator(v) => v.io_write(buf),
            TLVValue::MessagePayload(v) => v.io_write(buf),
            TLVValue::MessageState(v) => v.io_write(buf),
            TLVValue::MoreMessagesToSend(v) => v.io_write(buf),
            TLVValue::MsAvailabilityStatus(v) => v.io_write(buf),
            TLVValue::MsMsgWaitFacilities(v) => v.io_write(buf),
            TLVValue::MsValidity(v) => v.io_write(buf),
            TLVValue::NetworkErrorCode(v) => v.io_write(buf),
            TLVValue::NumberOfMessages(v) => v.io_write(buf),
            TLVValue::PayloadType(v) => v.io_write(buf),
            TLVValue::PrivacyIndicator(v) => v.io_write(buf),
            TLVValue::QosTimeToLive(v) => v.io_write(buf),
            TLVValue::ReceiptedMessageId(v) => v.io_write(buf),
            TLVValue::SarMsgRefNum(v) => v.io_write(buf),
            TLVValue::SarSegmentSeqnum(v) => v.io_write(buf),
            TLVValue::SarTotalSegments(v) => v.io_write(buf),
            TLVValue::ScInterfaceVersion(v) => v.io_write(buf),
            TLVValue::SetDpf(v) => v.io_write(buf),
            TLVValue::SmsSignal(v) => v.io_write(buf),
            TLVValue::SourceAddrSubunit(v) => v.io_write(buf),
            TLVValue::SourceBearerType(v) => v.io_write(buf),
            TLVValue::SourceNetworkId(v) => v.io_write(buf),
            TLVValue::SourceNetworkType(v) => v.io_write(buf),
            TLVValue::SourceNodeId(v) => v.io_write(buf),
            TLVValue::SourcePort(v) => v.io_write(buf),
            TLVValue::SourceSubaddress(v) => v.io_write(buf),
            TLVValue::SourceTelematicsId(v) => v.io_write(buf),
            TLVValue::UserMessageReference(v) => v.io_write(buf),
            TLVValue::UserResponseCode(v) => v.io_write(buf),
            TLVValue::UssdServiceOp(v) => v.io_write(buf),
            TLVValue::Other { value, .. } => value.io_write(buf),
        }
    }
}
