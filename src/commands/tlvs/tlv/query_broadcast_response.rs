use super::Tlv;
use crate::{
    commands::{
        tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
        types::UserMessageReference,
    },
    types::OctetString,
};

crate::create! {
    #[repr(u16)]
    @[skip_test]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum QueryBroadcastResponseTlvTag {
        UserMessageReference = 0x0204,
        BroadcastEndTime = 0x0609,
        Other(u16),
    }
}

impl From<u16> for QueryBroadcastResponseTlvTag {
    fn from(tag: u16) -> Self {
        match tag {
            0x0204 => QueryBroadcastResponseTlvTag::UserMessageReference,
            0x0609 => QueryBroadcastResponseTlvTag::BroadcastEndTime,
            other => QueryBroadcastResponseTlvTag::Other(other),
        }
    }
}

impl From<QueryBroadcastResponseTlvTag> for u16 {
    fn from(tag: QueryBroadcastResponseTlvTag) -> Self {
        match tag {
            QueryBroadcastResponseTlvTag::UserMessageReference => 0x0204,
            QueryBroadcastResponseTlvTag::BroadcastEndTime => 0x0609,
            QueryBroadcastResponseTlvTag::Other(other) => other,
        }
    }
}

impl From<QueryBroadcastResponseTlvTag> for TlvTag {
    fn from(tag: QueryBroadcastResponseTlvTag) -> Self {
        match tag {
            QueryBroadcastResponseTlvTag::UserMessageReference => TlvTag::UserMessageReference,
            QueryBroadcastResponseTlvTag::BroadcastEndTime => TlvTag::BroadcastEndTime,
            QueryBroadcastResponseTlvTag::Other(other) => TlvTag::Other(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueryBroadcastResponseTlvValue {
    BroadcastEndTime(OctetString<0, 17>),
    UserMessageReference(UserMessageReference),
}

impl From<QueryBroadcastResponseTlvValue> for TlvValue {
    fn from(value: QueryBroadcastResponseTlvValue) -> Self {
        match value {
            QueryBroadcastResponseTlvValue::BroadcastEndTime(value) => {
                TlvValue::BroadcastEndTime(value)
            }
            QueryBroadcastResponseTlvValue::UserMessageReference(value) => {
                TlvValue::UserMessageReference(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueryBroadcastResponseTlv {
    tlv: Tlv,
}

impl QueryBroadcastResponseTlv {
    pub fn new(value: QueryBroadcastResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: QueryBroadcastResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<QueryBroadcastResponseTlvTag> for Tlv {
    fn from(tag: QueryBroadcastResponseTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<QueryBroadcastResponseTlvValue> for QueryBroadcastResponseTlv {
    fn from(value: QueryBroadcastResponseTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<QueryBroadcastResponseTlvValue> for Tlv {
    fn from(value: QueryBroadcastResponseTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<QueryBroadcastResponseTlv> for Tlv {
    fn from(tlv: QueryBroadcastResponseTlv) -> Self {
        tlv.tlv
    }
}
