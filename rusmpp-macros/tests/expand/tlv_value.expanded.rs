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
    pub const fn tag(&self) -> TlvTag {
        match self {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(_) => {
                TlvTag::AlertOnMessageDelivery
            }
            BroadcastRequestTlvValue::BroadcastChannelIndicator(_) => {
                TlvTag::BroadcastChannelIndicator
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(_) => {
                TlvTag::BroadcastContentTypeInfo
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(_) => {
                TlvTag::BroadcastMessageClass
            }
            BroadcastRequestTlvValue::BroadcastServiceGroup(_) => {
                TlvTag::BroadcastServiceGroup
            }
            BroadcastRequestTlvValue::CallbackNum(_) => TlvTag::CallbackNum,
            BroadcastRequestTlvValue::CallbackNumAtag(_) => TlvTag::CallbackNumAtag,
            BroadcastRequestTlvValue::CallbackNumPresInd(_) => TlvTag::CallbackNumPresInd,
            BroadcastRequestTlvValue::DestAddrSubunit(_) => TlvTag::DestAddrSubunit,
            BroadcastRequestTlvValue::DestSubaddress(_) => TlvTag::DestSubaddress,
            BroadcastRequestTlvValue::DestPort(_) => TlvTag::DestPort,
            BroadcastRequestTlvValue::DisplayTime(_) => TlvTag::DisplayTime,
            BroadcastRequestTlvValue::LanguageIndicator(_) => TlvTag::LanguageIndicator,
            BroadcastRequestTlvValue::MessagePayload(_) => TlvTag::MessagePayload,
            BroadcastRequestTlvValue::MsValidity(_) => TlvTag::MsValidity,
            BroadcastRequestTlvValue::PayloadType(_) => TlvTag::PayloadType,
            BroadcastRequestTlvValue::PrivacyIndicator(_) => TlvTag::PrivacyIndicator,
            BroadcastRequestTlvValue::SmsSignal(_) => TlvTag::SmsSignal,
            BroadcastRequestTlvValue::SourceAddrSubunit(_) => TlvTag::SourceAddrSubunit,
            BroadcastRequestTlvValue::SourcePort(_) => TlvTag::SourcePort,
            BroadcastRequestTlvValue::SourceSubaddress(_) => TlvTag::SourceSubaddress,
            BroadcastRequestTlvValue::UserMessageReference(_) => {
                TlvTag::UserMessageReference
            }
        }
    }
}
impl From<BroadcastRequestTlvValue> for TlvValue {
    fn from(value: BroadcastRequestTlvValue) -> Self {
        match value {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(value) => {
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
            BroadcastRequestTlvValue::CallbackNumAtag(value) => {
                TlvValue::CallbackNumAtag(value)
            }
            BroadcastRequestTlvValue::CallbackNumPresInd(value) => {
                TlvValue::CallbackNumPresInd(value)
            }
            BroadcastRequestTlvValue::DestAddrSubunit(value) => {
                TlvValue::DestAddrSubunit(value)
            }
            BroadcastRequestTlvValue::DestSubaddress(value) => {
                TlvValue::DestSubaddress(value)
            }
            BroadcastRequestTlvValue::DestPort(value) => TlvValue::DestPort(value),
            BroadcastRequestTlvValue::DisplayTime(value) => TlvValue::DisplayTime(value),
            BroadcastRequestTlvValue::LanguageIndicator(value) => {
                TlvValue::LanguageIndicator(value)
            }
            BroadcastRequestTlvValue::MessagePayload(value) => {
                TlvValue::MessagePayload(value)
            }
            BroadcastRequestTlvValue::MsValidity(value) => TlvValue::MsValidity(value),
            BroadcastRequestTlvValue::PayloadType(value) => TlvValue::PayloadType(value),
            BroadcastRequestTlvValue::PrivacyIndicator(value) => {
                TlvValue::PrivacyIndicator(value)
            }
            BroadcastRequestTlvValue::SmsSignal(value) => TlvValue::SmsSignal(value),
            BroadcastRequestTlvValue::SourceAddrSubunit(value) => {
                TlvValue::SourceAddrSubunit(value)
            }
            BroadcastRequestTlvValue::SourcePort(value) => TlvValue::SourcePort(value),
            BroadcastRequestTlvValue::SourceSubaddress(value) => {
                TlvValue::SourceSubaddress(value)
            }
            BroadcastRequestTlvValue::UserMessageReference(value) => {
                TlvValue::UserMessageReference(value)
            }
        }
    }
}
impl From<BroadcastRequestTlvValue> for Tlv {
    fn from(value: BroadcastRequestTlvValue) -> Self {
        Self::new(TlvValue::from(value))
    }
}
/// Docs
///
/// More docs
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
#[automatically_derived]
impl<'a> ::core::fmt::Debug for BroadcastRequestTlvValue<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            BroadcastRequestTlvValue::BroadcastAreaIdentifier(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastAreaIdentifier",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::BroadcastContentType(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastContentType",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::BroadcastFrequencyInterval(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastFrequencyInterval",
                    &__self_0,
                )
            }
            BroadcastRequestTlvValue::BroadcastRepNum(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BroadcastRepNum",
                    &__self_0,
                )
            }
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
impl<'a> BroadcastRequestTlvValue<'a> {
    pub const fn tag(&self) -> TlvTag {
        match self {
            BroadcastRequestTlvValue::BroadcastAreaIdentifier(_) => {
                TlvTag::BroadcastAreaIdentifier
            }
            BroadcastRequestTlvValue::BroadcastContentType(_) => {
                TlvTag::BroadcastContentType
            }
            BroadcastRequestTlvValue::BroadcastFrequencyInterval(_) => {
                TlvTag::BroadcastFrequencyInterval
            }
            BroadcastRequestTlvValue::BroadcastRepNum(_) => TlvTag::BroadcastRepNum,
            BroadcastRequestTlvValue::AlertOnMessageDelivery(_) => {
                TlvTag::AlertOnMessageDelivery
            }
            BroadcastRequestTlvValue::BroadcastChannelIndicator(_) => {
                TlvTag::BroadcastChannelIndicator
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(_) => {
                TlvTag::BroadcastContentTypeInfo
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(_) => {
                TlvTag::BroadcastMessageClass
            }
            BroadcastRequestTlvValue::BroadcastServiceGroup(_) => {
                TlvTag::BroadcastServiceGroup
            }
            BroadcastRequestTlvValue::CallbackNum(_) => TlvTag::CallbackNum,
            BroadcastRequestTlvValue::CallbackNumAtag(_) => TlvTag::CallbackNumAtag,
            BroadcastRequestTlvValue::CallbackNumPresInd(_) => TlvTag::CallbackNumPresInd,
            BroadcastRequestTlvValue::DestAddrSubunit(_) => TlvTag::DestAddrSubunit,
            BroadcastRequestTlvValue::DestSubaddress(_) => TlvTag::DestSubaddress,
            BroadcastRequestTlvValue::DestPort(_) => TlvTag::DestPort,
            BroadcastRequestTlvValue::DisplayTime(_) => TlvTag::DisplayTime,
            BroadcastRequestTlvValue::LanguageIndicator(_) => TlvTag::LanguageIndicator,
            BroadcastRequestTlvValue::MessagePayload(_) => TlvTag::MessagePayload,
            BroadcastRequestTlvValue::MsValidity(_) => TlvTag::MsValidity,
            BroadcastRequestTlvValue::PayloadType(_) => TlvTag::PayloadType,
            BroadcastRequestTlvValue::PrivacyIndicator(_) => TlvTag::PrivacyIndicator,
            BroadcastRequestTlvValue::SmsSignal(_) => TlvTag::SmsSignal,
            BroadcastRequestTlvValue::SourceAddrSubunit(_) => TlvTag::SourceAddrSubunit,
            BroadcastRequestTlvValue::SourcePort(_) => TlvTag::SourcePort,
            BroadcastRequestTlvValue::SourceSubaddress(_) => TlvTag::SourceSubaddress,
            BroadcastRequestTlvValue::UserMessageReference(_) => {
                TlvTag::UserMessageReference
            }
        }
    }
}
impl<'a> From<BroadcastRequestTlvValue<'a>> for TlvValue<'a> {
    fn from(value: BroadcastRequestTlvValue<'a>) -> Self {
        match value {
            BroadcastRequestTlvValue::BroadcastAreaIdentifier(value) => {
                TlvValue::BroadcastAreaIdentifier(value)
            }
            BroadcastRequestTlvValue::BroadcastContentType(value) => {
                TlvValue::BroadcastContentType(value)
            }
            BroadcastRequestTlvValue::BroadcastFrequencyInterval(value) => {
                TlvValue::BroadcastFrequencyInterval(value)
            }
            BroadcastRequestTlvValue::BroadcastRepNum(value) => {
                TlvValue::BroadcastRepNum(value)
            }
            BroadcastRequestTlvValue::AlertOnMessageDelivery(value) => {
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
            BroadcastRequestTlvValue::CallbackNumAtag(value) => {
                TlvValue::CallbackNumAtag(value)
            }
            BroadcastRequestTlvValue::CallbackNumPresInd(value) => {
                TlvValue::CallbackNumPresInd(value)
            }
            BroadcastRequestTlvValue::DestAddrSubunit(value) => {
                TlvValue::DestAddrSubunit(value)
            }
            BroadcastRequestTlvValue::DestSubaddress(value) => {
                TlvValue::DestSubaddress(value)
            }
            BroadcastRequestTlvValue::DestPort(value) => TlvValue::DestPort(value),
            BroadcastRequestTlvValue::DisplayTime(value) => TlvValue::DisplayTime(value),
            BroadcastRequestTlvValue::LanguageIndicator(value) => {
                TlvValue::LanguageIndicator(value)
            }
            BroadcastRequestTlvValue::MessagePayload(value) => {
                TlvValue::MessagePayload(value)
            }
            BroadcastRequestTlvValue::MsValidity(value) => TlvValue::MsValidity(value),
            BroadcastRequestTlvValue::PayloadType(value) => TlvValue::PayloadType(value),
            BroadcastRequestTlvValue::PrivacyIndicator(value) => {
                TlvValue::PrivacyIndicator(value)
            }
            BroadcastRequestTlvValue::SmsSignal(value) => TlvValue::SmsSignal(value),
            BroadcastRequestTlvValue::SourceAddrSubunit(value) => {
                TlvValue::SourceAddrSubunit(value)
            }
            BroadcastRequestTlvValue::SourcePort(value) => TlvValue::SourcePort(value),
            BroadcastRequestTlvValue::SourceSubaddress(value) => {
                TlvValue::SourceSubaddress(value)
            }
            BroadcastRequestTlvValue::UserMessageReference(value) => {
                TlvValue::UserMessageReference(value)
            }
        }
    }
}
impl<'a> From<BroadcastRequestTlvValue<'a>> for Tlv<'a> {
    fn from(value: BroadcastRequestTlvValue<'a>) -> Self {
        Self::new(TlvValue::from(value))
    }
}
