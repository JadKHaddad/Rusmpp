use super::Tlv;
use crate::commands::{
    tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
    types::{broadcast_area_identifier::BroadcastAreaIdentifier, command_status::CommandStatus},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastResponseTlvTag {
    BroadcastErrorStatus,
    BroadcastAreaIdentifier,
}

impl From<BroadcastResponseTlvTag> for TlvTag {
    fn from(v: BroadcastResponseTlvTag) -> Self {
        match v {
            BroadcastResponseTlvTag::BroadcastErrorStatus => TlvTag::BroadcastErrorStatus,
            BroadcastResponseTlvTag::BroadcastAreaIdentifier => TlvTag::BroadcastAreaIdentifier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastResponseTlvValue {
    BroadcastErrorStatus(CommandStatus),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
}

impl From<BroadcastResponseTlvValue> for TlvValue {
    fn from(value: BroadcastResponseTlvValue) -> Self {
        match value {
            BroadcastResponseTlvValue::BroadcastErrorStatus(value) => {
                TlvValue::BroadcastErrorStatus(value)
            }
            BroadcastResponseTlvValue::BroadcastAreaIdentifier(value) => {
                TlvValue::BroadcastAreaIdentifier(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastResponseTlv {
    tlv: Tlv,
}

impl BroadcastResponseTlv {
    pub fn new(value: BroadcastResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: BroadcastResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<BroadcastResponseTlvTag> for Tlv {
    fn from(tag: BroadcastResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<BroadcastResponseTlvValue> for BroadcastResponseTlv {
    fn from(value: BroadcastResponseTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<BroadcastResponseTlvValue> for Tlv {
    fn from(value: BroadcastResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<BroadcastResponseTlv> for Tlv {
    fn from(tlv: BroadcastResponseTlv) -> Self {
        tlv.tlv
    }
}
