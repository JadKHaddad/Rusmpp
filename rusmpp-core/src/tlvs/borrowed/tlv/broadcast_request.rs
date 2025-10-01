use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::OctetString,
    values::{
        addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMessageDelivery,
        broadcast_area_identifier::borrowed::BroadcastAreaIdentifier,
        broadcast_channel_indicator::BroadcastChannelIndicator,
        broadcast_content_type::BroadcastContentType,
        broadcast_frequency_interval::BroadcastFrequencyInterval,
        broadcast_message_class::BroadcastMessageClass, broadcast_rep_num::BroadcastRepNum,
        callback_num_pres_ind::CallbackNumPresInd, display_time::DisplayTime,
        language_indicator::LanguageIndicator, message_payload::borrowed::MessagePayload,
        ms_validity::MsValidity, payload_type::PayloadType, privacy_indicator::PrivacyIndicator,
        sub_address::borrowed::Subaddress, user_message_reference::UserMessageReference,
    },
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum BroadcastRequestTlvValue<'a> {
    BroadcastAreaIdentifier(BroadcastAreaIdentifier<'a>),
    BroadcastContentType(BroadcastContentType),
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastRepNum(BroadcastRepNum),
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    BroadcastContentTypeInfo(OctetString<'a, 0, 255>),
    BroadcastMessageClass(BroadcastMessageClass),
    BroadcastServiceGroup(OctetString<'a, 1, 255>),
    CallbackNum(OctetString<'a, 4, 19>),
    CallbackNumAtag(OctetString<'a, 0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrSubunit(AddrSubunit),
    DestSubaddress(Subaddress<'a>),
    DestPort(u16),
    DisplayTime(DisplayTime),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload<'a>),
    MsValidity(MsValidity),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourcePort(u16),
    SourceSubaddress(Subaddress<'a>),
    UserMessageReference(UserMessageReference),
}
