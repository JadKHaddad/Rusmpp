use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::tlv::{CancelBroadcastTLV, TLV},
        types::{npi::Npi, service_type::ServiceType, ton::Ton},
    },
    types::c_octet_string::COctetString,
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
pub struct CancelBroadcastSm {
    pub serivce_type: ServiceType,
    pub message_id: COctetString<1, 65>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 21>,
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
    tlvs: Vec<TLV>,
}

impl CancelBroadcastSm {
    pub fn new(
        serivce_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        tlvs: Vec<CancelBroadcastTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect();

        Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        }
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<CancelBroadcastTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: CancelBroadcastTLV) {
        self.tlvs.push(tlv.into());
    }
}

impl CancelBroadcastSmBuilder {
    pub fn tlvs(&mut self, tlvs: Vec<CancelBroadcastTLV>) -> &mut Self {
        self.tlvs = Some(tlvs.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn push_tlv(&mut self, tlv: CancelBroadcastTLV) -> &mut Self {
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
        defaut_write_read_with_length_compare::<CancelBroadcastSm>().await;
    }
}
