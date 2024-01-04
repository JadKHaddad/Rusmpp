use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::tlvs::tlv::{BroadcastResponseTLV, TLV},
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
    RusmppIoReadLength,
)]
pub struct BroadcastSmResp {
    message_id: COctetString<1, 65>,
    #[rusmpp_io_read(length=(length - all_before))]
    tlvs: Vec<TLV>,
}

impl BroadcastSmResp {
    pub fn new(message_id: COctetString<1, 65>, tlvs: Vec<BroadcastResponseTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect();

        Self { message_id, tlvs }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (COctetString<1, 65>, Vec<TLV>) {
        (self.message_id, self.tlvs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<BroadcastSmResp>().await;
    }
}
