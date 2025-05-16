mod tag {
    use crate::tlvs::TlvTag;

    crate::create! {
        #[repr(u16)]
        @[skip_test]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        pub enum MessageSubmissionRequestTlvTag {
            AlertOnMessageDelivery = 0x130C,
            BillingIdentification = 0x060B,
            CallbackNum = 0x0381,
            CallbackNumAtag = 0x0303,
            CallbackNumPresInd = 0x0302,
            DestAddrNpCountry = 0x0613,
            DestAddrNpInformation = 0x0612,
            DestAddrNpResolution = 0x0611,
            DestAddrSubunit = 0x0005,
            DestBearerType = 0x0007,
            DestNetworkId = 0x060E,
            DestNetworkType = 0x0006,
            DestNodeId = 0x0610,
            DestSubaddress = 0x0203,
            DestTelematicsId = 0x0008,
            DestPort = 0x020B,
            DisplayTime = 0x1201,
            ItsReplyType = 0x1380,
            ItsSessionInfo = 0x1383,
            LanguageIndicator = 0x020D,
            MessagePayload = 0x0424,
            MoreMessagesToSend = 0x0426,
            MsMsgWaitFacilities = 0x0030,
            MsValidity = 0x1204,
            NumberOfMessages = 0x0304,
            PayloadType = 0x0019,
            PrivacyIndicator = 0x0201,
            QosTimeToLive = 0x0017,
            SarMsgRefNum = 0x020C,
            SarSegmentSeqnum = 0x020F,
            SarTotalSegments = 0x020E,
            SetDpf = 0x0421,
            SmsSignal = 0x1203,
            SourceAddrSubunit = 0x000D,
            SourceBearerType = 0x000F,
            SourceNetworkId = 0x060D,
            SourceNetworkType = 0x000E,
            SourceNodeId = 0x060F,
            SourcePort = 0x020A,
            SourceSubaddress = 0x0202,
            SourceTelematicsId = 0x0010,
            UserMessageReference = 0x0204,
            UserResponseCode = 0x0205,
            UssdServiceOp = 0x0501,
            Other(u16),
        }
    }

    impl From<u16> for MessageSubmissionRequestTlvTag {
        fn from(tag: u16) -> Self {
            match tag {
                0x130C => MessageSubmissionRequestTlvTag::AlertOnMessageDelivery,
                0x060B => MessageSubmissionRequestTlvTag::BillingIdentification,
                0x0381 => MessageSubmissionRequestTlvTag::CallbackNum,
                0x0303 => MessageSubmissionRequestTlvTag::CallbackNumAtag,
                0x0302 => MessageSubmissionRequestTlvTag::CallbackNumPresInd,
                0x0613 => MessageSubmissionRequestTlvTag::DestAddrNpCountry,
                0x0612 => MessageSubmissionRequestTlvTag::DestAddrNpInformation,
                0x0611 => MessageSubmissionRequestTlvTag::DestAddrNpResolution,
                0x0005 => MessageSubmissionRequestTlvTag::DestAddrSubunit,
                0x0007 => MessageSubmissionRequestTlvTag::DestBearerType,
                0x060E => MessageSubmissionRequestTlvTag::DestNetworkId,
                0x0006 => MessageSubmissionRequestTlvTag::DestNetworkType,
                0x0610 => MessageSubmissionRequestTlvTag::DestNodeId,
                0x0203 => MessageSubmissionRequestTlvTag::DestSubaddress,
                0x0008 => MessageSubmissionRequestTlvTag::DestTelematicsId,
                0x020B => MessageSubmissionRequestTlvTag::DestPort,
                0x1201 => MessageSubmissionRequestTlvTag::DisplayTime,
                0x1380 => MessageSubmissionRequestTlvTag::ItsReplyType,
                0x1383 => MessageSubmissionRequestTlvTag::ItsSessionInfo,
                0x020D => MessageSubmissionRequestTlvTag::LanguageIndicator,
                0x0424 => MessageSubmissionRequestTlvTag::MessagePayload,
                0x0426 => MessageSubmissionRequestTlvTag::MoreMessagesToSend,
                0x0030 => MessageSubmissionRequestTlvTag::MsMsgWaitFacilities,
                0x1204 => MessageSubmissionRequestTlvTag::MsValidity,
                0x0304 => MessageSubmissionRequestTlvTag::NumberOfMessages,
                0x0019 => MessageSubmissionRequestTlvTag::PayloadType,
                0x0201 => MessageSubmissionRequestTlvTag::PrivacyIndicator,
                0x0017 => MessageSubmissionRequestTlvTag::QosTimeToLive,
                0x020C => MessageSubmissionRequestTlvTag::SarMsgRefNum,
                0x020F => MessageSubmissionRequestTlvTag::SarSegmentSeqnum,
                0x020E => MessageSubmissionRequestTlvTag::SarTotalSegments,
                0x0421 => MessageSubmissionRequestTlvTag::SetDpf,
                0x1203 => MessageSubmissionRequestTlvTag::SmsSignal,
                0x000D => MessageSubmissionRequestTlvTag::SourceAddrSubunit,
                0x000F => MessageSubmissionRequestTlvTag::SourceBearerType,
                0x060D => MessageSubmissionRequestTlvTag::SourceNetworkId,
                0x000E => MessageSubmissionRequestTlvTag::SourceNetworkType,
                0x060F => MessageSubmissionRequestTlvTag::SourceNodeId,
                0x020A => MessageSubmissionRequestTlvTag::SourcePort,
                0x0202 => MessageSubmissionRequestTlvTag::SourceSubaddress,
                0x0010 => MessageSubmissionRequestTlvTag::SourceTelematicsId,
                0x0204 => MessageSubmissionRequestTlvTag::UserMessageReference,
                0x0205 => MessageSubmissionRequestTlvTag::UserResponseCode,
                0x0501 => MessageSubmissionRequestTlvTag::UssdServiceOp,
                other => MessageSubmissionRequestTlvTag::Other(other),
            }
        }
    }

    impl From<MessageSubmissionRequestTlvTag> for u16 {
        fn from(tag: MessageSubmissionRequestTlvTag) -> Self {
            match tag {
                MessageSubmissionRequestTlvTag::AlertOnMessageDelivery => 0x130C,
                MessageSubmissionRequestTlvTag::BillingIdentification => 0x060B,
                MessageSubmissionRequestTlvTag::CallbackNum => 0x0381,
                MessageSubmissionRequestTlvTag::CallbackNumAtag => 0x0303,
                MessageSubmissionRequestTlvTag::CallbackNumPresInd => 0x0302,
                MessageSubmissionRequestTlvTag::DestAddrNpCountry => 0x0613,
                MessageSubmissionRequestTlvTag::DestAddrNpInformation => 0x0612,
                MessageSubmissionRequestTlvTag::DestAddrNpResolution => 0x0611,
                MessageSubmissionRequestTlvTag::DestAddrSubunit => 0x0005,
                MessageSubmissionRequestTlvTag::DestBearerType => 0x0007,
                MessageSubmissionRequestTlvTag::DestNetworkId => 0x060E,
                MessageSubmissionRequestTlvTag::DestNetworkType => 0x0006,
                MessageSubmissionRequestTlvTag::DestNodeId => 0x0610,
                MessageSubmissionRequestTlvTag::DestSubaddress => 0x0203,
                MessageSubmissionRequestTlvTag::DestTelematicsId => 0x0008,
                MessageSubmissionRequestTlvTag::DestPort => 0x020B,
                MessageSubmissionRequestTlvTag::DisplayTime => 0x1201,
                MessageSubmissionRequestTlvTag::ItsReplyType => 0x1380,
                MessageSubmissionRequestTlvTag::ItsSessionInfo => 0x1383,
                MessageSubmissionRequestTlvTag::LanguageIndicator => 0x020D,
                MessageSubmissionRequestTlvTag::MessagePayload => 0x0424,
                MessageSubmissionRequestTlvTag::MoreMessagesToSend => 0x0426,
                MessageSubmissionRequestTlvTag::MsMsgWaitFacilities => 0x0030,
                MessageSubmissionRequestTlvTag::MsValidity => 0x1204,
                MessageSubmissionRequestTlvTag::NumberOfMessages => 0x0304,
                MessageSubmissionRequestTlvTag::PayloadType => 0x0019,
                MessageSubmissionRequestTlvTag::PrivacyIndicator => 0x0201,
                MessageSubmissionRequestTlvTag::QosTimeToLive => 0x0017,
                MessageSubmissionRequestTlvTag::SarMsgRefNum => 0x020C,
                MessageSubmissionRequestTlvTag::SarSegmentSeqnum => 0x020F,
                MessageSubmissionRequestTlvTag::SarTotalSegments => 0x020E,
                MessageSubmissionRequestTlvTag::SetDpf => 0x0421,
                MessageSubmissionRequestTlvTag::SmsSignal => 0x1203,
                MessageSubmissionRequestTlvTag::SourceAddrSubunit => 0x000D,
                MessageSubmissionRequestTlvTag::SourceBearerType => 0x000F,
                MessageSubmissionRequestTlvTag::SourceNetworkId => 0x060D,
                MessageSubmissionRequestTlvTag::SourceNetworkType => 0x000E,
                MessageSubmissionRequestTlvTag::SourceNodeId => 0x060F,
                MessageSubmissionRequestTlvTag::SourcePort => 0x020A,
                MessageSubmissionRequestTlvTag::SourceSubaddress => 0x0202,
                MessageSubmissionRequestTlvTag::SourceTelematicsId => 0x0010,
                MessageSubmissionRequestTlvTag::UserMessageReference => 0x0204,
                MessageSubmissionRequestTlvTag::UserResponseCode => 0x0205,
                MessageSubmissionRequestTlvTag::UssdServiceOp => 0x0501,
                MessageSubmissionRequestTlvTag::Other(other) => other,
            }
        }
    }

    impl From<MessageSubmissionRequestTlvTag> for TlvTag {
        fn from(tag: MessageSubmissionRequestTlvTag) -> Self {
            match tag {
                MessageSubmissionRequestTlvTag::AlertOnMessageDelivery => {
                    TlvTag::AlertOnMessageDelivery
                }
                MessageSubmissionRequestTlvTag::BillingIdentification => {
                    TlvTag::BillingIdentification
                }
                MessageSubmissionRequestTlvTag::CallbackNum => TlvTag::CallbackNum,
                MessageSubmissionRequestTlvTag::CallbackNumAtag => TlvTag::CallbackNumAtag,
                MessageSubmissionRequestTlvTag::CallbackNumPresInd => TlvTag::CallbackNumPresInd,
                MessageSubmissionRequestTlvTag::DestAddrNpCountry => TlvTag::DestAddrNpCountry,
                MessageSubmissionRequestTlvTag::DestAddrNpInformation => {
                    TlvTag::DestAddrNpInformation
                }
                MessageSubmissionRequestTlvTag::DestAddrNpResolution => {
                    TlvTag::DestAddrNpResolution
                }
                MessageSubmissionRequestTlvTag::DestAddrSubunit => TlvTag::DestAddrSubunit,
                MessageSubmissionRequestTlvTag::DestBearerType => TlvTag::DestBearerType,
                MessageSubmissionRequestTlvTag::DestNetworkId => TlvTag::DestNetworkId,
                MessageSubmissionRequestTlvTag::DestNetworkType => TlvTag::DestNetworkType,
                MessageSubmissionRequestTlvTag::DestNodeId => TlvTag::DestNodeId,
                MessageSubmissionRequestTlvTag::DestSubaddress => TlvTag::DestSubaddress,
                MessageSubmissionRequestTlvTag::DestTelematicsId => TlvTag::DestTelematicsId,
                MessageSubmissionRequestTlvTag::DestPort => TlvTag::DestPort,
                MessageSubmissionRequestTlvTag::DisplayTime => TlvTag::DisplayTime,
                MessageSubmissionRequestTlvTag::ItsReplyType => TlvTag::ItsReplyType,
                MessageSubmissionRequestTlvTag::ItsSessionInfo => TlvTag::ItsSessionInfo,
                MessageSubmissionRequestTlvTag::LanguageIndicator => TlvTag::LanguageIndicator,
                MessageSubmissionRequestTlvTag::MessagePayload => TlvTag::MessagePayload,
                MessageSubmissionRequestTlvTag::MoreMessagesToSend => TlvTag::MoreMessagesToSend,
                MessageSubmissionRequestTlvTag::MsMsgWaitFacilities => TlvTag::MsMsgWaitFacilities,
                MessageSubmissionRequestTlvTag::MsValidity => TlvTag::MsValidity,
                MessageSubmissionRequestTlvTag::NumberOfMessages => TlvTag::NumberOfMessages,
                MessageSubmissionRequestTlvTag::PayloadType => TlvTag::PayloadType,
                MessageSubmissionRequestTlvTag::PrivacyIndicator => TlvTag::PrivacyIndicator,
                MessageSubmissionRequestTlvTag::QosTimeToLive => TlvTag::QosTimeToLive,
                MessageSubmissionRequestTlvTag::SarMsgRefNum => TlvTag::SarMsgRefNum,
                MessageSubmissionRequestTlvTag::SarSegmentSeqnum => TlvTag::SarSegmentSeqnum,
                MessageSubmissionRequestTlvTag::SarTotalSegments => TlvTag::SarTotalSegments,
                MessageSubmissionRequestTlvTag::SetDpf => TlvTag::SetDpf,
                MessageSubmissionRequestTlvTag::SmsSignal => TlvTag::SmsSignal,
                MessageSubmissionRequestTlvTag::SourceAddrSubunit => TlvTag::SourceAddrSubunit,
                MessageSubmissionRequestTlvTag::SourceBearerType => TlvTag::SourceBearerType,
                MessageSubmissionRequestTlvTag::SourceNetworkId => TlvTag::SourceNetworkId,
                MessageSubmissionRequestTlvTag::SourceNetworkType => TlvTag::SourceNetworkType,
                MessageSubmissionRequestTlvTag::SourceNodeId => TlvTag::SourceNodeId,
                MessageSubmissionRequestTlvTag::SourcePort => TlvTag::SourcePort,
                MessageSubmissionRequestTlvTag::SourceSubaddress => TlvTag::SourceSubaddress,
                MessageSubmissionRequestTlvTag::SourceTelematicsId => TlvTag::SourceTelematicsId,
                MessageSubmissionRequestTlvTag::UserMessageReference => {
                    TlvTag::UserMessageReference
                }
                MessageSubmissionRequestTlvTag::UserResponseCode => TlvTag::UserResponseCode,
                MessageSubmissionRequestTlvTag::UssdServiceOp => TlvTag::UssdServiceOp,
                MessageSubmissionRequestTlvTag::Other(other) => TlvTag::Other(other),
            }
        }
    }
}

