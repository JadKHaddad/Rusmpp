use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::tlv::{MessageDeliveryRequestTLV, MessageSubmissionRequestTLV, TLV},
        types::{
            data_coding::DataCoding, esm_class::EsmClass, priority_flag::PriorityFlag,
            registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::ServiceType,
        },
    },
    prelude::{Npi, TLVTag, Ton},
};
use derive_builder::Builder;
use getset::{Getters, Setters};
use rusmpp_io::types::{
    c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
    octet_string::OctetString,
};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[duplicate::duplicate_item(
  mod_name          struct_name     struct_builder_name     tlv_type_name;
  [ submit_sm ]     [ SubmitSm ]    [ SubmitSmBuilder ]     [ MessageSubmissionRequestTLV ];
  [ deliver_sm ]    [ DeliverSm ]   [ DeliverSmBuilder ]    [ MessageDeliveryRequestTLV ];
)]
pub mod mod_name {
    use super::*;

    #[derive(
        Default,
        Getters,
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
    #[builder(default)]
    /// Short message in ssm is always superceded by the message payload.
    /// Clear message payload to use the short message.
    pub struct struct_name {
        pub serivce_type: ServiceType,
        pub source_addr_ton: Ton,
        pub source_addr_npi: Npi,
        pub source_addr: COctetString<1, 21>,
        pub dest_addr_ton: Ton,
        pub dest_addr_npi: Npi,
        pub destination_addr: COctetString<1, 21>,
        pub esm_class: EsmClass,
        pub protocol_id: u8,
        pub priority_flag: PriorityFlag,
        pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
        pub validity_period: EmptyOrFullCOctetString<17>,
        pub registered_delivery: RegisteredDelivery,
        pub replace_if_present_flag: ReplaceIfPresentFlag,
        pub data_coding: DataCoding,
        /// The sm_default_msg_id parameter specifies the MC index of a pre-defined (‘canned’)
        /// message.
        pub sm_default_msg_id: u8,
        #[builder(setter(custom))]
        sm_length: u8,
        #[rusmpp_io_read(length=(sm_length))]
        #[builder(setter(custom))]
        short_message: OctetString<0, 255>,
        #[rusmpp_io_read(length=(length - all_before))]
        #[builder(setter(custom))]
        tlvs: Vec<TLV>,
    }

    impl struct_name {
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
            tlvs: Vec<tlv_type_name>,
        ) -> Self {
            let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
            let sm_length = short_message.length() as u8;

            let mut deliver_sm = Self {
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
                tlvs,
            };

            deliver_sm.clear_short_message_if_message_payload_exists();

            deliver_sm
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

        /// Clears the short message and short message length if the message payload is set.
        /// Returns true if the short message and short message length were cleared.
        fn clear_short_message_if_message_payload_exists(&mut self) -> bool {
            let message_payload_exists = self
                .tlvs
                .iter()
                .any(|v| matches!(v.tag(), TLVTag::MessagePayload));

            if message_payload_exists {
                self.short_message = OctetString::empty();
                self.sm_length = 0;

                return true;
            };

            false
        }

        /// Sets the short message and short message length.
        /// Updates the short message and short message length accordingly.
        /// Has no effect if the message payload is set.
        /// Returns true if the short message and short message length were set.
        pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) -> bool {
            self.sm_length = short_message.length() as u8;
            self.short_message = short_message;

            !self.clear_short_message_if_message_payload_exists()
        }

        pub fn set_tlvs(&mut self, tlvs: Vec<tlv_type_name>) {
            let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
            self.tlvs = tlvs;
            self.clear_short_message_if_message_payload_exists();
        }

        pub fn push_tlv(&mut self, tlv: tlv_type_name) {
            let tlv = tlv.into();
            self.tlvs.push(tlv);
            self.clear_short_message_if_message_payload_exists();
        }
    }

    impl struct_builder_name {
        /// Clears the short message and short message length if the message payload is set.
        fn clear_short_message_if_message_payload_exists(&mut self) {
            if let Some(ref tlvs) = self.tlvs {
                let message_payload_exists = tlvs
                    .iter()
                    .any(|v| matches!(v.tag(), TLVTag::MessagePayload));

                if message_payload_exists {
                    self.short_message = None;
                    self.sm_length = None;
                };
            }
        }

        pub fn short_message(&mut self, short_message: OctetString<0, 255>) -> &mut Self {
            self.sm_length = Some(short_message.length() as u8);
            self.short_message = Some(short_message);
            self.clear_short_message_if_message_payload_exists();
            self
        }

        pub fn tlvs(&mut self, tlvs: Vec<tlv_type_name>) -> &mut Self {
            let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
            self.tlvs = Some(tlvs);
            self.clear_short_message_if_message_payload_exists();
            self
        }

        pub fn push_tlv(&mut self, tlv: tlv_type_name) -> &mut Self {
            let tlv = tlv.into();
            self.tlvs.get_or_insert_with(Vec::new).push(tlv);
            self.clear_short_message_if_message_payload_exists();
            self
        }
    }
}

pub use deliver_sm::*;
pub use submit_sm::*;
