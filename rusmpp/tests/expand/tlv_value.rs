rusmpp::create_tlv_value! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
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
}
