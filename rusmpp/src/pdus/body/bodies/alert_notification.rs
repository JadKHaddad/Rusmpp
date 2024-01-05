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
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

// Default is okay to derive because ms_availability_status will be None.
// For the Builder, ms_availability_status setter will be private and have a nother name
// so that we implement the setter ourselves.
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
pub struct AlertNotification {
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
    source_addr: COctetString<1, 65>,
    #[getset(get_copy = "pub", set = "pub")]
    esme_addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    esme_addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
    esme_addr: COctetString<1, 65>,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
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

    pub fn set_ms_availability_status(
        &mut self,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) {
        self.ms_availability_status =
            ms_availability_status.map(|v| TLV::new(TLVValue::MsAvailabilityStatus(v)));
    }
}

impl AlertNotificationBuilder {
    pub fn ms_availability_status(
        &mut self,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) -> &mut Self {
        self.ms_availability_status =
            Some(ms_availability_status.map(|v| TLV::new(TLVValue::MsAvailabilityStatus(v))));
        self
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

    #[test]
    fn builder() {
        let alert_notification = AlertNotificationBuilder::default().build().unwrap();

        assert_eq!(alert_notification, AlertNotification::default());

        let alert_notification = AlertNotificationBuilder::default()
            .ms_availability_status(Some(MsAvailabilityStatus::Available))
            .build()
            .unwrap();

        assert_eq!(
            alert_notification
                .ms_availability_status()
                .as_ref()
                .unwrap()
                .value()
                .unwrap(),
            &TLVValue::MsAvailabilityStatus(MsAvailabilityStatus::Available)
        );
    }
}
