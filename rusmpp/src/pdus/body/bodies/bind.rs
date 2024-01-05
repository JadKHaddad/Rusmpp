use crate::{
    pdus::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use derive_new::new;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

#[derive(
    new,
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
    RusmppIoRead,
)]
#[builder(default)]
pub struct Bind {
    #[getset(get = "pub", set = "pub")]
    pub system_id: COctetString<1, 16>,
    #[getset(get = "pub", set = "pub")]
    pub password: COctetString<1, 9>,
    #[getset(get = "pub", set = "pub")]
    pub system_type: COctetString<1, 13>,
    #[getset(get = "pub", set = "pub")]
    pub interface_version: InterfaceVersion,
    #[getset(get_copy = "pub", set = "pub")]
    pub addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    pub addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
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
