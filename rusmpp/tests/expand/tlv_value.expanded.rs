/// Docs
///
/// More docs
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
#[automatically_derived]
impl ::core::fmt::Debug for BroadcastRequestTlvValue {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "AlertOnMessageDelivery",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::BroadcastChannelIndicator(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastChannelIndicator",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastContentTypeInfo",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastMessageClass",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::BroadcastServiceGroup(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastServiceGroup",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::CallbackNum(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "CallbackNum",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::CallbackNumAtag(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "CallbackNumAtag",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::CallbackNumPresInd(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "CallbackNumPresInd",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::DestAddrSubunit(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "DestAddrSubunit",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::DestSubaddress(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "DestSubaddress",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::DestPort(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "DestPort",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::DisplayTime(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "DisplayTime",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::LanguageIndicator(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "LanguageIndicator",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::MessagePayload(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "MessagePayload",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::MsValidity(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "MsValidity",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::PayloadType(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "PayloadType",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::PrivacyIndicator(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "PrivacyIndicator",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::SmsSignal(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SmsSignal",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::SourceAddrSubunit(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SourceAddrSubunit",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::SourcePort(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SourcePort",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::SourceSubaddress(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SourceSubaddress",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::UserMessageReference(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "UserMessageReference",
                    &__self_0,
                )
            }
        }
    }
}
impl BroadcastRequestTlvValue {
    pub const fn tag(&self) -> ::rusmpp::tlvs::TlvTag {
        match self {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(_) => {
                ::rusmpp::tlvs::TlvTag::AlertOnMessageDelivery
            }
            BroadcastRequestTlvValue::BroadcastChannelIndicator(_) => {
                ::rusmpp::tlvs::TlvTag::BroadcastChannelIndicator
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(_) => {
                ::rusmpp::tlvs::TlvTag::BroadcastContentTypeInfo
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(_) => {
                ::rusmpp::tlvs::TlvTag::BroadcastMessageClass
            }
            BroadcastRequestTlvValue::BroadcastServiceGroup(_) => {
                ::rusmpp::tlvs::TlvTag::BroadcastServiceGroup
            }
            BroadcastRequestTlvValue::CallbackNum(_) => {
                ::rusmpp::tlvs::TlvTag::CallbackNum
            }
            BroadcastRequestTlvValue::CallbackNumAtag(_) => {
                ::rusmpp::tlvs::TlvTag::CallbackNumAtag
            }
            BroadcastRequestTlvValue::CallbackNumPresInd(_) => {
                ::rusmpp::tlvs::TlvTag::CallbackNumPresInd
            }
            BroadcastRequestTlvValue::DestAddrSubunit(_) => {
                ::rusmpp::tlvs::TlvTag::DestAddrSubunit
            }
            BroadcastRequestTlvValue::DestSubaddress(_) => {
                ::rusmpp::tlvs::TlvTag::DestSubaddress
            }
            BroadcastRequestTlvValue::DestPort(_) => ::rusmpp::tlvs::TlvTag::DestPort,
            BroadcastRequestTlvValue::DisplayTime(_) => {
                ::rusmpp::tlvs::TlvTag::DisplayTime
            }
            BroadcastRequestTlvValue::LanguageIndicator(_) => {
                ::rusmpp::tlvs::TlvTag::LanguageIndicator
            }
            BroadcastRequestTlvValue::MessagePayload(_) => {
                ::rusmpp::tlvs::TlvTag::MessagePayload
            }
            BroadcastRequestTlvValue::MsValidity(_) => ::rusmpp::tlvs::TlvTag::MsValidity,
            BroadcastRequestTlvValue::PayloadType(_) => {
                ::rusmpp::tlvs::TlvTag::PayloadType
            }
            BroadcastRequestTlvValue::PrivacyIndicator(_) => {
                ::rusmpp::tlvs::TlvTag::PrivacyIndicator
            }
            BroadcastRequestTlvValue::SmsSignal(_) => ::rusmpp::tlvs::TlvTag::SmsSignal,
            BroadcastRequestTlvValue::SourceAddrSubunit(_) => {
                ::rusmpp::tlvs::TlvTag::SourceAddrSubunit
            }
            BroadcastRequestTlvValue::SourcePort(_) => ::rusmpp::tlvs::TlvTag::SourcePort,
            BroadcastRequestTlvValue::SourceSubaddress(_) => {
                ::rusmpp::tlvs::TlvTag::SourceSubaddress
            }
            BroadcastRequestTlvValue::UserMessageReference(_) => {
                ::rusmpp::tlvs::TlvTag::UserMessageReference
            }
        }
    }
}
impl From<BroadcastRequestTlvValue> for ::rusmpp::tlvs::TlvValue {
    fn from(value: BroadcastRequestTlvValue) -> Self {
        match value {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(value) => {
                ::rusmpp::tlvs::TlvValue::AlertOnMessageDelivery(value)
            }
            BroadcastRequestTlvValue::BroadcastChannelIndicator(value) => {
                ::rusmpp::tlvs::TlvValue::BroadcastChannelIndicator(value)
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(value) => {
                ::rusmpp::tlvs::TlvValue::BroadcastContentTypeInfo(value)
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(value) => {
                ::rusmpp::tlvs::TlvValue::BroadcastMessageClass(value)
            }
            BroadcastRequestTlvValue::BroadcastServiceGroup(value) => {
                ::rusmpp::tlvs::TlvValue::BroadcastServiceGroup(value)
            }
            BroadcastRequestTlvValue::CallbackNum(value) => {
                ::rusmpp::tlvs::TlvValue::CallbackNum(value)
            }
            BroadcastRequestTlvValue::CallbackNumAtag(value) => {
                ::rusmpp::tlvs::TlvValue::CallbackNumAtag(value)
            }
            BroadcastRequestTlvValue::CallbackNumPresInd(value) => {
                ::rusmpp::tlvs::TlvValue::CallbackNumPresInd(value)
            }
            BroadcastRequestTlvValue::DestAddrSubunit(value) => {
                ::rusmpp::tlvs::TlvValue::DestAddrSubunit(value)
            }
            BroadcastRequestTlvValue::DestSubaddress(value) => {
                ::rusmpp::tlvs::TlvValue::DestSubaddress(value)
            }
            BroadcastRequestTlvValue::DestPort(value) => {
                ::rusmpp::tlvs::TlvValue::DestPort(value)
            }
            BroadcastRequestTlvValue::DisplayTime(value) => {
                ::rusmpp::tlvs::TlvValue::DisplayTime(value)
            }
            BroadcastRequestTlvValue::LanguageIndicator(value) => {
                ::rusmpp::tlvs::TlvValue::LanguageIndicator(value)
            }
            BroadcastRequestTlvValue::MessagePayload(value) => {
                ::rusmpp::tlvs::TlvValue::MessagePayload(value)
            }
            BroadcastRequestTlvValue::MsValidity(value) => {
                ::rusmpp::tlvs::TlvValue::MsValidity(value)
            }
            BroadcastRequestTlvValue::PayloadType(value) => {
                ::rusmpp::tlvs::TlvValue::PayloadType(value)
            }
            BroadcastRequestTlvValue::PrivacyIndicator(value) => {
                ::rusmpp::tlvs::TlvValue::PrivacyIndicator(value)
            }
            BroadcastRequestTlvValue::SmsSignal(value) => {
                ::rusmpp::tlvs::TlvValue::SmsSignal(value)
            }
            BroadcastRequestTlvValue::SourceAddrSubunit(value) => {
                ::rusmpp::tlvs::TlvValue::SourceAddrSubunit(value)
            }
            BroadcastRequestTlvValue::SourcePort(value) => {
                ::rusmpp::tlvs::TlvValue::SourcePort(value)
            }
            BroadcastRequestTlvValue::SourceSubaddress(value) => {
                ::rusmpp::tlvs::TlvValue::SourceSubaddress(value)
            }
            BroadcastRequestTlvValue::UserMessageReference(value) => {
                ::rusmpp::tlvs::TlvValue::UserMessageReference(value)
            }
        }
    }
}
impl From<BroadcastRequestTlvValue> for ::rusmpp::tlvs::Tlv {
    fn from(value: BroadcastRequestTlvValue) -> Self {
        Self::new(::rusmpp::tlvs::TlvValue::from(value))
    }
}