mod value {
    use crate::{
        commands::types::{
            addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMessageDelivery,
            bearer_type::BearerType, callback_num_pres_ind::CallbackNumPresInd,
            dest_addr_np_resolution::DestAddrNpResolution, display_time::DisplayTime,
            its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
            language_indicator::LanguageIndicator, more_messages_to_send::MoreMessagesToSend,
            ms_msg_wait_facilities::MsMsgWaitFacilities, ms_validity::MsValidity,
            network_type::NetworkType, number_of_messages::NumberOfMessages,
            payload_type::PayloadType, privacy_indicator::PrivacyIndicator, set_dpf::SetDpf,
            sub_address::Subaddress, ussd_service_op::UssdServiceOp, MessagePayload,
            UserMessageReference,
        },
        types::{AnyOctetString, COctetString, OctetString},
    };

    use super::tag::MessageSubmissionRequestTlvTag;

    crate::create_tlv_value! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        pub enum MessageSubmissionRequestTlvValue {
            AlertOnMessageDelivery(AlertOnMessageDelivery),
            BillingIdentification(OctetString<0, 1024>),
            CallbackNum(OctetString<4, 19>),
            CallbackNumAtag(OctetString<0, 65>),
            CallbackNumPresInd(CallbackNumPresInd),
            DestAddrNpCountry(OctetString<1, 5>),
            DestAddrNpInformation(OctetString<0, 10>),
            DestAddrNpResolution(DestAddrNpResolution),
            DestAddrSubunit(AddrSubunit),
            DestBearerType(BearerType),
            DestNetworkId(COctetString<7, 66>),
            DestNetworkType(NetworkType),
            DestNodeId(OctetString<6, 6>),
            DestSubaddress(Subaddress),
            DestTelematicsId(u16),
            DestPort(u16),
            DisplayTime(DisplayTime),
            ItsReplyType(ItsReplyType),
            ItsSessionInfo(ItsSessionInfo),
            LanguageIndicator(LanguageIndicator),
            MessagePayload(MessagePayload),
            MoreMessagesToSend(MoreMessagesToSend),
            MsMsgWaitFacilities(MsMsgWaitFacilities),
            MsValidity(MsValidity),
            NumberOfMessages(NumberOfMessages),
            PayloadType(PayloadType),
            PrivacyIndicator(PrivacyIndicator),
            QosTimeToLive(u32),
            SarMsgRefNum(u16),
            SarSegmentSeqnum(u8),
            SarTotalSegments(u8),
            SetDpf(SetDpf),
            SmsSignal(u16),
            SourceAddrSubunit(AddrSubunit),
            SourceBearerType(BearerType),
            SourceNetworkId(COctetString<7, 66>),
            SourceNetworkType(NetworkType),
            SourceNodeId(OctetString<6, 6>),
            SourcePort(u16),
            SourceSubaddress(Subaddress),
            SourceTelematicsId(u16),
            UserMessageReference(UserMessageReference),
            UserResponseCode(u8),
            UssdServiceOp(UssdServiceOp),
            @Other {
                tag: MessageSubmissionRequestTlvTag,
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

    use super::{tag::MessageSubmissionRequestTlvTag, value::MessageSubmissionRequestTlvValue};

    crate::create! {
        @[skip_test]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        pub struct MessageSubmissionRequestTlv {
            tag: MessageSubmissionRequestTlvTag,
            value_length: u16,
            @[key = tag, length = value_length]
            value: Option<MessageSubmissionRequestTlvValue>,
        }
    }

    impl MessageSubmissionRequestTlv {
        pub fn new(value: impl Into<MessageSubmissionRequestTlvValue>) -> Self {
            let value = value.into();
            let tag = value.tag();
            let value_length = value.length() as u16;

            Self {
                tag,
                value_length,
                value: Some(value),
            }
        }

        pub const fn tag(&self) -> MessageSubmissionRequestTlvTag {
            self.tag
        }

        pub const fn value_length(&self) -> u16 {
            self.value_length
        }

        pub const fn value(&self) -> Option<&MessageSubmissionRequestTlvValue> {
            self.value.as_ref()
        }
    }

    impl From<MessageSubmissionRequestTlvValue> for MessageSubmissionRequestTlv {
        fn from(value: MessageSubmissionRequestTlvValue) -> Self {
            Self::new(value)
        }
    }

    impl From<MessageSubmissionRequestTlv> for Tlv {
        fn from(tlv: MessageSubmissionRequestTlv) -> Self {
            Self {
                tag: TlvTag::from(tlv.tag),
                value_length: tlv.value_length,
                value: tlv.value.map(TlvValue::from),
            }
        }
    }
}

pub use tag::MessageSubmissionRequestTlvTag;
pub use tlv::MessageSubmissionRequestTlv;
pub use value::MessageSubmissionRequestTlvValue;
