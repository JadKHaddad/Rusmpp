use rusmpp_macros::RusmppIo;

use crate::pdus::{
    tlvs::{
        tlv::{BroadcastRequestTLV, TLV},
        tlv_value::TLVValue,
        tlv_values::{
            broadcast_area_identifier::BroadcastAreaIdentifier,
            broadcast_content_type::BroadcastContentType,
            broadcast_frequency_interval::BroadcastFrequencyInterval,
        },
    },
    types::{
        data_coding::DataCoding, npi::Npi, priority_flag::PriorityFlag,
        replace_if_present_flag::ReplaceIfPresentFlag, service_type::ServiceType, ton::Ton,
    },
};

use rusmpp_io::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    },
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct BroadcastSm {
    serivce_type: ServiceType,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    message_id: COctetString<1, 65>,
    priority_flag: PriorityFlag,
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    validity_period: EmptyOrFullCOctetString<17>,
    replace_if_present_flag: ReplaceIfPresentFlag,
    data_coding: DataCoding,
    sm_default_msg_id: u8,
    broadcast_area_identifier: TLV,
    broadcast_content_type: TLV,
    broadcast_rep_num: TLV,
    broadcast_frequency_interval: TLV,
    tlvs: Vec<TLV>,
}

impl BroadcastSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        message_id: COctetString<1, 65>,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        broadcast_area_identifier: BroadcastAreaIdentifier,
        broadcast_content_type: BroadcastContentType,
        broadcast_rep_num: u16,
        broadcast_frequency_interval: BroadcastFrequencyInterval,
        tlvs: Vec<BroadcastRequestTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();

        let broadcast_area_identifier =
            TLV::new(TLVValue::BroadcastAreaIdentifier(broadcast_area_identifier));
        let broadcast_content_type =
            TLV::new(TLVValue::BroadcastContentType(broadcast_content_type));
        let broadcast_rep_num = TLV::new(TLVValue::BroadcastRepNum(broadcast_rep_num));
        let broadcast_frequency_interval = TLV::new(TLVValue::BroadcastFrequencyInterval(
            broadcast_frequency_interval,
        ));

        Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            message_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            broadcast_area_identifier,
            broadcast_content_type,
            broadcast_rep_num,
            broadcast_frequency_interval,
            tlvs,
        }
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.serivce_type
    }

    pub fn source_addr_ton(&self) -> &Ton {
        &self.source_addr_ton
    }

    pub fn source_addr_npi(&self) -> &Npi {
        &self.source_addr_npi
    }

    pub fn source_addr(&self) -> &COctetString<1, 21> {
        &self.source_addr
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn priority_flag(&self) -> &PriorityFlag {
        &self.priority_flag
    }

    pub fn schedule_delivery_time(&self) -> &EmptyOrFullCOctetString<17> {
        &self.schedule_delivery_time
    }

    pub fn validity_period(&self) -> &EmptyOrFullCOctetString<17> {
        &self.validity_period
    }

    pub fn replace_if_present_flag(&self) -> &ReplaceIfPresentFlag {
        &self.replace_if_present_flag
    }

    pub fn data_coding(&self) -> &DataCoding {
        &self.data_coding
    }

    pub fn sm_default_msg_id(&self) -> u8 {
        self.sm_default_msg_id
    }

    pub fn broadcast_area_identifier(&self) -> &TLV {
        &self.broadcast_area_identifier
    }

    pub fn broadcast_content_type(&self) -> &TLV {
        &self.broadcast_content_type
    }

    pub fn broadcast_rep_num(&self) -> &TLV {
        &self.broadcast_rep_num
    }

    pub fn broadcast_frequency_interval(&self) -> &TLV {
        &self.broadcast_frequency_interval
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    #[allow(clippy::type_complexity)]
    pub fn into_parts(
        self,
    ) -> (
        ServiceType,
        Ton,
        Npi,
        COctetString<1, 21>,
        COctetString<1, 65>,
        PriorityFlag,
        EmptyOrFullCOctetString<17>,
        EmptyOrFullCOctetString<17>,
        ReplaceIfPresentFlag,
        DataCoding,
        u8,
        TLV,
        TLV,
        TLV,
        TLV,
        Vec<TLV>,
    ) {
        (
            self.serivce_type,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.message_id,
            self.priority_flag,
            self.schedule_delivery_time,
            self.validity_period,
            self.replace_if_present_flag,
            self.data_coding,
            self.sm_default_msg_id,
            self.broadcast_area_identifier,
            self.broadcast_content_type,
            self.broadcast_rep_num,
            self.broadcast_frequency_interval,
            self.tlvs,
        )
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for BroadcastSm {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let serivce_type = ServiceType::async_io_read(buf).await?;
        let source_addr_ton = Ton::async_io_read(buf).await?;
        let source_addr_npi = Npi::async_io_read(buf).await?;
        let source_addr = COctetString::async_io_read(buf).await?;
        let message_id = COctetString::async_io_read(buf).await?;
        let priority_flag = PriorityFlag::async_io_read(buf).await?;
        let schedule_delivery_time = EmptyOrFullCOctetString::async_io_read(buf).await?;
        let validity_period = EmptyOrFullCOctetString::async_io_read(buf).await?;
        let replace_if_present_flag = ReplaceIfPresentFlag::async_io_read(buf).await?;
        let data_coding = DataCoding::async_io_read(buf).await?;
        let sm_default_msg_id = u8::async_io_read(buf).await?;
        let broadcast_area_identifier = TLV::async_io_read(buf).await?;
        let broadcast_content_type = TLV::async_io_read(buf).await?;
        let broadcast_rep_num = TLV::async_io_read(buf).await?;
        let broadcast_frequency_interval = TLV::async_io_read(buf).await?;

        let tlvs_expected_len = length
            .saturating_sub(serivce_type.length())
            .saturating_sub(source_addr_ton.length())
            .saturating_sub(source_addr_npi.length())
            .saturating_sub(source_addr.length())
            .saturating_sub(message_id.length())
            .saturating_sub(priority_flag.length())
            .saturating_sub(schedule_delivery_time.length())
            .saturating_sub(validity_period.length())
            .saturating_sub(replace_if_present_flag.length())
            .saturating_sub(data_coding.length())
            .saturating_sub(sm_default_msg_id.length())
            .saturating_sub(broadcast_area_identifier.length())
            .saturating_sub(broadcast_content_type.length())
            .saturating_sub(broadcast_rep_num.length())
            .saturating_sub(broadcast_frequency_interval.length());

        let tlvs = Vec::<TLV>::async_io_read(buf, tlvs_expected_len).await?;

        Ok(Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            message_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            broadcast_area_identifier,
            broadcast_content_type,
            broadcast_rep_num,
            broadcast_frequency_interval,
            tlvs,
        })
    }
}
