use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::types::c_octet_string::COctetString;

use super::{command_status::CommandStatus, npi::Npi, ton::Ton};

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
pub struct UnsuccessSme {
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: COctetString<1, 21>,
    pub error_status_code: CommandStatus,
}

impl UnsuccessSme {
    pub fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
        error_status_code: CommandStatus,
    ) -> Self {
        Self {
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            error_status_code,
        }
    }
}
