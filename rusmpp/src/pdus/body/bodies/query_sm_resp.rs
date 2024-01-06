use crate::{
    pdus::tlvs::tlv_values::message_state::MessageState,
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
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
pub struct QuerySmResp {
    #[getset(get = "pub", set = "pub")]
    pub message_id: COctetString<1, 65>,
    #[getset(get = "pub", set = "pub")]
    pub final_date: EmptyOrFullCOctetString<17>,
    #[getset(get_copy = "pub", set = "pub")]
    pub message_state: MessageState,
    #[getset(get_copy = "pub", set = "pub")]
    pub error_code: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_compare::<QuerySmResp>().await;
    }
}
