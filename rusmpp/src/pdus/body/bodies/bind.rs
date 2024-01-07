use crate::{
    pdus::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use derive_new::new;
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

#[derive(
    new,
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
    RusmppIoRead,
)]
#[builder(default)]
pub struct Bind {
    pub system_id: COctetString<1, 16>,
    pub password: COctetString<1, 9>,
    pub system_type: COctetString<1, 13>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: COctetString<1, 41>,
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
