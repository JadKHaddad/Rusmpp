use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::tlv::{MessageSubmissionRequestTLV, TLV},
        types::{
            data_coding::DataCoding, esm_class::EsmClass, npi::Npi,
            registered_delivery::RegisteredDelivery, service_type::ServiceType, ton::Ton,
        },
    },
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

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
    RusmppIoReadLength,
)]
#[builder(default)]
pub struct DataSm {
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
    registered_delivery: RegisteredDelivery,
    #[getset(get_copy = "pub", set = "pub")]
    data_coding: DataCoding,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(private, setter(name = "_tlvs"))]
    tlvs: Vec<TLV>,
}

impl DataSm {
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
        registered_delivery: RegisteredDelivery,
        data_coding: DataCoding,
        tlvs: Vec<MessageSubmissionRequestTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            registered_delivery,
            data_coding,
            tlvs,
        }
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageSubmissionRequestTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionRequestTLV) {
        self.tlvs.push(tlv.into());
    }
}

impl DataSmBuilder {
    pub fn tlvs(&mut self, tlvs: Vec<MessageSubmissionRequestTLV>) -> &mut Self {
        self.tlvs = Some(tlvs.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionRequestTLV) -> &mut Self {
        self.tlvs.get_or_insert_with(Vec::new).push(tlv.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<DataSm>().await;
    }
}
