use crate::{
    tlvs::{HasTlvTag, TlvTag},
    types::AnyOctetString,
};

crate::create! {
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
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
    const TAG: TlvTag = TlvTag::MessagePayload;
}
