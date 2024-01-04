use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::{
    pdus::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
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
pub struct Bind {
    pub system_id: COctetString<1, 16>,
    pub password: COctetString<1, 9>,
    pub system_type: COctetString<1, 13>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: COctetString<1, 41>,
}

impl Bind {
    pub fn new(
        system_id: COctetString<1, 16>,
        password: COctetString<1, 9>,
        system_type: COctetString<1, 13>,
        interface_version: InterfaceVersion,
        addr_ton: Ton,
        addr_npi: Npi,
        address_range: COctetString<1, 41>,
    ) -> Self {
        Self {
            system_id,
            password,
            system_type,
            interface_version,
            addr_ton,
            addr_npi,
            address_range,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_compare::<Bind>().await;
    }
}
