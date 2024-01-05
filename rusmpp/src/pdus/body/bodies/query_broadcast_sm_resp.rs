use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::tlvs::{
        tlv::{QueryBroadcastResponseTLV, TLV},
        tlv_value::TLVValue,
        tlv_values::{
            broadcast_area_identifier::BroadcastAreaIdentifier,
            broadcast_area_success::BroadcastAreaSuccess, message_state::MessageState,
        },
    },
    types::c_octet_string::COctetString,
};
use derivative::Derivative;
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Derivative,
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
#[derivative(Default)]
#[builder(default)]
pub struct QueryBroadcastSmResp {
    #[getset(get = "pub", set = "pub")]
    message_id: COctetString<1, 65>,
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    #[derivative(Default(value = "TLVValue::MessageState(Default::default()).into()"))]
    message_state: TLV,
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    #[derivative(Default(value = "TLVValue::BroadcastAreaIdentifier(Default::default()).into()"))]
    broadcast_area_identifier: TLV,
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    #[derivative(Default(value = "TLVValue::BroadcastAreaSuccess(Default::default()).into()"))]
    broadcast_area_success: TLV,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
    tlvs: Vec<TLV>,
}

impl QueryBroadcastSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        message_state: MessageState,
        broadcast_area_identifier: BroadcastAreaIdentifier,
        broadcast_area_success: BroadcastAreaSuccess,
        tlvs: Vec<QueryBroadcastResponseTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();

        let message_state = TLV::new(TLVValue::MessageState(message_state));
        let broadcast_area_identifier =
            TLV::new(TLVValue::BroadcastAreaIdentifier(broadcast_area_identifier));
        let broadcast_area_success =
            TLV::new(TLVValue::BroadcastAreaSuccess(broadcast_area_success));

        Self {
            message_id,
            message_state,
            broadcast_area_identifier,
            broadcast_area_success,
            tlvs,
        }
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<QueryBroadcastResponseTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: QueryBroadcastResponseTLV) {
        self.tlvs.push(tlv.into());
    }

    pub fn set_message_state(&mut self, message_state: MessageState) {
        self.message_state = TLV::new(TLVValue::MessageState(message_state));
    }

    pub fn set_broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) {
        self.broadcast_area_identifier =
            TLV::new(TLVValue::BroadcastAreaIdentifier(broadcast_area_identifier));
    }

    pub fn set_broadcast_area_success(&mut self, broadcast_area_success: BroadcastAreaSuccess) {
        self.broadcast_area_success =
            TLV::new(TLVValue::BroadcastAreaSuccess(broadcast_area_success));
    }
}

impl QueryBroadcastSmRespBuilder {
    pub fn tlvs(&mut self, tlvs: Vec<QueryBroadcastResponseTLV>) -> &mut Self {
        self.tlvs = Some(tlvs.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn push_tlv(&mut self, tlv: QueryBroadcastResponseTLV) -> &mut Self {
        self.tlvs.get_or_insert_with(Vec::new).push(tlv.into());
        self
    }

    pub fn message_state(&mut self, message_state: MessageState) -> &mut Self {
        self.message_state = Some(TLV::new(TLVValue::MessageState(message_state)));
        self
    }

    pub fn broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) -> &mut Self {
        self.broadcast_area_identifier = Some(TLV::new(TLVValue::BroadcastAreaIdentifier(
            broadcast_area_identifier,
        )));
        self
    }

    pub fn broadcast_area_success(
        &mut self,
        broadcast_area_success: BroadcastAreaSuccess,
    ) -> &mut Self {
        self.broadcast_area_success = Some(TLV::new(TLVValue::BroadcastAreaSuccess(
            broadcast_area_success,
        )));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<QueryBroadcastSmResp>().await;
    }
}
