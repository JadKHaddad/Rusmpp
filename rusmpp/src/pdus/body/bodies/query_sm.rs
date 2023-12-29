use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::{
    pdus::types::{npi::Npi, ton::Ton},
    types::c_octet_string::COctetString,
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct QuerySm {
    pub message_id: COctetString<1, 65>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 21>,
}

impl QuerySm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        }
    }
}
