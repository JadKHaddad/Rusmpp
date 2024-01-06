use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoReadWithLength, IoReadWithLength},
    },
    pdus::{
        tlvs::{tlv::TLV, tlv_tag::TLVTag},
        types::{
            data_coding::DataCoding, esm_class::EsmClass, npi::Npi, priority_flag::PriorityFlag,
            registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::ServiceType, ton::Ton,
        },
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        octet_string::OctetString,
    },
};
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

#[derive(
    Default,
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
    RusmppIoRead,
)]
#[builder(default)]
pub struct SSm {
    #[getset(get = "pub", set = "pub")]
    serivce_type: ServiceType,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
    source_addr: COctetString<1, 21>,
    #[getset(get_copy = "pub", set = "pub")]
    dest_addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    dest_addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
    destination_addr: COctetString<1, 21>,
    #[getset(get_copy = "pub", set = "pub")]
    esm_class: EsmClass,
    #[getset(get_copy = "pub", set = "pub")]
    protocol_id: u8,
    #[getset(get_copy = "pub", set = "pub")]
    priority_flag: PriorityFlag,
    #[getset(get = "pub", set = "pub")]
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    #[getset(get = "pub", set = "pub")]
    validity_period: EmptyOrFullCOctetString<17>,
    #[getset(get_copy = "pub", set = "pub")]
    registered_delivery: RegisteredDelivery,
    #[getset(get_copy = "pub", set = "pub")]
    replace_if_present_flag: ReplaceIfPresentFlag,
    #[getset(get_copy = "pub", set = "pub")]
    data_coding: DataCoding,
    /// The sm_default_msg_id parameter specifies the MC index of a pre-defined (‘canned’)
    /// message.
    #[getset(get_copy = "pub", set = "pub")]
    sm_default_msg_id: u8,
    #[getset(get_copy = "pub")]
    #[builder(setter(custom))]
    sm_length: u8,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(sm_length))]
    #[builder(setter(custom))]
    short_message: OctetString<0, 255>,
}

impl SSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
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
    ) -> Self {
        let sm_length = short_message.length() as u8;

        Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
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
        }
    }

    /// Clears the short message and short message length if the message payload is set.
    /// Returns true if the short message and short message length were cleared.
    pub(crate) fn check_for_message_payload_and_clear_short_message(
        &mut self,
        tlvs: &[TLV],
    ) -> bool {
        let message_payload_exists = tlvs
            .iter()
            .any(|v| matches!(v.tag(), TLVTag::MessagePayload));

        if message_payload_exists {
            self.short_message = OctetString::empty();
            self.sm_length = 0;

            return true;
        };

        false
    }

    pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) {
        self.sm_length = short_message.length() as u8;
        self.short_message = short_message;
    }
}

impl SSmBuilder {
    pub fn short_message(&mut self, short_message: OctetString<0, 255>) -> &mut Self {
        self.sm_length = Some(short_message.length() as u8);
        self.short_message = Some(short_message);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::test_utils::defaut_write_read_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_compare::<SSm>().await;
    }

    #[test]
    fn builder() {
        let s_sm = SSmBuilder::default().build().unwrap();
        assert_eq!(s_sm.sm_length(), 0);
        assert_eq!(s_sm.short_message(), &OctetString::empty());

        let s_sm = SSmBuilder::default()
            .short_message(OctetString::from_str("hello").unwrap())
            .build()
            .unwrap();

        assert_eq!(s_sm.sm_length(), 5);
        assert_eq!(
            s_sm.short_message(),
            &OctetString::from_str("hello").unwrap()
        );
    }
}
