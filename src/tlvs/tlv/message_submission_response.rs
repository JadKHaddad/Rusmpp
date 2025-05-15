mod tag {
    use crate::tlvs::TlvTag;

    crate::create! {
        #[repr(u16)]
        @[skip_test]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum MessageSubmissionResponseTlvTag {
            AdditionalStatusInfoText = 0x001D,
            DpfResult = 0x0420,
            NetworkErrorCode = 0x0423,
            DeliveryFailureReason = 0x0425,
            Other(u16),
        }
    }

    impl From<u16> for MessageSubmissionResponseTlvTag {
        fn from(tag: u16) -> Self {
            match tag {
                0x001D => MessageSubmissionResponseTlvTag::AdditionalStatusInfoText,
                0x0420 => MessageSubmissionResponseTlvTag::DpfResult,
                0x0423 => MessageSubmissionResponseTlvTag::NetworkErrorCode,
                0x0425 => MessageSubmissionResponseTlvTag::DeliveryFailureReason,
                other => MessageSubmissionResponseTlvTag::Other(other),
            }
        }
    }

    impl From<MessageSubmissionResponseTlvTag> for u16 {
        fn from(tag: MessageSubmissionResponseTlvTag) -> Self {
            match tag {
                MessageSubmissionResponseTlvTag::AdditionalStatusInfoText => 0x001D,
                MessageSubmissionResponseTlvTag::DpfResult => 0x0420,
                MessageSubmissionResponseTlvTag::NetworkErrorCode => 0x0423,
                MessageSubmissionResponseTlvTag::DeliveryFailureReason => 0x0425,
                MessageSubmissionResponseTlvTag::Other(other) => other,
            }
        }
    }

    impl From<MessageSubmissionResponseTlvTag> for TlvTag {
        fn from(tag: MessageSubmissionResponseTlvTag) -> Self {
            match tag {
                MessageSubmissionResponseTlvTag::AdditionalStatusInfoText => {
                    TlvTag::AdditionalStatusInfoText
                }
                MessageSubmissionResponseTlvTag::DpfResult => TlvTag::DpfResult,
                MessageSubmissionResponseTlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode,
                MessageSubmissionResponseTlvTag::DeliveryFailureReason => {
                    TlvTag::DeliveryFailureReason
                }
                MessageSubmissionResponseTlvTag::Other(other) => TlvTag::Other(other),
            }
        }
    }
}

mod value {
    use crate::{
        commands::types::{
            delivery_failure_reason::DeliveryFailureReason, dpf_result::DpfResult,
            network_error_code::NetworkErrorCode,
        },
        types::{AnyOctetString, COctetString},
    };

    use super::tag::MessageSubmissionResponseTlvTag;

    crate::create_tlv_value! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum MessageSubmissionResponseTlvValue {
            AdditionalStatusInfoText(COctetString<1, 256>),
            DeliveryFailureReason(DeliveryFailureReason),
            DpfResult(DpfResult),
            NetworkErrorCode(NetworkErrorCode),
            @Other {
                tag: MessageSubmissionResponseTlvTag,
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

    use super::{tag::MessageSubmissionResponseTlvTag, value::MessageSubmissionResponseTlvValue};

    crate::create! {
        @[skip_test]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct MessageSubmissionResponseTlv {
            tag: MessageSubmissionResponseTlvTag,
            value_length: u16,
            @[key = tag, length = value_length]
            value: Option<MessageSubmissionResponseTlvValue>,
        }
    }

    impl MessageSubmissionResponseTlv {
        pub fn new(value: impl Into<MessageSubmissionResponseTlvValue>) -> Self {
            let value = value.into();
            let tag = value.tag();
            let value_length = value.length() as u16;

            Self {
                tag,
                value_length,
                value: Some(value),
            }
        }

        pub const fn tag(&self) -> MessageSubmissionResponseTlvTag {
            self.tag
        }

        pub const fn value_length(&self) -> u16 {
            self.value_length
        }

        pub const fn value(&self) -> Option<&MessageSubmissionResponseTlvValue> {
            self.value.as_ref()
        }
    }

    impl From<MessageSubmissionResponseTlvValue> for MessageSubmissionResponseTlv {
        fn from(value: MessageSubmissionResponseTlvValue) -> Self {
            Self::new(value)
        }
    }

    impl From<MessageSubmissionResponseTlv> for Tlv {
        fn from(tlv: MessageSubmissionResponseTlv) -> Self {
            Self {
                tag: TlvTag::from(tlv.tag),
                value_length: tlv.value_length,
                value: tlv.value.map(TlvValue::from),
            }
        }
    }
}

pub use tag::MessageSubmissionResponseTlvTag;
pub use tlv::MessageSubmissionResponseTlv;
pub use value::MessageSubmissionResponseTlvValue;
