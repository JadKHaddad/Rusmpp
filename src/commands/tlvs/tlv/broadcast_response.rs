use super::Tlv;
use crate::commands::{
    tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
    types::{broadcast_area_identifier::BroadcastAreaIdentifier, command_status::CommandStatus},
};

crate::create! {
    #[repr(u16)]
    @[skip_test]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BroadcastResponseTlvTag {
        BroadcastAreaIdentifier = 0x0606,
        BroadcastErrorStatus = 0x0607,
        Other(u16),
    }
}

impl From<u16> for BroadcastResponseTlvTag {
    fn from(tag: u16) -> Self {
        match tag {
            0x0606 => BroadcastResponseTlvTag::BroadcastAreaIdentifier,
            0x0607 => BroadcastResponseTlvTag::BroadcastErrorStatus,

            other => BroadcastResponseTlvTag::Other(other),
        }
    }
}

impl From<BroadcastResponseTlvTag> for u16 {
    fn from(tag: BroadcastResponseTlvTag) -> Self {
        match tag {
            BroadcastResponseTlvTag::BroadcastAreaIdentifier => 0x0606,
            BroadcastResponseTlvTag::BroadcastErrorStatus => 0x0607,
            BroadcastResponseTlvTag::Other(other) => other,
        }
    }
}

impl From<BroadcastResponseTlvTag> for TlvTag {
    fn from(tag: BroadcastResponseTlvTag) -> Self {
        match tag {
            BroadcastResponseTlvTag::BroadcastAreaIdentifier => TlvTag::BroadcastAreaIdentifier,
            BroadcastResponseTlvTag::BroadcastErrorStatus => TlvTag::BroadcastErrorStatus,
            BroadcastResponseTlvTag::Other(other) => TlvTag::Other(other),
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
