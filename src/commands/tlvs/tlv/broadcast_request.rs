use super::TLV;
use crate::{
    commands::{
        tlvs::{tlv_tag::TLVTag, tlv_value::TLVValue},
        types::{
            addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMsgDelivery,
            broadcast_channel_indicator::BroadcastChannelIndicator,
            broadcast_message_class::BroadcastMessageClass,
            callback_num_pres_ind::CallbackNumPresInd, display_time::DisplayTime,
            language_indicator::LanguageIndicator, ms_validity::MsValidity,
            payload_type::PayloadType, privacy_indicator::PrivacyIndicator, subaddress::Subaddress,
        },
    },
    types::{no_fixed_size_octet_string::NoFixedSizeOctetString, octet_string::OctetString},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastRequestTLVTag {
    AlertOnMsgDelivery,
    BroadcastChannelIndicator,
    BroadcastContentTypeInfo,
    BroadcastMessageClass,
    BroadcastServiceGroup,
    CallbackNum,
    CallbackNumAtag,
    CallbackNumPresInd,
    DestAddrSubunit,
    DestSubaddress,
    DestPort,
    DisplayTime,
    LanguageIndicator,
    MessagePayload,
    MsValidity,
    PayloadType,
    PrivacyIndicator,
    SmsSignal,
    SourceAddrSubunit,
    SourcePort,
    SourceSubaddress,
    UserMessageReference,
}

impl From<BroadcastRequestTLVTag> for TLVTag {
    fn from(v: BroadcastRequestTLVTag) -> Self {
        match v {
            BroadcastRequestTLVTag::AlertOnMsgDelivery => TLVTag::AlertOnMessageDelivery,
            BroadcastRequestTLVTag::BroadcastChannelIndicator => TLVTag::BroadcastChannelIndicator,
            BroadcastRequestTLVTag::BroadcastContentTypeInfo => TLVTag::BroadcastContentTypeInfo,
            BroadcastRequestTLVTag::BroadcastMessageClass => TLVTag::BroadcastMessageClass,
            BroadcastRequestTLVTag::BroadcastServiceGroup => TLVTag::BroadcastServiceGroup,
            BroadcastRequestTLVTag::CallbackNum => TLVTag::CallbackNum,
            BroadcastRequestTLVTag::CallbackNumAtag => TLVTag::CallbackNumAtag,
            BroadcastRequestTLVTag::CallbackNumPresInd => TLVTag::CallbackNumPresInd,
            BroadcastRequestTLVTag::DestAddrSubunit => TLVTag::DestAddrSubunit,
            BroadcastRequestTLVTag::DestSubaddress => TLVTag::DestSubaddress,
            BroadcastRequestTLVTag::DestPort => TLVTag::DestPort,
            BroadcastRequestTLVTag::DisplayTime => TLVTag::DisplayTime,
            BroadcastRequestTLVTag::LanguageIndicator => TLVTag::LanguageIndicator,
            BroadcastRequestTLVTag::MessagePayload => TLVTag::MessagePayload,
            BroadcastRequestTLVTag::MsValidity => TLVTag::MsValidity,
            BroadcastRequestTLVTag::PayloadType => TLVTag::PayloadType,
            BroadcastRequestTLVTag::PrivacyIndicator => TLVTag::PrivacyIndicator,
            BroadcastRequestTLVTag::SmsSignal => TLVTag::SmsSignal,
            BroadcastRequestTLVTag::SourceAddrSubunit => TLVTag::SourceAddrSubunit,
            BroadcastRequestTLVTag::SourcePort => TLVTag::SourcePort,
            BroadcastRequestTLVTag::SourceSubaddress => TLVTag::SourceSubaddress,
            BroadcastRequestTLVTag::UserMessageReference => TLVTag::UserMessageReference,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BroadcastRequestTLVValue {
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
    MessagePayload(NoFixedSizeOctetString),
    MsValidity(MsValidity),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    UserMessageReference(u16),
}

impl From<BroadcastRequestTLVValue> for TLVValue {
    fn from(value: BroadcastRequestTLVValue) -> Self {
        match value {
            BroadcastRequestTLVValue::AlertOnMsgDelivery(value) => {
                TLVValue::AlertOnMessageDelivery(value)
            }
            BroadcastRequestTLVValue::BroadcastChannelIndicator(value) => {
                TLVValue::BroadcastChannelIndicator(value)
            }
            BroadcastRequestTLVValue::BroadcastContentTypeInfo(value) => {
                TLVValue::BroadcastContentTypeInfo(value)
            }
            BroadcastRequestTLVValue::BroadcastMessageClass(value) => {
                TLVValue::BroadcastMessageClass(value)
            }
            BroadcastRequestTLVValue::BroadcastServiceGroup(value) => {
                TLVValue::BroadcastServiceGroup(value)
            }
            BroadcastRequestTLVValue::CallbackNum(value) => TLVValue::CallbackNum(value),
            BroadcastRequestTLVValue::CallbackNumAtag(value) => TLVValue::CallbackNumAtag(value),
            BroadcastRequestTLVValue::CallbackNumPresInd(value) => {
                TLVValue::CallbackNumPresInd(value)
            }
            BroadcastRequestTLVValue::DestAddrSubunit(value) => TLVValue::DestAddrSubunit(value),
            BroadcastRequestTLVValue::DestSubaddress(value) => TLVValue::DestSubaddress(value),
            BroadcastRequestTLVValue::DestPort(value) => TLVValue::DestPort(value),
            BroadcastRequestTLVValue::DisplayTime(value) => TLVValue::DisplayTime(value),
            BroadcastRequestTLVValue::LanguageIndicator(value) => {
                TLVValue::LanguageIndicator(value)
            }
            BroadcastRequestTLVValue::MessagePayload(value) => TLVValue::MessagePayload(value),
            BroadcastRequestTLVValue::MsValidity(value) => TLVValue::MsValidity(value),
            BroadcastRequestTLVValue::PayloadType(value) => TLVValue::PayloadType(value),
            BroadcastRequestTLVValue::PrivacyIndicator(value) => TLVValue::PrivacyIndicator(value),
            BroadcastRequestTLVValue::SmsSignal(value) => TLVValue::SmsSignal(value),
            BroadcastRequestTLVValue::SourceAddrSubunit(value) => {
                TLVValue::SourceAddrSubunit(value)
            }
            BroadcastRequestTLVValue::SourcePort(value) => TLVValue::SourcePort(value),
            BroadcastRequestTLVValue::SourceSubaddress(value) => TLVValue::SourceSubaddress(value),
            BroadcastRequestTLVValue::UserMessageReference(value) => {
                TLVValue::UserMessageReference(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastRequestTLV {
    tlv: TLV,
}

impl BroadcastRequestTLV {
    pub fn new(value: BroadcastRequestTLVValue) -> Self {
        let value = TLVValue::from(value);
        let tlv = TLV::from(value);

        Self { tlv }
    }

    pub fn without_value(tag: BroadcastRequestTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        let tlv = TLV::from(tag);

        Self { tlv }
    }
}

impl From<BroadcastRequestTLVTag> for TLV {
    fn from(tag: BroadcastRequestTLVTag) -> Self {
        let tag = TLVTag::from(tag);
        TLV::from(tag)
    }
}

impl From<BroadcastRequestTLVValue> for BroadcastRequestTLV {
    fn from(value: BroadcastRequestTLVValue) -> Self {
        Self::new(value)
    }
}

impl From<BroadcastRequestTLVValue> for TLV {
    fn from(value: BroadcastRequestTLVValue) -> Self {
        let value = TLVValue::from(value);
        TLV::from(value)
    }
}

impl From<BroadcastRequestTLV> for TLV {
    fn from(tlv: BroadcastRequestTLV) -> Self {
        tlv.tlv
    }
}
