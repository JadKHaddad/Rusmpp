use crate::{
    pdus::types::{npi::Npi, service_type::ServiceType, ton::Ton},
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use derive_new::new;
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

#[allow(clippy::too_many_arguments)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_compare::<CancelSm>().await;
    }
}
