use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
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
use derive_builder::Builder;
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Default,
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
pub struct SubmitMulti {
    pub serivce_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 21>,
    #[builder(setter(custom))]
    number_of_dests: u8,
    #[builder(setter(custom))]
    #[rusmpp_io_read(count=number_of_dests)]
    dest_address: Vec<DestAddress>,
    pub esm_class: EsmClass,
    pub protocol_id: u8,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
    pub validity_period: EmptyOrFullCOctetString<17>,
    pub registered_delivery: RegisteredDelivery,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
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
        let sm_length = short_message.length() as u8;

        let mut submit_mutli = Self {
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
        };

        submit_mutli.clear_short_message_if_message_payload_exists();

        submit_mutli
    }

    pub fn number_of_dests(&self) -> u8 {
        self.number_of_dests
    }

    pub fn dest_address(&self) -> &[DestAddress] {
        &self.dest_address
    }

    pub fn set_dest_address(&mut self, dest_address: Vec<DestAddress>) {
        self.number_of_dests = dest_address.len() as u8;
        self.dest_address = dest_address;
    }

    pub fn push_dest_address(&mut self, dest_address: DestAddress) {
        self.dest_address.push(dest_address);
        self.number_of_dests = self.dest_address.len() as u8;
    }

    pub fn sm_length(&self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
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

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageSubmissionRequestTLV>) {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        self.tlvs = tlvs;
        self.clear_short_message_if_message_payload_exists();
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionRequestTLV) {
        let tlv = tlv.into();
        self.tlvs.push(tlv);
        self.clear_short_message_if_message_payload_exists();
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
}

impl SubmitMultiBuilder {
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

    pub fn number_of_dests(&mut self, number_of_dests: u8) -> &mut Self {
        self.number_of_dests = Some(number_of_dests);
        self
    }

    pub fn dest_address(&mut self, dest_address: Vec<DestAddress>) -> &mut Self {
        self.number_of_dests = Some(dest_address.len() as u8);
        self.dest_address = Some(dest_address);
        self
    }

    pub fn push_dest_address(&mut self, dest_address: DestAddress) -> &mut Self {
        let self_dest_address = self.dest_address.get_or_insert_with(Vec::new);
        self_dest_address.push(dest_address);
        self.number_of_dests = Some(self_dest_address.len() as u8);

        self
    }

    pub fn short_message(&mut self, short_message: OctetString<0, 255>) -> &mut Self {
        self.sm_length = Some(short_message.length() as u8);
        self.short_message = Some(short_message);
        self.clear_short_message_if_message_payload_exists();
        self
    }

    pub fn tlvs(&mut self, tlvs: Vec<MessageSubmissionRequestTLV>) -> &mut Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        self.tlvs = Some(tlvs);
        self.clear_short_message_if_message_payload_exists();
        self
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionRequestTLV) -> &mut Self {
        let tlv = tlv.into();
        self.tlvs.get_or_insert_with(Vec::new).push(tlv);
        self.clear_short_message_if_message_payload_exists();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<SubmitMulti>().await;
    }
}
