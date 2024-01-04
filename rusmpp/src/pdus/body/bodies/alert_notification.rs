use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::{tlv::TLV, tlv_values::ms_availability_status::MsAvailabilityStatus},
        types::{npi::Npi, ton::Ton},
    },
    prelude::TLVValue,
    types::c_octet_string::COctetString,
};

#[derive(
    Default,
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
pub struct AlertNotification {
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 65>,
    esme_addr_ton: Ton,
    esme_addr_npi: Npi,
    esme_addr: COctetString<1, 65>,
    #[rusmpp_io_read(length=(length - all_before))]
    ms_availability_status: Option<TLV>,
}

impl AlertNotification {
    pub fn new(
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 65>,
        esme_addr_ton: Ton,
        esme_addr_npi: Npi,
        esme_addr: COctetString<1, 65>,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) -> Self {
        Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status: ms_availability_status
                .map(|v| TLV::new(TLVValue::MsAvailabilityStatus(v))),
        }
    }

    pub fn source_addr_ton(&self) -> &Ton {
        &self.source_addr_ton
    }

    pub fn source_addr_npi(&self) -> &Npi {
        &self.source_addr_npi
    }

    pub fn source_addr(&self) -> &COctetString<1, 65> {
        &self.source_addr
    }

    pub fn esme_addr_ton(&self) -> &Ton {
        &self.esme_addr_ton
    }

    pub fn esme_addr_npi(&self) -> &Npi {
        &self.esme_addr_npi
    }

    pub fn esme_addr(&self) -> &COctetString<1, 65> {
        &self.esme_addr
    }

    pub fn ms_availability_status(&self) -> Option<&TLV> {
        self.ms_availability_status.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<AlertNotification>().await;
    }
}
