use super::Tlv;
use crate::commands::{
    tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
    types::{broadcast_content_type::BroadcastContentType, UserMessageReference},
};

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancelBroadcastTlvTag {
    UserMessageReference = 0x0204,
    BroadcastContentType = 0x0601,
    Other(u16),
}

impl From<u16> for CancelBroadcastTlvTag {
    fn from(tag: u16) -> Self {
        match tag {
            0x0204 => CancelBroadcastTlvTag::UserMessageReference,
            0x0601 => CancelBroadcastTlvTag::BroadcastContentType,
            other => CancelBroadcastTlvTag::Other(other),
        }
    }
}

impl From<CancelBroadcastTlvTag> for u16 {
    fn from(tag: CancelBroadcastTlvTag) -> Self {
        match tag {
            CancelBroadcastTlvTag::UserMessageReference => 0x0204,
            CancelBroadcastTlvTag::BroadcastContentType => 0x0601,
            CancelBroadcastTlvTag::Other(other) => other,
        }
    }
}

impl From<CancelBroadcastTlvTag> for TlvTag {
    fn from(tag: CancelBroadcastTlvTag) -> Self {
        match tag {
            CancelBroadcastTlvTag::UserMessageReference => TlvTag::UserMessageReference,
            CancelBroadcastTlvTag::BroadcastContentType => TlvTag::BroadcastContentType,
            CancelBroadcastTlvTag::Other(other) => TlvTag::Other(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancelBroadcastTlvValue {
    /// Specifies the content type of the message.
    BroadcastContentType(BroadcastContentType),
    /// ESME assigned message reference number.
    ///
    /// Note: The message_id field should be set to NULL if
    /// using the user_message_reference TLV.
    UserMessageReference(UserMessageReference),
}

impl From<CancelBroadcastTlvValue> for TlvValue {
    fn from(value: CancelBroadcastTlvValue) -> Self {
        match value {
            CancelBroadcastTlvValue::BroadcastContentType(value) => {
                TlvValue::BroadcastContentType(value)
            }
            CancelBroadcastTlvValue::UserMessageReference(value) => {
                TlvValue::UserMessageReference(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CancelBroadcastTlv {
    tlv: Tlv,
}

impl CancelBroadcastTlv {
    pub fn new(value: CancelBroadcastTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: CancelBroadcastTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<CancelBroadcastTlvTag> for Tlv {
    fn from(tag: CancelBroadcastTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<CancelBroadcastTlvValue> for CancelBroadcastTlv {
    fn from(value: CancelBroadcastTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<CancelBroadcastTlvValue> for Tlv {
    fn from(value: CancelBroadcastTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<CancelBroadcastTlv> for Tlv {
    fn from(tlv: CancelBroadcastTlv) -> Self {
        tlv.tlv
    }
}
