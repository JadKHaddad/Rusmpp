use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{npi::Npi, ton::Ton},
    },
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

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
pub struct QueryBroadcastSm {
    #[getset(get = "pub", set = "pub")]
    message_id: COctetString<1, 65>,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
    source_addr: COctetString<1, 21>,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
    user_message_reference: Option<TLV>,
}

impl QueryBroadcastSm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        user_message_reference: Option<u16>,
    ) -> Self {
        let user_message_reference =
            user_message_reference.map(|v| TLV::new(TLVValue::UserMessageReference(v)));

        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            user_message_reference,
        }
    }

    pub fn set_user_message_reference(&mut self, user_message_reference: Option<u16>) {
        self.user_message_reference =
            user_message_reference.map(|v| TLV::new(TLVValue::UserMessageReference(v)));
    }
}

impl QueryBroadcastSmBuilder {
    pub fn user_message_reference(&mut self, user_message_reference: Option<u16>) -> &mut Self {
        self.user_message_reference = user_message_reference
            .map(|v| TLV::new(TLVValue::UserMessageReference(v)))
            .into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<QueryBroadcastSm>().await;
    }
}
