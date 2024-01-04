use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::{
    pdus::types::{npi::Npi, service_type::ServiceType, ton::Ton},
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
    RusmppIoRead,
)]
pub struct CancelSm {
    pub serivce_type: ServiceType,
    pub message_id: COctetString<1, 65>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 21>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: COctetString<1, 21>,
}

impl CancelSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_compare::<CancelSm>().await;
    }
}
