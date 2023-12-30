use super::{
    super::tlv_values::{
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
    TLVValue,
};
use crate::pdus::{
    tlvs::tlv_tag::TLVTag,
    types::{command_status::CommandStatus, interface_version::InterfaceVersion},
};
use rusmpp_io::{
    io::read::{
        AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadWithLength, AsyncIoReadable,
        IoReadError,
    },
    types::{
        c_octet_string::COctetString, no_fixed_size_octet_string::NoFixedSizeOctetString,
        octet_string::OctetString,
    },
};

#[async_trait::async_trait]
impl AsyncIoReadWithKeyOptional for TLVValue {
    type Key = TLVTag;

    async fn async_io_read(
        key: Self::Key,
        buf: &mut AsyncIoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError> {
        let read = match key {
            TLVTag::AdditionalStatusInfoText => {
                TLVValue::AdditionalStatusInfoText(COctetString::async_io_read(buf).await?)
            }
            TLVTag::AlertOnMessageDelivery => {
                TLVValue::AlertOnMessageDelivery(AlertOnMsgDelivery::async_io_read(buf).await?)
            }
            TLVTag::BillingIdentification => {
                TLVValue::BillingIdentification(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::BroadcastAreaIdentifier => TLVValue::BroadcastAreaIdentifier(
                BroadcastAreaIdentifier::async_io_read(buf, length).await?,
            ),
            TLVTag::BroadcastAreaSuccess => {
                TLVValue::BroadcastAreaSuccess(BroadcastAreaSuccess::async_io_read(buf).await?)
            }
            TLVTag::BroadcastContentTypeInfo => {
                TLVValue::BroadcastContentTypeInfo(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::BroadcastChannelIndicator => TLVValue::BroadcastChannelIndicator(
                BroadcastChannelIndicator::async_io_read(buf).await?,
            ),
            TLVTag::BroadcastContentType => {
                TLVValue::BroadcastContentType(BroadcastContentType::async_io_read(buf).await?)
            }
            TLVTag::BroadcastEndTime => {
                TLVValue::BroadcastEndTime(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::BroadcastErrorStatus => {
                TLVValue::BroadcastErrorStatus(CommandStatus::async_io_read(buf).await?)
            }
            TLVTag::BroadcastFrequencyInterval => TLVValue::BroadcastFrequencyInterval(
                BroadcastFrequencyInterval::async_io_read(buf).await?,
            ),
            TLVTag::BroadcastMessageClass => {
                TLVValue::BroadcastMessageClass(BroadcastMessageClass::async_io_read(buf).await?)
            }
            TLVTag::BroadcastRepNum => TLVValue::BroadcastRepNum(u16::async_io_read(buf).await?),
            TLVTag::BroadcastServiceGroup => {
                TLVValue::BroadcastServiceGroup(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::CallbackNum => {
                TLVValue::CallbackNum(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::CallbackNumAtag => {
                TLVValue::CallbackNumAtag(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::CallbackNumPresInd => {
                TLVValue::CallbackNumPresInd(CallbackNumPresInd::async_io_read(buf).await?)
            }
            TLVTag::CongestionState => {
                TLVValue::CongestionState(CongestionState::async_io_read(buf).await?)
            }
            TLVTag::DeliveryFailureReason => {
                TLVValue::DeliveryFailureReason(DeliveryFailureReason::async_io_read(buf).await?)
            }
            TLVTag::DestAddrNpCountry => {
                TLVValue::DestAddrNpCountry(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::DestAddrNpInformation => {
                TLVValue::DestAddrNpInformation(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::DestAddrNpResolution => {
                TLVValue::DestAddrNpResolution(DestAddrNpResolution::async_io_read(buf).await?)
            }
            TLVTag::DestAddrSubunit => {
                TLVValue::DestAddrSubunit(AddrSubunit::async_io_read(buf).await?)
            }
            TLVTag::DestBearerType => {
                TLVValue::DestBearerType(BearerType::async_io_read(buf).await?)
            }
            TLVTag::DestNetworkId => {
                TLVValue::DestNetworkId(COctetString::async_io_read(buf).await?)
            }
            TLVTag::DestNetworkType => {
                TLVValue::DestNetworkType(NetworkType::async_io_read(buf).await?)
            }
            TLVTag::DestNodeId => {
                TLVValue::DestNodeId(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::DestSubaddress => {
                TLVValue::DestSubaddress(Subaddress::async_io_read(buf, length).await?)
            }
            TLVTag::DestTelematicsId => TLVValue::DestTelematicsId(u16::async_io_read(buf).await?),
            TLVTag::DestPort => TLVValue::DestPort(u16::async_io_read(buf).await?),
            TLVTag::DisplayTime => TLVValue::DisplayTime(DisplayTime::async_io_read(buf).await?),
            TLVTag::DpfResult => TLVValue::DpfResult(DpfResult::async_io_read(buf).await?),
            TLVTag::ItsReplyType => TLVValue::ItsReplyType(ItsReplyType::async_io_read(buf).await?),
            TLVTag::ItsSessionInfo => {
                TLVValue::ItsSessionInfo(ItsSessionInfo::async_io_read(buf).await?)
            }
            TLVTag::LanguageIndicator => {
                TLVValue::LanguageIndicator(LanguageIndicator::async_io_read(buf).await?)
            }
            TLVTag::MessagePayload => {
                TLVValue::MessagePayload(NoFixedSizeOctetString::async_io_read(buf, length).await?)
            }
            TLVTag::MessageState => TLVValue::MessageState(MessageState::async_io_read(buf).await?),
            TLVTag::MoreMessagesToSend => {
                TLVValue::MoreMessagesToSend(MoreMessagesToSend::async_io_read(buf).await?)
            }
            TLVTag::MsAvailabilityStatus => {
                TLVValue::MsAvailabilityStatus(MsAvailabilityStatus::async_io_read(buf).await?)
            }
            TLVTag::MsMsgWaitFacilities => {
                TLVValue::MsMsgWaitFacilities(MsMsgWaitFacilities::async_io_read(buf).await?)
            }
            TLVTag::MsValidity => {
                TLVValue::MsValidity(MsValidity::async_io_read(buf, length).await?)
            }
            TLVTag::NetworkErrorCode => {
                TLVValue::NetworkErrorCode(NetworkErrorCode::async_io_read(buf).await?)
            }
            TLVTag::NumberOfMessages => {
                TLVValue::NumberOfMessages(NumberOfMessages::async_io_read(buf).await?)
            }
            TLVTag::PayloadType => TLVValue::PayloadType(PayloadType::async_io_read(buf).await?),
            TLVTag::PrivacyIndicator => {
                TLVValue::PrivacyIndicator(PrivacyIndicator::async_io_read(buf).await?)
            }
            TLVTag::QosTimeToLive => TLVValue::QosTimeToLive(u32::async_io_read(buf).await?),
            TLVTag::ReceiptedMessageId => {
                TLVValue::ReceiptedMessageId(COctetString::async_io_read(buf).await?)
            }
            TLVTag::SarMsgRefNum => TLVValue::SarMsgRefNum(u16::async_io_read(buf).await?),
            TLVTag::SarSegmentSeqnum => TLVValue::SarSegmentSeqnum(u8::async_io_read(buf).await?),
            TLVTag::SarTotalSegments => TLVValue::SarTotalSegments(u8::async_io_read(buf).await?),
            TLVTag::ScInterfaceVersion => {
                TLVValue::ScInterfaceVersion(InterfaceVersion::async_io_read(buf).await?)
            }
            TLVTag::SetDpf => TLVValue::SetDpf(SetDpf::async_io_read(buf).await?),
            TLVTag::SmsSignal => TLVValue::SmsSignal(u16::async_io_read(buf).await?),
            TLVTag::SourceAddrSubunit => {
                TLVValue::SourceAddrSubunit(AddrSubunit::async_io_read(buf).await?)
            }
            TLVTag::SourceBearerType => {
                TLVValue::SourceBearerType(BearerType::async_io_read(buf).await?)
            }
            TLVTag::SourceNetworkId => {
                TLVValue::SourceNetworkId(COctetString::async_io_read(buf).await?)
            }
            TLVTag::SourceNetworkType => {
                TLVValue::SourceNetworkType(NetworkType::async_io_read(buf).await?)
            }
            TLVTag::SourceNodeId => {
                TLVValue::SourceNodeId(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::SourcePort => TLVValue::SourcePort(u16::async_io_read(buf).await?),
            TLVTag::SourceSubaddress => {
                TLVValue::SourceSubaddress(Subaddress::async_io_read(buf, length).await?)
            }
            TLVTag::SourceTelematicsId => {
                TLVValue::SourceTelematicsId(u16::async_io_read(buf).await?)
            }
            TLVTag::UserMessageReference => {
                TLVValue::UserMessageReference(u16::async_io_read(buf).await?)
            }
            TLVTag::UserResponseCode => TLVValue::UserResponseCode(u8::async_io_read(buf).await?),
            TLVTag::UssdServiceOp => {
                TLVValue::UssdServiceOp(UssdServiceOp::async_io_read(buf).await?)
            }
            TLVTag::Other(_) => TLVValue::Other {
                tag: key,
                value: NoFixedSizeOctetString::async_io_read(buf, length).await?,
            },
        };

        Ok(Some(read))
    }
}
