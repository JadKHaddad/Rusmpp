use super::Tlv;
use crate::{
    commands::{
        tlvs::{tlv_tag::TlvTag, tlv_value::TlvValue},
        types::{
            addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMsgDelivery,
            broadcast_channel_indicator::BroadcastChannelIndicator,
            broadcast_message_class::BroadcastMessageClass,
            callback_num_pres_ind::CallbackNumPresInd, display_time::DisplayTime,
            language_indicator::LanguageIndicator, ms_validity::MsValidity,
            payload_type::PayloadType, privacy_indicator::PrivacyIndicator,
            sub_address::Subaddress, MessagePayload, UserMessageReference,
        },
    },
    types::OctetString,
};

crate::create! {
    #[repr(u16)]
    @[skip_test]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BroadcastRequestTlvTag {
        AlertOnMessageDelivery = 0x130C,
        BroadcastChannelIndicator = 0x0600,
        BroadcastContentTypeInfo = 0x0602,
        BroadcastMessageClass = 0x0603,
        BroadcastServiceGroup = 0x060A,
        CallbackNum = 0x0381,
        CallbackNumAtag = 0x0303,
        CallbackNumPresInd = 0x0302,
        DestAddrSubunit = 0x0005,
        DestSubaddress = 0x0203,
        DestPort = 0x020B,
        DisplayTime = 0x1201,
        LanguageIndicator = 0x020D,
        MessagePayload = 0x0424,
        MsValidity = 0x1204,
        PayloadType = 0x0019,
        PrivacyIndicator = 0x0201,
        SmsSignal = 0x1203,
        SourceAddrSubunit = 0x000D,
        SourcePort = 0x020A,
        SourceSubaddress = 0x0202,
        UserMessageReference = 0x0204,
        Other(u16),
    }
}

impl From<u16> for BroadcastRequestTlvTag {
    fn from(value: u16) -> Self {
        match value {
            0x130C => BroadcastRequestTlvTag::AlertOnMessageDelivery,
            0x0600 => BroadcastRequestTlvTag::BroadcastChannelIndicator,
            0x0602 => BroadcastRequestTlvTag::BroadcastContentTypeInfo,
            0x0603 => BroadcastRequestTlvTag::BroadcastMessageClass,
            0x060A => BroadcastRequestTlvTag::BroadcastServiceGroup,
            0x0381 => BroadcastRequestTlvTag::CallbackNum,
            0x0303 => BroadcastRequestTlvTag::CallbackNumAtag,
            0x0302 => BroadcastRequestTlvTag::CallbackNumPresInd,
            0x0005 => BroadcastRequestTlvTag::DestAddrSubunit,
            0x0203 => BroadcastRequestTlvTag::DestSubaddress,
            0x020B => BroadcastRequestTlvTag::DestPort,
            0x1201 => BroadcastRequestTlvTag::DisplayTime,
            0x020D => BroadcastRequestTlvTag::LanguageIndicator,
            0x0424 => BroadcastRequestTlvTag::MessagePayload,
            0x1204 => BroadcastRequestTlvTag::MsValidity,
            0x0019 => BroadcastRequestTlvTag::PayloadType,
            0x0201 => BroadcastRequestTlvTag::PrivacyIndicator,
            0x1203 => BroadcastRequestTlvTag::SmsSignal,
            0x000D => BroadcastRequestTlvTag::SourceAddrSubunit,
            0x020A => BroadcastRequestTlvTag::SourcePort,
            0x0202 => BroadcastRequestTlvTag::SourceSubaddress,
            0x0204 => BroadcastRequestTlvTag::UserMessageReference,
            other => BroadcastRequestTlvTag::Other(other),
        }
    }
}

