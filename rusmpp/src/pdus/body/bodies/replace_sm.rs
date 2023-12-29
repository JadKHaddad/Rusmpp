use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{length::IoLength, read::AsyncIoRead},
    pdus::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{npi::Npi, registered_delivery::RegisteredDelivery, ton::Ton},
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        no_fixed_size_octet_string::NoFixedSizeOctetString, octet_string::OctetString,
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
pub struct ReplaceSm {
    message_id: COctetString<1, 65>,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    validity_period: EmptyOrFullCOctetString<17>,
    registered_delivery: RegisteredDelivery,
    sm_default_msg_id: u8,
    sm_length: u8,
    #[rusmpp_io_read(length=(sm_length))]
    short_message: OctetString<0, 255>,
    #[rusmpp_io_read(length=(length - all_before))]
    message_payload: Option<TLV>,
}

impl ReplaceSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        message_payload: Option<NoFixedSizeOctetString>,
    ) -> Self {
        let message_payload = message_payload.map(|v| TLV::new(TLVValue::MessagePayload(v)));

        let short_message = if message_payload.is_some() {
            OctetString::empty()
        } else {
            short_message
        };

        let sm_length = short_message.length() as u8;

        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            sm_length,
            short_message,
            message_payload,
        }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
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

    pub fn schedule_delivery_time(&self) -> &EmptyOrFullCOctetString<17> {
        &self.schedule_delivery_time
    }

    pub fn validity_period(&self) -> &EmptyOrFullCOctetString<17> {
        &self.validity_period
    }

    pub fn registered_delivery(&self) -> &RegisteredDelivery {
        &self.registered_delivery
    }

    pub fn sm_default_msg_id(&self) -> &u8 {
        &self.sm_default_msg_id
    }

    pub fn sm_length(&self) -> &u8 {
        &self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
    }

    pub fn message_payload(&self) -> Option<&TLV> {
        self.message_payload.as_ref()
    }

    #[allow(clippy::type_complexity)]
    pub fn into_parts(
        self,
    ) -> (
        COctetString<1, 65>,
        Ton,
        Npi,
        COctetString<1, 21>,
        EmptyOrFullCOctetString<17>,
        EmptyOrFullCOctetString<17>,
        RegisteredDelivery,
        u8,
        u8,
        OctetString<0, 255>,
        Option<TLV>,
    ) {
        (
            self.message_id,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.schedule_delivery_time,
            self.validity_period,
            self.registered_delivery,
            self.sm_default_msg_id,
            self.sm_length,
            self.short_message,
            self.message_payload,
        )
    }
}
