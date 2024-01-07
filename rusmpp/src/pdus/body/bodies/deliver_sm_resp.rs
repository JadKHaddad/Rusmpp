use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::tlvs::tlv::{MessageDeliveryResponseTLV, TLV},
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
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
    RusmppIoReadLength,
)]
#[builder(default)]
pub struct DeliverSmResp {
    pub message_id: COctetString<1, 65>,
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
    tlvs: Vec<TLV>,
}

impl DeliverSmResp {
    pub fn new(message_id: COctetString<1, 65>, tlvs: Vec<MessageDeliveryResponseTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect();

        Self { message_id, tlvs }
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageDeliveryResponseTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: MessageDeliveryResponseTLV) {
        self.tlvs.push(tlv.into());
    }
}

impl DeliverSmRespBuilder {
    pub fn tlvs(&mut self, tlvs: Vec<MessageDeliveryResponseTLV>) -> &mut Self {
        self.tlvs = Some(tlvs.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn push_tlv(&mut self, tlv: MessageDeliveryResponseTLV) -> &mut Self {
        self.tlvs.get_or_insert_with(Vec::new).push(tlv.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<DeliverSmResp>().await;
    }
}