impl From<BroadcastRequestTlvTag> for u16 {
    fn from(tag: BroadcastRequestTlvTag) -> Self {
        match tag {
            BroadcastRequestTlvTag::AlertOnMessageDelivery => 0x130C,
            BroadcastRequestTlvTag::BroadcastChannelIndicator => 0x0600,
            BroadcastRequestTlvTag::BroadcastContentTypeInfo => 0x0602,
            BroadcastRequestTlvTag::BroadcastMessageClass => 0x0603,
            BroadcastRequestTlvTag::BroadcastServiceGroup => 0x060A,
            BroadcastRequestTlvTag::CallbackNum => 0x0381,
            BroadcastRequestTlvTag::CallbackNumAtag => 0x0303,
            BroadcastRequestTlvTag::CallbackNumPresInd => 0x0302,
            BroadcastRequestTlvTag::DestAddrSubunit => 0x0005,
            BroadcastRequestTlvTag::DestSubaddress => 0x0203,
            BroadcastRequestTlvTag::DestPort => 0x020B,
            BroadcastRequestTlvTag::DisplayTime => 0x1201,
            BroadcastRequestTlvTag::LanguageIndicator => 0x020D,
            BroadcastRequestTlvTag::MessagePayload => 0x0424,
            BroadcastRequestTlvTag::MsValidity => 0x1204,
            BroadcastRequestTlvTag::PayloadType => 0x0019,
            BroadcastRequestTlvTag::PrivacyIndicator => 0x0201,
            BroadcastRequestTlvTag::SmsSignal => 0x1203,
            BroadcastRequestTlvTag::SourceAddrSubunit => 0x000D,
            BroadcastRequestTlvTag::SourcePort => 0x020A,
            BroadcastRequestTlvTag::SourceSubaddress => 0x0202,
            BroadcastRequestTlvTag::UserMessageReference => 0x0204,
            BroadcastRequestTlvTag::Other(other) => other,
        }
    }
}

