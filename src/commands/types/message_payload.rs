use crate::{commands::tlvs::tlv::HasTlvTag, types::AnyOctetString, TlvTag};

crate::create! {
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MessagePayload {
        @[length = unchecked]
        pub value: AnyOctetString,
    }
}

impl MessagePayload {
    pub fn new(value: AnyOctetString) -> Self {
        Self { value }
    }
}

impl From<AnyOctetString> for MessagePayload {
    fn from(value: AnyOctetString) -> Self {
        Self::new(value)
    }
}

impl From<MessagePayload> for AnyOctetString {
    fn from(value: MessagePayload) -> Self {
        value.value
    }
}

impl HasTlvTag for MessagePayload {
    fn tlv_tag() -> TlvTag {
        TlvTag::MessagePayload
    }
}
