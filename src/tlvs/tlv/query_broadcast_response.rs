mod tag {
    use crate::tlvs::TlvTag;

    crate::create! {
        #[repr(u16)]
        @[skip_test]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum QueryBroadcastResponseTlvTag {
            MessageState = 0x0427,
            BroadcastAreaIdentifier = 0x0606,
            BroadcastAreaSuccess = 0x0608,
            UserMessageReference = 0x0204,
            BroadcastEndTime = 0x0609,
            Other(u16),
        }
    }

    impl From<u16> for QueryBroadcastResponseTlvTag {
        fn from(tag: u16) -> Self {
            match tag {
                0x0427 => QueryBroadcastResponseTlvTag::MessageState,
                0x0606 => QueryBroadcastResponseTlvTag::BroadcastAreaIdentifier,
                0x0608 => QueryBroadcastResponseTlvTag::BroadcastAreaSuccess,
                0x0204 => QueryBroadcastResponseTlvTag::UserMessageReference,
                0x0609 => QueryBroadcastResponseTlvTag::BroadcastEndTime,
                other => QueryBroadcastResponseTlvTag::Other(other),
            }
        }
    }

    impl From<QueryBroadcastResponseTlvTag> for u16 {
        fn from(tag: QueryBroadcastResponseTlvTag) -> Self {
            match tag {
                QueryBroadcastResponseTlvTag::MessageState => 0x0427,
                QueryBroadcastResponseTlvTag::BroadcastAreaIdentifier => 0x0606,
                QueryBroadcastResponseTlvTag::BroadcastAreaSuccess => 0x0608,
                QueryBroadcastResponseTlvTag::UserMessageReference => 0x0204,
                QueryBroadcastResponseTlvTag::BroadcastEndTime => 0x0609,
                QueryBroadcastResponseTlvTag::Other(other) => other,
            }
        }
    }

    impl From<QueryBroadcastResponseTlvTag> for TlvTag {
        fn from(tag: QueryBroadcastResponseTlvTag) -> Self {
            match tag {
                QueryBroadcastResponseTlvTag::MessageState => TlvTag::MessageState,
                QueryBroadcastResponseTlvTag::BroadcastAreaIdentifier => {
                    TlvTag::BroadcastAreaIdentifier
                }
                QueryBroadcastResponseTlvTag::BroadcastAreaSuccess => TlvTag::BroadcastAreaSuccess,
                QueryBroadcastResponseTlvTag::UserMessageReference => TlvTag::UserMessageReference,
                QueryBroadcastResponseTlvTag::BroadcastEndTime => TlvTag::BroadcastEndTime,
                QueryBroadcastResponseTlvTag::Other(other) => TlvTag::Other(other),
            }
        }
    }
}

mod value {
    use crate::{
        commands::types::{
            BroadcastAreaIdentifier, BroadcastAreaSuccess, MessageState, UserMessageReference,
        },
        types::{AnyOctetString, OctetString},
    };

    use super::tag::QueryBroadcastResponseTlvTag;

    crate::create_tlv_value! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum QueryBroadcastResponseTlvValue {
            MessageState(MessageState),
            BroadcastAreaIdentifier(BroadcastAreaIdentifier),
            BroadcastAreaSuccess(BroadcastAreaSuccess),
            BroadcastEndTime(OctetString<0, 17>),
            UserMessageReference(UserMessageReference),
            @Other {
                tag: QueryBroadcastResponseTlvTag,
                value: AnyOctetString,
            },
        }
    }
}

mod tlv {
    use crate::{
        encode::Length,
        tlvs::{Tlv, TlvTag, TlvValue},
    };

    use super::{tag::QueryBroadcastResponseTlvTag, value::QueryBroadcastResponseTlvValue};

    crate::create! {
        @[skip_test]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct QueryBroadcastResponseTlv {
            tag: QueryBroadcastResponseTlvTag,
            value_length: u16,
            @[key = tag, length = value_length]
            value: Option<QueryBroadcastResponseTlvValue>,
        }
    }

    impl QueryBroadcastResponseTlv {
        pub fn new(value: impl Into<QueryBroadcastResponseTlvValue>) -> Self {
            let value = value.into();
            let tag = value.tag();
            let value_length = value.length() as u16;

            Self {
                tag,
                value_length,
                value: Some(value),
            }
        }

        pub const fn tag(&self) -> QueryBroadcastResponseTlvTag {
            self.tag
        }

        pub const fn value_length(&self) -> u16 {
            self.value_length
        }

        pub const fn value(&self) -> Option<&QueryBroadcastResponseTlvValue> {
            self.value.as_ref()
        }
    }

    impl From<QueryBroadcastResponseTlvValue> for QueryBroadcastResponseTlv {
        fn from(value: QueryBroadcastResponseTlvValue) -> Self {
            Self::new(value)
        }
    }

    impl From<QueryBroadcastResponseTlv> for Tlv {
        fn from(tlv: QueryBroadcastResponseTlv) -> Self {
            Self {
                tag: TlvTag::from(tlv.tag),
                value_length: tlv.value_length,
                value: tlv.value.map(TlvValue::from),
            }
        }
    }
}

pub use tag::QueryBroadcastResponseTlvTag;
pub use tlv::QueryBroadcastResponseTlv;
pub use value::QueryBroadcastResponseTlvValue;
