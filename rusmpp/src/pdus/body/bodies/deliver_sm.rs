use super::s_sm::SSm;
use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::tlvs::tlv::{MessageDeliveryRequestTLV, TLV},
};
use derive_builder::Builder;
use getset::{Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Default,
    Getters,
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
pub struct DeliverSm {
    #[getset(get = "pub", set = "pub")]
    ssm: SSm,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(private, setter(name = "_tlvs"))]
    tlvs: Vec<TLV>,
}

impl DeliverSm {
    pub fn new(ssm: SSm, tlvs: Vec<MessageDeliveryRequestTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        let ssm = SSm::check_for_message_payload_and_update(ssm, &tlvs);

        Self { ssm, tlvs }
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageDeliveryRequestTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: MessageDeliveryRequestTLV) {
        self.tlvs.push(tlv.into());
    }
}

impl DeliverSmBuilder {
    pub fn tlvs(&mut self, tlvs: Vec<MessageDeliveryRequestTLV>) -> &mut Self {
        self.tlvs = Some(tlvs.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn push_tlv(&mut self, tlv: MessageDeliveryRequestTLV) -> &mut Self {
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
        defaut_write_read_with_length_compare::<DeliverSm>().await;
    }
}
