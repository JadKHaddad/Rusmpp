use super::TLV;
use crate::commands::{
    tlvs::{tlv_tag::TLVTag, tlv_value::TLVValue},
    types::broadcast_content_type::BroadcastContentType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancelBroadcastTLVTag {
    BroadcastContentType,
    UserMessageReference,
}

impl From<CancelBroadcastTLVTag> for TLVTag {
    fn from(v: CancelBroadcastTLVTag) -> Self {
        match v {
            CancelBroadcastTLVTag::BroadcastContentType => TLVTag::BroadcastContentType,
            CancelBroadcastTLVTag::UserMessageReference => TLVTag::UserMessageReference,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancelBroadcastTLVValue {
    BroadcastContentType(BroadcastContentType),
    UserMessageReference(u16),
}

impl From<CancelBroadcastTLVValue> for TLVValue {
    fn from(value: CancelBroadcastTLVValue) -> Self {
        match value {
            CancelBroadcastTLVValue::BroadcastContentType(value) => {
                TLVValue::BroadcastContentType(value)
            }
            CancelBroadcastTLVValue::UserMessageReference(value) => {
                TLVValue::UserMessageReference(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CancelBroadcastTLV {
    tlv: TLV,
}

impl CancelBroadcastTLV {
    pub fn new(value: CancelBroadcastTLVValue) -> Self {
        let value = TLVValue::from(value);
        let tlv = TLV::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: CancelBroadcastTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        let tlv = TLV::from(tag);

        Self { tlv }
    }
}

impl From<CancelBroadcastTLVTag> for TLV {
    fn from(tag: CancelBroadcastTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        TLV::from(tag)
    }
}

impl From<CancelBroadcastTLVValue> for TLV {
    fn from(value: CancelBroadcastTLVValue) -> Self {
        let value = TLVValue::from(value);
        TLV::from(value)
    }
}

impl From<CancelBroadcastTLV> for TLV {
    fn from(tlv: CancelBroadcastTLV) -> Self {
        tlv.tlv
    }
}
