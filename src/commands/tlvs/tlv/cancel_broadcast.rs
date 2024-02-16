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
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn without_value(tag: CancelBroadcastTLVTag) -> Self {
        let tlv = TLV::without_value(tag.into());

        Self { tlv }
    }
}

impl From<CancelBroadcastTLV> for TLV {
    fn from(tlv: CancelBroadcastTLV) -> Self {
        tlv.tlv
    }
}
