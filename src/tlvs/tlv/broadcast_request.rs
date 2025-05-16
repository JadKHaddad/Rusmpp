use crate::{
    commands::types::{
        addr_subunit::AddrSubunit, alert_on_msg_delivery::AlertOnMessageDelivery,
        broadcast_channel_indicator::BroadcastChannelIndicator,
        broadcast_message_class::BroadcastMessageClass, callback_num_pres_ind::CallbackNumPresInd,
        display_time::DisplayTime, language_indicator::LanguageIndicator, ms_validity::MsValidity,
        payload_type::PayloadType, privacy_indicator::PrivacyIndicator, sub_address::Subaddress,
        BroadcastAreaIdentifier, BroadcastContentType, BroadcastFrequencyInterval, BroadcastRepNum,
        MessagePayload, UserMessageReference,
    },
    types::OctetString,
};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BroadcastRequestTlvValue {
        BroadcastAreaIdentifier(BroadcastAreaIdentifier),
        BroadcastContentType(BroadcastContentType),
        BroadcastFrequencyInterval(BroadcastFrequencyInterval),
        BroadcastRepNum(BroadcastRepNum),
        AlertOnMessageDelivery(AlertOnMessageDelivery),
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
}
