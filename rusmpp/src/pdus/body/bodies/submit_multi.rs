use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{length::IoLength, read::AsyncIoRead},
    pdus::{
        tlvs::{
            tlv::{MessageSubmissionRequestTLV, TLV},
            tlv_tag::TLVTag,
        },
        types::{
            data_coding::DataCoding, dest_address::DestAddress, esm_class::EsmClass, npi::Npi,
            priority_flag::PriorityFlag, registered_delivery::RegisteredDelivery,
            replace_if_present_flag::ReplaceIfPresentFlag, service_type::ServiceType, ton::Ton,
        },
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        octet_string::OctetString,
    },
};

#[derive(
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
pub struct SubmitMulti {
    serivce_type: ServiceType,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    number_of_dests: u8,
    #[rusmpp_io_read(count=number_of_dests)]
    dest_address: Vec<DestAddress>,
    esm_class: EsmClass,
    protocol_id: u8,
    priority_flag: PriorityFlag,
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    validity_period: EmptyOrFullCOctetString<17>,
    registered_delivery: RegisteredDelivery,
    replace_if_present_flag: ReplaceIfPresentFlag,
    data_coding: DataCoding,
    sm_default_msg_id: u8,
    sm_length: u8,
    #[rusmpp_io_read(length=(sm_length))]
    short_message: OctetString<0, 255>,
    #[rusmpp_io_read(length=(length - all_before))]
    tlvs: Vec<TLV>,
}

impl SubmitMulti {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_address: Vec<DestAddress>,
        esm_class: EsmClass,
        protocol_id: u8,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        tlvs: Vec<MessageSubmissionRequestTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        let number_of_dests = dest_address.len() as u8;

        let message_payload_exists = tlvs
            .iter()
            .any(|v| matches!(v.tag(), TLVTag::MessagePayload));

        let short_message = if message_payload_exists {
            OctetString::empty()
        } else {
            short_message
        };

        let sm_length = short_message.length() as u8;

        Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            number_of_dests,
            dest_address,
            esm_class,
            protocol_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            sm_length,
            short_message,
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

    pub fn number_of_dests(&self) -> u8 {
        self.number_of_dests
    }

    pub fn dest_address(&self) -> &[DestAddress] {
        &self.dest_address
    }

    pub fn esm_class(&self) -> &EsmClass {
        &self.esm_class
    }

    pub fn protocol_id(&self) -> u8 {
        self.protocol_id
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

    pub fn registered_delivery(&self) -> &RegisteredDelivery {
        &self.registered_delivery
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

    pub fn sm_length(&self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
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
        u8,
        Vec<DestAddress>,
        EsmClass,
        u8,
        PriorityFlag,
        EmptyOrFullCOctetString<17>,
        EmptyOrFullCOctetString<17>,
        RegisteredDelivery,
        ReplaceIfPresentFlag,
        DataCoding,
        u8,
        u8,
        OctetString<0, 255>,
        Vec<TLV>,
    ) {
        (
            self.serivce_type,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.number_of_dests,
            self.dest_address,
            self.esm_class,
            self.protocol_id,
            self.priority_flag,
            self.schedule_delivery_time,
            self.validity_period,
            self.registered_delivery,
            self.replace_if_present_flag,
            self.data_coding,
            self.sm_default_msg_id,
            self.sm_length,
            self.short_message,
            self.tlvs,
        )
    }
}
