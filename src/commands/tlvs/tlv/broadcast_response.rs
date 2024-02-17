use super::TLV;
use crate::commands::{
    tlvs::{tlv_tag::TLVTag, tlv_value::TLVValue},
    types::{broadcast_area_identifier::BroadcastAreaIdentifier, command_status::CommandStatus},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastResponseTLVTag {
    BroadcastErrorStatus,
    BroadcastAreaIdentifier,
}

impl From<BroadcastResponseTLVTag> for TLVTag {
    fn from(v: BroadcastResponseTLVTag) -> Self {
        match v {
            BroadcastResponseTLVTag::BroadcastErrorStatus => TLVTag::BroadcastErrorStatus,
            BroadcastResponseTLVTag::BroadcastAreaIdentifier => TLVTag::BroadcastAreaIdentifier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastResponseTLVValue {
    BroadcastErrorStatus(CommandStatus),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
}

impl From<BroadcastResponseTLVValue> for TLVValue {
    fn from(value: BroadcastResponseTLVValue) -> Self {
        match value {
            BroadcastResponseTLVValue::BroadcastErrorStatus(value) => {
                TLVValue::BroadcastErrorStatus(value)
            }
            BroadcastResponseTLVValue::BroadcastAreaIdentifier(value) => {
                TLVValue::BroadcastAreaIdentifier(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastResponseTLV {
    tlv: TLV,
}

impl BroadcastResponseTLV {
    pub fn new(value: BroadcastResponseTLVValue) -> Self {
        let value = TLVValue::from(value);
        let tlv = TLV::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: BroadcastResponseTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        let tlv = TLV::from(tag);

        Self { tlv }
    }
}

impl From<BroadcastResponseTLVTag> for TLV {
    fn from(tag: BroadcastResponseTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        TLV::from(tag)
    }
}

impl From<BroadcastResponseTLVValue> for TLV {
    fn from(value: BroadcastResponseTLVValue) -> Self {
        let value = TLVValue::from(value);
        TLV::from(value)
    }
}

impl From<BroadcastResponseTLV> for TLV {
    fn from(tlv: BroadcastResponseTLV) -> Self {
        tlv.tlv
    }
}
