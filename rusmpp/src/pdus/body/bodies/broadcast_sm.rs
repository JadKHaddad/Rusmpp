use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
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
    },
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
};
use derivative::Derivative;
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Derivative,
    Getters,
    CopyGetters,
    Setters,
    Builder,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoReadLength,
)]
#[derivative(Default)]
#[builder(default)]
pub struct BroadcastSm {
    #[getset(get = "pub", set = "pub")]
    serivce_type: ServiceType,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
    source_addr: COctetString<1, 21>,
    #[getset(get = "pub", set = "pub")]
    message_id: COctetString<1, 65>,
    #[getset(get = "pub", set = "pub")]
    priority_flag: PriorityFlag,
    #[getset(get = "pub", set = "pub")]
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    #[getset(get = "pub", set = "pub")]
    validity_period: EmptyOrFullCOctetString<17>,
    #[getset(get = "pub", set = "pub")]
    replace_if_present_flag: ReplaceIfPresentFlag,
    #[getset(get = "pub", set = "pub")]
    data_coding: DataCoding,
    #[getset(get_copy = "pub", set = "pub")]
    sm_default_msg_id: u8,
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    #[derivative(Default(value = "TLVValue::BroadcastAreaIdentifier(Default::default()).into()"))]
    broadcast_area_identifier: TLV,
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    #[derivative(Default(value = "TLVValue::BroadcastContentType(Default::default()).into()"))]
    broadcast_content_type: TLV,
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    #[derivative(Default(value = "TLVValue::BroadcastRepNum(Default::default()).into()"))]
    broadcast_rep_num: TLV,
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    #[derivative(Default(value = "TLVValue::BroadcastAreaIdentifier(Default::default()).into()"))]
    broadcast_frequency_interval: TLV,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
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

    pub fn set_tlvs(&mut self, tlvs: Vec<BroadcastRequestTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: BroadcastRequestTLV) {
        self.tlvs.push(tlv.into());
    }

    pub fn set_broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) {
        self.broadcast_area_identifier =
            TLV::new(TLVValue::BroadcastAreaIdentifier(broadcast_area_identifier));
    }

    pub fn set_broadcast_content_type(&mut self, broadcast_content_type: BroadcastContentType) {
        self.broadcast_content_type =
            TLV::new(TLVValue::BroadcastContentType(broadcast_content_type));
    }

    pub fn set_broadcast_rep_num(&mut self, broadcast_rep_num: u16) {
        self.broadcast_rep_num = TLV::new(TLVValue::BroadcastRepNum(broadcast_rep_num));
    }

    pub fn set_broadcast_frequency_interval(
        &mut self,
        broadcast_frequency_interval: BroadcastFrequencyInterval,
    ) {
        self.broadcast_frequency_interval = TLV::new(TLVValue::BroadcastFrequencyInterval(
            broadcast_frequency_interval,
        ));
    }
}

impl BroadcastSmBuilder {
    pub fn tlvs(&mut self, tlvs: Vec<BroadcastRequestTLV>) -> &mut Self {
        self.tlvs = Some(tlvs.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn push_tlv(&mut self, tlv: BroadcastRequestTLV) -> &mut Self {
        self.tlvs.get_or_insert_with(Vec::new).push(tlv.into());
        self
    }

    pub fn broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) -> &mut Self {
        self.broadcast_area_identifier = Some(TLV::new(TLVValue::BroadcastAreaIdentifier(
            broadcast_area_identifier,
        )));
        self
    }

    pub fn broadcast_content_type(
        &mut self,
        broadcast_content_type: BroadcastContentType,
    ) -> &mut Self {
        self.broadcast_content_type = Some(TLV::new(TLVValue::BroadcastContentType(
            broadcast_content_type,
        )));
        self
    }

    pub fn broadcast_rep_num(&mut self, broadcast_rep_num: u16) -> &mut Self {
        self.broadcast_rep_num = Some(TLV::new(TLVValue::BroadcastRepNum(broadcast_rep_num)));
        self
    }

    pub fn broadcast_frequency_interval(
        &mut self,
        broadcast_frequency_interval: BroadcastFrequencyInterval,
    ) -> &mut Self {
        self.broadcast_frequency_interval = Some(TLV::new(TLVValue::BroadcastFrequencyInterval(
            broadcast_frequency_interval,
        )));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<BroadcastSm>().await;
    }
}
