use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    values::*,
};

// XXX: This value has no lifetime parameters but annotated with non_exhaustive. If a new value with a lifetime parameter is added in the future it will be a breaking change.
#[non_exhaustive]
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

impl CancelBroadcastTlvValue {
    pub const fn tag(&self) -> TlvTag {
        match self {
            CancelBroadcastTlvValue::BroadcastContentType(_) => TlvTag::BroadcastContentType,
            CancelBroadcastTlvValue::UserMessageReference(_) => TlvTag::UserMessageReference,
        }
    }
}

impl From<CancelBroadcastTlvValue> for TlvValue<'_> {
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

impl From<CancelBroadcastTlvValue> for Tlv<'_> {
    fn from(value: CancelBroadcastTlvValue) -> Self {
        Self::new(TlvValue::from(value))
    }
}
