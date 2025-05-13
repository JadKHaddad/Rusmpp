use super::Tlv;
use crate::{
    commands::{
        tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
        types::UserMessageReference,
    },
    types::OctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueryBroadcastResponseTlvTag {
    BroadcastEndTime,
    UserMessageReference,
}

impl From<QueryBroadcastResponseTlvTag> for TlvTag {
    fn from(value: QueryBroadcastResponseTlvTag) -> Self {
        match value {
            QueryBroadcastResponseTlvTag::BroadcastEndTime => TlvTag::BroadcastEndTime,
            QueryBroadcastResponseTlvTag::UserMessageReference => TlvTag::UserMessageReference,
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
