use super::TLV;
use crate::{
    commands::tlvs::{tlv_tag::TLVTag, tlv_value::TLVValue},
    types::octet_string::OctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueryBroadcastResponseTLVTag {
    BroadcastEndTime,
    UserMessageReference,
}

impl From<QueryBroadcastResponseTLVTag> for TLVTag {
    fn from(v: QueryBroadcastResponseTLVTag) -> Self {
        match v {
            QueryBroadcastResponseTLVTag::BroadcastEndTime => TLVTag::BroadcastEndTime,
            QueryBroadcastResponseTLVTag::UserMessageReference => TLVTag::UserMessageReference,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueryBroadcastResponseTLVValue {
    BroadcastEndTime(OctetString<0, 17>),
    UserMessageReference(u16),
}

impl From<QueryBroadcastResponseTLVValue> for TLVValue {
    fn from(value: QueryBroadcastResponseTLVValue) -> Self {
        match value {
            QueryBroadcastResponseTLVValue::BroadcastEndTime(value) => {
                TLVValue::BroadcastEndTime(value)
            }
            QueryBroadcastResponseTLVValue::UserMessageReference(value) => {
                TLVValue::UserMessageReference(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueryBroadcastResponseTLV {
    tlv: TLV,
}

impl QueryBroadcastResponseTLV {
    pub fn new(value: QueryBroadcastResponseTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn without_value(tag: QueryBroadcastResponseTLVTag) -> Self {
        let tlv = TLV::without_value(tag.into());

        Self { tlv }
    }
}

impl From<QueryBroadcastResponseTLV> for TLV {
    fn from(tlv: QueryBroadcastResponseTLV) -> Self {
        tlv.tlv
    }
}
