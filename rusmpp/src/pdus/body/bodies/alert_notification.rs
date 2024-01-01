use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::tlv::TLV,
        types::{npi::Npi, ton::Ton},
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
pub struct AlertNotification {
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 65>,
    pub esme_addr_ton: Ton,
    pub esme_addr_npi: Npi,
    pub esme_addr: COctetString<1, 65>,
    #[rusmpp_io_read(length=(length - all_before))]
    pub ms_availability_status: Option<TLV>,
}

impl AlertNotification {
    pub fn new(
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 65>,
        esme_addr_ton: Ton,
        esme_addr_npi: Npi,
        esme_addr: COctetString<1, 65>,
        ms_availability_status: Option<TLV>,
    ) -> Self {
        Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusmpp_io::io::{read::AsyncIoReadWithLength, write::AsyncIoWrite};
    use std::{io::Cursor, str::FromStr};

    #[tokio::test]
    async fn write_read_compare() {
        let alert_notification = AlertNotification::new(
            Ton::International,
            Npi::Isdn,
            COctetString::from_str("source_addr").unwrap(),
            Ton::International,
            Npi::Isdn,
            COctetString::from_str("esme_addr").unwrap(),
            None,
        );

        let mut curser = Cursor::new(Vec::new());

        alert_notification
            .async_io_write(&mut curser)
            .await
            .expect("Failed to write bytes");

        curser.set_position(0);

        let alert_notification_read =
            AlertNotification::async_io_read(&mut curser, alert_notification.length())
                .await
                .expect("Failed to read bytes");

        assert_eq!(alert_notification, alert_notification_read);
    }
}
