use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

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
pub struct DataSm {
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
    #[rusmpp_io_read(length=(length - all_before))]
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

    pub fn dest_addr_ton(&self) -> &Ton {
        &self.dest_addr_ton
    }

    pub fn dest_addr_npi(&self) -> &Npi {
        &self.dest_addr_npi
    }

    pub fn destination_addr(&self) -> &COctetString<1, 21> {
        &self.destination_addr
    }

    pub fn esm_class(&self) -> &EsmClass {
        &self.esm_class
    }

    pub fn registered_delivery(&self) -> &RegisteredDelivery {
        &self.registered_delivery
    }

    pub fn data_coding(&self) -> &DataCoding {
        &self.data_coding
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
        Ton,
        Npi,
        COctetString<1, 21>,
        EsmClass,
        RegisteredDelivery,
        DataCoding,
        Vec<TLV>,
    ) {
        (
            self.serivce_type,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.dest_addr_ton,
            self.dest_addr_npi,
            self.destination_addr,
            self.esm_class,
            self.registered_delivery,
            self.data_coding,
            self.tlvs,
        )
    }
}
