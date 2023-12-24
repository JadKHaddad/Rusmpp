use crate::{
    io::{
        length::IoLength,
        read::{
            AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadWithLength, AsyncIoReadable,
            IoReadError,
        },
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::{command_status::CommandStatus, interface_version::InterfaceVersion},
    types::{
        c_octet_string::COctetString, no_fixed_size_octet_string::NoFixedSizeOctetString,
        octet_string::OctetString,
    },
};

use super::{
    tlv_tag::TLVTag,
    tlv_values::{
        alert_on_message_delivery::AlertOnMessageDelivery,
        broadcast_area_identifier::BroadcastAreaIdentifier,
        broadcast_area_success::BroadcastAreaSuccess,
        broadcast_channel_indicator::BroadcastChannelIndicator,
        broadcast_content_type::BroadcastContentType,
        broadcast_frequency_interval::BroadcastFrequencyInterval,
        broadcast_message_class::BroadcastMessageClass, callback_num_pres_ind::CallbackNumPresInd,
        congestion_state::CongestionState, delivery_failure_reason::DeliveryFailureReason,
        dest_addr_np_resolution::DestAddrNpResolution, dest_addr_subunit::DestAddrSubunit,
        dest_bearer_type::DestBearerType, dest_network_type::DestNetworkType,
        ms_availability_status::MsAvailabilityStatus, ms_msg_wait_facilities::MsMsgWaitFacilities,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TLVValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    AlertOnMessageDelivery(AlertOnMessageDelivery),
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
    DestAddrSubunit(DestAddrSubunit),
    DestBearerType(DestBearerType),
    DestNetworkId(COctetString<7, 66>),
    DestNetworkType(DestNetworkType),
    DestNodeId(OctetString<6, 6>),
    MsAvailabilityStatus(MsAvailabilityStatus),
    MsMsgWaitFacilities(MsMsgWaitFacilities),
    ScInterfaceVersion(InterfaceVersion),
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
            TLVValue::MsAvailabilityStatus(_) => TLVTag::MsAvailabilityStatus,
            TLVValue::MsMsgWaitFacilities(_) => TLVTag::MsMsgWaitFacilities,
            TLVValue::ScInterfaceVersion(_) => TLVTag::ScInterfaceVersion,
            TLVValue::Other { tag, .. } => *tag,
        }
    }
}

impl IoLength for TLVValue {
    fn length(&self) -> usize {
        match self {
            TLVValue::AdditionalStatusInfoText(v) => v.length(),
            TLVValue::AlertOnMessageDelivery(v) => v.length(),
            TLVValue::BillingIdentification(v) => v.length(),
            TLVValue::BroadcastAreaIdentifier(v) => v.length(),
            TLVValue::BroadcastAreaSuccess(v) => v.length(),
            TLVValue::BroadcastContentTypeInfo(v) => v.length(),
            TLVValue::BroadcastChannelIndicator(v) => v.length(),
            TLVValue::BroadcastContentType(v) => v.length(),
            TLVValue::BroadcastEndTime(v) => v.length(),
            TLVValue::BroadcastErrorStatus(v) => v.length(),
            TLVValue::BroadcastFrequencyInterval(v) => v.length(),
            TLVValue::BroadcastMessageClass(v) => v.length(),
            TLVValue::BroadcastRepNum(v) => v.length(),
            TLVValue::BroadcastServiceGroup(v) => v.length(),
            TLVValue::CallbackNum(v) => v.length(),
            TLVValue::CallbackNumAtag(v) => v.length(),
            TLVValue::CallbackNumPresInd(v) => v.length(),
            TLVValue::CongestionState(v) => v.length(),
            TLVValue::DeliveryFailureReason(v) => v.length(),
            TLVValue::DestAddrNpCountry(v) => v.length(),
            TLVValue::DestAddrNpInformation(v) => v.length(),
            TLVValue::DestAddrNpResolution(v) => v.length(),
            TLVValue::DestAddrSubunit(v) => v.length(),
            TLVValue::DestBearerType(v) => v.length(),
            TLVValue::DestNetworkId(v) => v.length(),
            TLVValue::DestNetworkType(v) => v.length(),
            TLVValue::DestNodeId(v) => v.length(),
            TLVValue::MsAvailabilityStatus(v) => v.length(),
            TLVValue::MsMsgWaitFacilities(v) => v.length(),
            TLVValue::ScInterfaceVersion(v) => v.length(),
            TLVValue::Other { value, .. } => value.length(),
        }
    }
}

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
            TLVValue::MsAvailabilityStatus(v) => v.async_io_write(buf).await,
            TLVValue::MsMsgWaitFacilities(v) => v.async_io_write(buf).await,
            TLVValue::ScInterfaceVersion(v) => v.async_io_write(buf).await,
            TLVValue::Other { value, .. } => value.async_io_write(buf).await,
        }
    }
}

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
                TLVValue::AlertOnMessageDelivery(AlertOnMessageDelivery::async_io_read(buf).await?)
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
                TLVValue::DestAddrSubunit(DestAddrSubunit::async_io_read(buf).await?)
            }
            TLVTag::DestBearerType => {
                TLVValue::DestBearerType(DestBearerType::async_io_read(buf).await?)
            }
            TLVTag::DestNetworkId => {
                TLVValue::DestNetworkId(COctetString::async_io_read(buf).await?)
            }
            TLVTag::DestNetworkType => {
                TLVValue::DestNetworkType(DestNetworkType::async_io_read(buf).await?)
            }
            TLVTag::DestNodeId => {
                TLVValue::DestNodeId(OctetString::async_io_read(buf, length).await?)
            }
            TLVTag::MsAvailabilityStatus => {
                TLVValue::MsAvailabilityStatus(MsAvailabilityStatus::async_io_read(buf).await?)
            }
            TLVTag::MsMsgWaitFacilities => {
                TLVValue::MsMsgWaitFacilities(MsMsgWaitFacilities::async_io_read(buf).await?)
            }
            TLVTag::ScInterfaceVersion => {
                TLVValue::ScInterfaceVersion(InterfaceVersion::async_io_read(buf).await?)
            }
            TLVTag::Other(_) => TLVValue::Other {
                tag: key,
                value: NoFixedSizeOctetString::async_io_read(buf, length).await?,
            },
            _ => return Err(IoReadError::UnsupportedKey { key: key.into() }),
        };

        Ok(Some(read))
    }
}
