use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::{
    pdus::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
    types::c_octet_string::COctetString,
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
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
    use rusmpp_io::io::{read::AsyncIoRead, write::AsyncIoWrite};
    use std::{io::Cursor, str::FromStr};

    #[tokio::test]
    async fn write_read_compare() {
        let bind = Bind::new(
            COctetString::from_str("system_id").unwrap(),
            COctetString::from_str("password").unwrap(),
            COctetString::from_str("system_type").unwrap(),
            InterfaceVersion::Smpp5_0,
            Ton::International,
            Npi::Isdn,
            COctetString::from_str("address_range").unwrap(),
        );

        let mut curser = Cursor::new(Vec::new());

        bind.async_io_write(&mut curser)
            .await
            .expect("Failed to write bytes");

        curser.set_position(0);

        let bind_read = Bind::async_io_read(&mut curser)
            .await
            .expect("Failed to read bytes");

        assert_eq!(bind, bind_read);
    }
}
