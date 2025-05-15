mod tag {
    use crate::tlvs::TlvTag;

    crate::create! {
        #[repr(u16)]
        @[skip_test]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum MessageDeliveryResponseTlvTag {
            AdditionalStatusInfoText = 0x001D,
            DeliveryFailureReason = 0x0425,
            NetworkErrorCode = 0x0423,
            Other(u16),
        }
    }

    impl From<u16> for MessageDeliveryResponseTlvTag {
        fn from(tag: u16) -> Self {
            match tag {
                0x001D => MessageDeliveryResponseTlvTag::AdditionalStatusInfoText,
                0x0425 => MessageDeliveryResponseTlvTag::DeliveryFailureReason,
                0x0423 => MessageDeliveryResponseTlvTag::NetworkErrorCode,
                other => MessageDeliveryResponseTlvTag::Other(other),
            }
        }
    }

    impl From<MessageDeliveryResponseTlvTag> for u16 {
        fn from(tag: MessageDeliveryResponseTlvTag) -> Self {
            match tag {
                MessageDeliveryResponseTlvTag::AdditionalStatusInfoText => 0x001D,
                MessageDeliveryResponseTlvTag::DeliveryFailureReason => 0x0425,
                MessageDeliveryResponseTlvTag::NetworkErrorCode => 0x0423,
                MessageDeliveryResponseTlvTag::Other(other) => other,
            }
        }
    }

    impl From<MessageDeliveryResponseTlvTag> for TlvTag {
        fn from(tag: MessageDeliveryResponseTlvTag) -> Self {
            match tag {
                MessageDeliveryResponseTlvTag::AdditionalStatusInfoText => {
                    TlvTag::AdditionalStatusInfoText
                }
                MessageDeliveryResponseTlvTag::DeliveryFailureReason => {
                    TlvTag::DeliveryFailureReason
                }
                MessageDeliveryResponseTlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode,
                MessageDeliveryResponseTlvTag::Other(other) => TlvTag::Other(other),
            }
        }
    }
}

mod value {
    use crate::{
        commands::types::{
            delivery_failure_reason::DeliveryFailureReason, network_error_code::NetworkErrorCode,
        },
        types::{AnyOctetString, COctetString},
    };

    use super::tag::MessageDeliveryResponseTlvTag;

    crate::create_tlv_value! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum MessageDeliveryResponseTlvValue {
            AdditionalStatusInfoText(COctetString<1, 256>),
            DeliveryFailureReason(DeliveryFailureReason),
            NetworkErrorCode(NetworkErrorCode),
            @Other {
                tag: MessageDeliveryResponseTlvTag,
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

    use super::{tag::MessageDeliveryResponseTlvTag, value::MessageDeliveryResponseTlvValue};

    crate::create! {
        @[skip_test]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct MessageDeliveryResponseTlv {
            tag: MessageDeliveryResponseTlvTag,
            value_length: u16,
            @[key = tag, length = value_length]
            value: Option<MessageDeliveryResponseTlvValue>,
        }
    }

    impl MessageDeliveryResponseTlv {
        pub fn new(value: MessageDeliveryResponseTlvValue) -> Self {
            let tag = value.tag();
            let value_length = value.length() as u16;

            Self {
                tag,
                value_length,
                value: Some(value),
            }
        }

        pub const fn tag(&self) -> MessageDeliveryResponseTlvTag {
            self.tag
        }

        pub const fn value_length(&self) -> u16 {
            self.value_length
        }

        pub const fn value(&self) -> Option<&MessageDeliveryResponseTlvValue> {
            self.value.as_ref()
        }
    }

    impl From<MessageDeliveryResponseTlvValue> for MessageDeliveryResponseTlv {
        fn from(value: MessageDeliveryResponseTlvValue) -> Self {
            Self::new(value)
        }
    }

    impl From<MessageDeliveryResponseTlv> for Tlv {
        fn from(tlv: MessageDeliveryResponseTlv) -> Self {
            Self {
                tag: TlvTag::from(tlv.tag),
                value_length: tlv.value_length,
                value: tlv.value.map(TlvValue::from),
            }
        }
    }
}

pub use tag::MessageDeliveryResponseTlvTag;
pub use tlv::MessageDeliveryResponseTlv;
pub use value::MessageDeliveryResponseTlvValue;
