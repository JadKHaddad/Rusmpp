mod tag {
    use crate::tlvs::TlvTag;

    crate::create! {
        #[repr(u16)]
        @[skip_test]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum BroadcastResponseTlvTag {
            BroadcastAreaIdentifier = 0x0606,
            BroadcastErrorStatus = 0x0607,
            Other(u16),
        }
    }

    impl From<u16> for BroadcastResponseTlvTag {
        fn from(tag: u16) -> Self {
            match tag {
                0x0606 => BroadcastResponseTlvTag::BroadcastAreaIdentifier,
                0x0607 => BroadcastResponseTlvTag::BroadcastErrorStatus,

                other => BroadcastResponseTlvTag::Other(other),
            }
        }
    }

    impl From<BroadcastResponseTlvTag> for u16 {
        fn from(tag: BroadcastResponseTlvTag) -> Self {
            match tag {
                BroadcastResponseTlvTag::BroadcastAreaIdentifier => 0x0606,
                BroadcastResponseTlvTag::BroadcastErrorStatus => 0x0607,
                BroadcastResponseTlvTag::Other(other) => other,
            }
        }
    }

    impl From<BroadcastResponseTlvTag> for TlvTag {
        fn from(tag: BroadcastResponseTlvTag) -> Self {
            match tag {
                BroadcastResponseTlvTag::BroadcastAreaIdentifier => TlvTag::BroadcastAreaIdentifier,
                BroadcastResponseTlvTag::BroadcastErrorStatus => TlvTag::BroadcastErrorStatus,
                BroadcastResponseTlvTag::Other(other) => TlvTag::Other(other),
            }
        }
    }
}

mod value {
    use crate::{
        commands::types::{
            broadcast_area_identifier::BroadcastAreaIdentifier, command_status::CommandStatus,
        },
        types::AnyOctetString,
    };

    use super::tag::BroadcastResponseTlvTag;

    crate::create_tlv_value! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub enum BroadcastResponseTlvValue {
                BroadcastErrorStatus(CommandStatus),
                BroadcastAreaIdentifier(BroadcastAreaIdentifier),
                @Other {
                    tag: BroadcastResponseTlvTag,
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

    use super::{tag::BroadcastResponseTlvTag, value::BroadcastResponseTlvValue};

    crate::create! {
        @[skip_test]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct BroadcastResponseTlv {
            tag: BroadcastResponseTlvTag,
            value_length: u16,
            @[key = tag, length = value_length]
            value: Option<BroadcastResponseTlvValue>,
        }
    }

    impl BroadcastResponseTlv {
        pub fn new(value: impl Into<BroadcastResponseTlvValue>) -> Self {
            let value = value.into();
            let tag = value.tag();
            let value_length = value.length() as u16;

            Self {
                tag,
                value_length,
                value: Some(value),
            }
        }

        pub const fn tag(&self) -> BroadcastResponseTlvTag {
            self.tag
        }

        pub const fn value_length(&self) -> u16 {
            self.value_length
        }

        pub const fn value(&self) -> Option<&BroadcastResponseTlvValue> {
            self.value.as_ref()
        }
    }

    impl From<BroadcastResponseTlvValue> for BroadcastResponseTlv {
        fn from(value: BroadcastResponseTlvValue) -> Self {
            Self::new(value)
        }
    }

    impl From<BroadcastResponseTlv> for Tlv {
        fn from(tlv: BroadcastResponseTlv) -> Self {
            Self {
                tag: TlvTag::from(tlv.tag),
                value_length: tlv.value_length,
                value: tlv.value.map(TlvValue::from),
            }
        }
    }
}

pub use tag::BroadcastResponseTlvTag;
pub use tlv::BroadcastResponseTlv;
pub use value::BroadcastResponseTlvValue;
