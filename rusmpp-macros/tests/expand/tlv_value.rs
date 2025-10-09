/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::TlvValue)]
pub enum BroadcastRequestTlvValue {
    /// Docs
    ///
    /// More docs
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

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::TlvValue)]
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