impl From<BroadcastRequestTlvTag> for TlvTag {
    fn from(tag: BroadcastRequestTlvTag) -> Self {
        match tag {
            BroadcastRequestTlvTag::AlertOnMessageDelivery => TlvTag::AlertOnMessageDelivery,
            BroadcastRequestTlvTag::BroadcastChannelIndicator => TlvTag::BroadcastChannelIndicator,
            BroadcastRequestTlvTag::BroadcastContentTypeInfo => TlvTag::BroadcastContentTypeInfo,
            BroadcastRequestTlvTag::BroadcastMessageClass => TlvTag::BroadcastMessageClass,
            BroadcastRequestTlvTag::BroadcastServiceGroup => TlvTag::BroadcastServiceGroup,
            BroadcastRequestTlvTag::CallbackNum => TlvTag::CallbackNum,
            BroadcastRequestTlvTag::CallbackNumAtag => TlvTag::CallbackNumAtag,
            BroadcastRequestTlvTag::CallbackNumPresInd => TlvTag::CallbackNumPresInd,
            BroadcastRequestTlvTag::DestAddrSubunit => TlvTag::DestAddrSubunit,
            BroadcastRequestTlvTag::DestSubaddress => TlvTag::DestSubaddress,
            BroadcastRequestTlvTag::DestPort => TlvTag::DestPort,
            BroadcastRequestTlvTag::DisplayTime => TlvTag::DisplayTime,
            BroadcastRequestTlvTag::LanguageIndicator => TlvTag::LanguageIndicator,
            BroadcastRequestTlvTag::MessagePayload => TlvTag::MessagePayload,
            BroadcastRequestTlvTag::MsValidity => TlvTag::MsValidity,
            BroadcastRequestTlvTag::PayloadType => TlvTag::PayloadType,
            BroadcastRequestTlvTag::PrivacyIndicator => TlvTag::PrivacyIndicator,
            BroadcastRequestTlvTag::SmsSignal => TlvTag::SmsSignal,
            BroadcastRequestTlvTag::SourceAddrSubunit => TlvTag::SourceAddrSubunit,
            BroadcastRequestTlvTag::SourcePort => TlvTag::SourcePort,
            BroadcastRequestTlvTag::SourceSubaddress => TlvTag::SourceSubaddress,
            BroadcastRequestTlvTag::UserMessageReference => TlvTag::UserMessageReference,
            BroadcastRequestTlvTag::Other(other) => TlvTag::Other(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastRequestTlvValue {
    AlertOnMsgDelivery(AlertOnMsgDelivery),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    BroadcastContentTypeInfo(OctetString<0, 255>),
    BroadcastMessageClass(BroadcastMessageClass),
    BroadcastServiceGroup(OctetString<1, 255>),
    CallbackNum(OctetString<4, 19>),
    CallbackNumAtag(OctetString<0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrSubunit(AddrSubunit),
    DestSubaddress(Subaddress),
    DestPort(u16),
    DisplayTime(DisplayTime),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload),
    MsValidity(MsValidity),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    UserMessageReference(UserMessageReference),
}

impl From<BroadcastRequestTlvValue> for TlvValue {
    fn from(value: BroadcastRequestTlvValue) -> Self {
        match value {
            BroadcastRequestTlvValue::AlertOnMsgDelivery(value) => {
                TlvValue::AlertOnMessageDelivery(value)
            }
            BroadcastRequestTlvValue::BroadcastChannelIndicator(value) => {
                TlvValue::BroadcastChannelIndicator(value)
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(value) => {
                TlvValue::BroadcastContentTypeInfo(value)
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(value) => {
                TlvValue::BroadcastMessageClass(value)
            }
            BroadcastRequestTlvValue::BroadcastServiceGroup(value) => {
                TlvValue::BroadcastServiceGroup(value)
            }
            BroadcastRequestTlvValue::CallbackNum(value) => TlvValue::CallbackNum(value),
            BroadcastRequestTlvValue::CallbackNumAtag(value) => TlvValue::CallbackNumAtag(value),
            BroadcastRequestTlvValue::CallbackNumPresInd(value) => {
                TlvValue::CallbackNumPresInd(value)
            }
            BroadcastRequestTlvValue::DestAddrSubunit(value) => TlvValue::DestAddrSubunit(value),
            BroadcastRequestTlvValue::DestSubaddress(value) => TlvValue::DestSubaddress(value),
            BroadcastRequestTlvValue::DestPort(value) => TlvValue::DestPort(value),
            BroadcastRequestTlvValue::DisplayTime(value) => TlvValue::DisplayTime(value),
            BroadcastRequestTlvValue::LanguageIndicator(value) => {
                TlvValue::LanguageIndicator(value)
            }
            BroadcastRequestTlvValue::MessagePayload(value) => TlvValue::MessagePayload(value),
            BroadcastRequestTlvValue::MsValidity(value) => TlvValue::MsValidity(value),
            BroadcastRequestTlvValue::PayloadType(value) => TlvValue::PayloadType(value),
            BroadcastRequestTlvValue::PrivacyIndicator(value) => TlvValue::PrivacyIndicator(value),
            BroadcastRequestTlvValue::SmsSignal(value) => TlvValue::SmsSignal(value),
            BroadcastRequestTlvValue::SourceAddrSubunit(value) => {
                TlvValue::SourceAddrSubunit(value)
            }
            BroadcastRequestTlvValue::SourcePort(value) => TlvValue::SourcePort(value),
            BroadcastRequestTlvValue::SourceSubaddress(value) => TlvValue::SourceSubaddress(value),
            BroadcastRequestTlvValue::UserMessageReference(value) => {
                TlvValue::UserMessageReference(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastRequestTlv {
    tlv: Tlv,
}

impl BroadcastRequestTlv {
    pub fn new(value: BroadcastRequestTlvValue) -> Self {
        let value = TlvValue::from(value);
        let tlv = Tlv::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: BroadcastRequestTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        let tlv = Tlv::from(tag);

        Self { tlv }
    }
}

impl From<BroadcastRequestTlvTag> for Tlv {
    fn from(tag: BroadcastRequestTlvTag) -> Self {
        let tag = TlvTag::from(tag);
        Tlv::from(tag)
    }
}

impl From<BroadcastRequestTlvValue> for BroadcastRequestTlv {
    fn from(value: BroadcastRequestTlvValue) -> Self {
        Self::new(value)
    }
}

impl From<BroadcastRequestTlvValue> for Tlv {
    fn from(value: BroadcastRequestTlvValue) -> Self {
        let value = TlvValue::from(value);
        Tlv::from(value)
    }
}

impl From<BroadcastRequestTlv> for Tlv {
    fn from(tlv: BroadcastRequestTlv) -> Self {
        tlv.tlv
    }
}
