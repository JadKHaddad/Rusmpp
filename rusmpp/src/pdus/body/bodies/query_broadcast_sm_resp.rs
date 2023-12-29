use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{length::IoLength, read::AsyncIoRead},
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

#[derive(
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
pub struct QueryBroadcastSmResp {
    message_id: COctetString<1, 65>,
    message_state: TLV,
    broadcast_area_identifier: TLV,
    broadcast_area_success: TLV,
    #[rusmpp_io_read(length=(length - all_before))]
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

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn message_state(&self) -> &TLV {
        &self.message_state
    }

    pub fn broadcast_area_identifier(&self) -> &TLV {
        &self.broadcast_area_identifier
    }

    pub fn broadcast_area_success(&self) -> &TLV {
        &self.broadcast_area_success
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (COctetString<1, 65>, TLV, TLV, TLV, Vec<TLV>) {
        (
            self.message_id,
            self.message_state,
            self.broadcast_area_identifier,
            self.broadcast_area_success,
            self.tlvs,
        )
    }
}
