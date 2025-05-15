mod tag {
    use crate::tlvs::TlvTag;

    crate::create! {
        #[repr(u16)]
        @[skip_test]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum CancelBroadcastTlvTag {
            UserMessageReference = 0x0204,
            BroadcastContentType = 0x0601,
            Other(u16),
        }
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
}

mod value {
    use crate::{
        commands::types::{broadcast_content_type::BroadcastContentType, UserMessageReference},
        types::AnyOctetString,
    };

    use super::tag::CancelBroadcastTlvTag;

    crate::create_tlv_value! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum CancelBroadcastTlvValue {
            /// Specifies the content type of the message.
            BroadcastContentType(BroadcastContentType),
            /// ESME assigned message reference number.
            ///
            /// Note: The message_id field should be set to NULL if
            /// using the user_message_reference TLV.
            UserMessageReference(UserMessageReference),
            @Other {
                tag: CancelBroadcastTlvTag,
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

    use super::{tag::CancelBroadcastTlvTag, value::CancelBroadcastTlvValue};

    crate::create! {
        @[skip_test]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct CancelBroadcastTlv {
            tag: CancelBroadcastTlvTag,
            value_length: u16,
            @[key = tag, length = value_length]
            value: Option<CancelBroadcastTlvValue>,
        }
    }

    impl CancelBroadcastTlv {
        pub fn new(value: impl Into<CancelBroadcastTlvValue>) -> Self {
            let value = value.into();
            let tag = value.tag();
            let value_length = value.length() as u16;

            Self {
                tag,
                value_length,
                value: Some(value),
            }
        }

        pub const fn tag(&self) -> CancelBroadcastTlvTag {
            self.tag
        }

        pub const fn value_length(&self) -> u16 {
            self.value_length
        }

        pub const fn value(&self) -> Option<&CancelBroadcastTlvValue> {
            self.value.as_ref()
        }
    }

    impl From<CancelBroadcastTlvValue> for CancelBroadcastTlv {
        fn from(value: CancelBroadcastTlvValue) -> Self {
            Self::new(value)
        }
    }

    impl From<CancelBroadcastTlv> for Tlv {
        fn from(tlv: CancelBroadcastTlv) -> Self {
            Self {
                tag: TlvTag::from(tlv.tag),
                value_length: tlv.value_length,
                value: tlv.value.map(TlvValue::from),
            }
        }
    }
}

pub use tag::CancelBroadcastTlvTag;
pub use tlv::CancelBroadcastTlv;
pub use value::CancelBroadcastTlvValue;
