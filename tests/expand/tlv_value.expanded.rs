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
    Other { tag: BroadcastRequestTlvTag, value: AnyOctetString },
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
            BroadcastRequestTlvValue::Other { tag: __self_0, value: __self_1 } => {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Other",
                    "tag",
                    __self_0,
                    "value",
                    &__self_1,
                )
            }
        }
    }
}
impl BroadcastRequestTlvValue {
    pub const fn tag(&self) -> BroadcastRequestTlvTag {
        match self {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(_) => {
                BroadcastRequestTlvTag::AlertOnMessageDelivery
            }
            BroadcastRequestTlvValue::BroadcastChannelIndicator(_) => {
                BroadcastRequestTlvTag::BroadcastChannelIndicator
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(_) => {
                BroadcastRequestTlvTag::BroadcastContentTypeInfo
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(_) => {
                BroadcastRequestTlvTag::BroadcastMessageClass
            }
            BroadcastRequestTlvValue::BroadcastServiceGroup(_) => {
                BroadcastRequestTlvTag::BroadcastServiceGroup
            }
            BroadcastRequestTlvValue::CallbackNum(_) => {
                BroadcastRequestTlvTag::CallbackNum
            }
            BroadcastRequestTlvValue::CallbackNumAtag(_) => {
                BroadcastRequestTlvTag::CallbackNumAtag
            }
            BroadcastRequestTlvValue::CallbackNumPresInd(_) => {
                BroadcastRequestTlvTag::CallbackNumPresInd
            }
            BroadcastRequestTlvValue::DestAddrSubunit(_) => {
                BroadcastRequestTlvTag::DestAddrSubunit
            }
            BroadcastRequestTlvValue::DestSubaddress(_) => {
                BroadcastRequestTlvTag::DestSubaddress
            }
            BroadcastRequestTlvValue::DestPort(_) => BroadcastRequestTlvTag::DestPort,
            BroadcastRequestTlvValue::DisplayTime(_) => {
                BroadcastRequestTlvTag::DisplayTime
            }
            BroadcastRequestTlvValue::LanguageIndicator(_) => {
                BroadcastRequestTlvTag::LanguageIndicator
            }
            BroadcastRequestTlvValue::MessagePayload(_) => {
                BroadcastRequestTlvTag::MessagePayload
            }
            BroadcastRequestTlvValue::MsValidity(_) => BroadcastRequestTlvTag::MsValidity,
            BroadcastRequestTlvValue::PayloadType(_) => {
                BroadcastRequestTlvTag::PayloadType
            }
            BroadcastRequestTlvValue::PrivacyIndicator(_) => {
                BroadcastRequestTlvTag::PrivacyIndicator
            }
            BroadcastRequestTlvValue::SmsSignal(_) => BroadcastRequestTlvTag::SmsSignal,
            BroadcastRequestTlvValue::SourceAddrSubunit(_) => {
                BroadcastRequestTlvTag::SourceAddrSubunit
            }
            BroadcastRequestTlvValue::SourcePort(_) => BroadcastRequestTlvTag::SourcePort,
            BroadcastRequestTlvValue::SourceSubaddress(_) => {
                BroadcastRequestTlvTag::SourceSubaddress
            }
            BroadcastRequestTlvValue::UserMessageReference(_) => {
                BroadcastRequestTlvTag::UserMessageReference
            }
            BroadcastRequestTlvValue::Other { tag, .. } => *tag,
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
            BroadcastRequestTlvValue::Other { tag, value } => {
                ::rusmpp::tlvs::TlvValue::Other {
                    tag: ::rusmpp::tlvs::TlvTag::from(tag),
                    value,
                }
            }
        }
    }
}
impl ::rusmpp::encode::Length for BroadcastRequestTlvValue {
    fn length(&self) -> usize {
        match self {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(value) => value.length(),
            BroadcastRequestTlvValue::BroadcastChannelIndicator(value) => value.length(),
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(value) => value.length(),
            BroadcastRequestTlvValue::BroadcastMessageClass(value) => value.length(),
            BroadcastRequestTlvValue::BroadcastServiceGroup(value) => value.length(),
            BroadcastRequestTlvValue::CallbackNum(value) => value.length(),
            BroadcastRequestTlvValue::CallbackNumAtag(value) => value.length(),
            BroadcastRequestTlvValue::CallbackNumPresInd(value) => value.length(),
            BroadcastRequestTlvValue::DestAddrSubunit(value) => value.length(),
            BroadcastRequestTlvValue::DestSubaddress(value) => value.length(),
            BroadcastRequestTlvValue::DestPort(value) => value.length(),
            BroadcastRequestTlvValue::DisplayTime(value) => value.length(),
            BroadcastRequestTlvValue::LanguageIndicator(value) => value.length(),
            BroadcastRequestTlvValue::MessagePayload(value) => value.length(),
            BroadcastRequestTlvValue::MsValidity(value) => value.length(),
            BroadcastRequestTlvValue::PayloadType(value) => value.length(),
            BroadcastRequestTlvValue::PrivacyIndicator(value) => value.length(),
            BroadcastRequestTlvValue::SmsSignal(value) => value.length(),
            BroadcastRequestTlvValue::SourceAddrSubunit(value) => value.length(),
            BroadcastRequestTlvValue::SourcePort(value) => value.length(),
            BroadcastRequestTlvValue::SourceSubaddress(value) => value.length(),
            BroadcastRequestTlvValue::UserMessageReference(value) => value.length(),
            BroadcastRequestTlvValue::Other { value, .. } => value.length(),
        }
    }
}
impl ::rusmpp::encode::Encode for BroadcastRequestTlvValue {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            BroadcastRequestTlvValue::AlertOnMessageDelivery(value) => value.encode(dst),
            BroadcastRequestTlvValue::BroadcastChannelIndicator(value) => {
                value.encode(dst)
            }
            BroadcastRequestTlvValue::BroadcastContentTypeInfo(value) => {
                value.encode(dst)
            }
            BroadcastRequestTlvValue::BroadcastMessageClass(value) => value.encode(dst),
            BroadcastRequestTlvValue::BroadcastServiceGroup(value) => value.encode(dst),
            BroadcastRequestTlvValue::CallbackNum(value) => value.encode(dst),
            BroadcastRequestTlvValue::CallbackNumAtag(value) => value.encode(dst),
            BroadcastRequestTlvValue::CallbackNumPresInd(value) => value.encode(dst),
            BroadcastRequestTlvValue::DestAddrSubunit(value) => value.encode(dst),
            BroadcastRequestTlvValue::DestSubaddress(value) => value.encode(dst),
            BroadcastRequestTlvValue::DestPort(value) => value.encode(dst),
            BroadcastRequestTlvValue::DisplayTime(value) => value.encode(dst),
            BroadcastRequestTlvValue::LanguageIndicator(value) => value.encode(dst),
            BroadcastRequestTlvValue::MessagePayload(value) => value.encode(dst),
            BroadcastRequestTlvValue::MsValidity(value) => value.encode(dst),
            BroadcastRequestTlvValue::PayloadType(value) => value.encode(dst),
            BroadcastRequestTlvValue::PrivacyIndicator(value) => value.encode(dst),
            BroadcastRequestTlvValue::SmsSignal(value) => value.encode(dst),
            BroadcastRequestTlvValue::SourceAddrSubunit(value) => value.encode(dst),
            BroadcastRequestTlvValue::SourcePort(value) => value.encode(dst),
            BroadcastRequestTlvValue::SourceSubaddress(value) => value.encode(dst),
            BroadcastRequestTlvValue::UserMessageReference(value) => value.encode(dst),
            BroadcastRequestTlvValue::Other { value, .. } => value.encode(dst),
        }
    }
}
impl ::rusmpp::decode::DecodeWithKey for BroadcastRequestTlvValue {
    type Key = BroadcastRequestTlvTag;
    fn decode(
        key: Self::Key,
        src: &[u8],
        length: usize,
    ) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        use ::rusmpp::decode::DecodeResultExt;
        let (value, size) = match key {
            BroadcastRequestTlvTag::AlertOnMessageDelivery => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::AlertOnMessageDelivery(value)
                    })?
            }
            BroadcastRequestTlvTag::BroadcastChannelIndicator => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::BroadcastChannelIndicator(value)
                    })?
            }
            BroadcastRequestTlvTag::BroadcastContentTypeInfo => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::BroadcastContentTypeInfo(value)
                    })?
            }
            BroadcastRequestTlvTag::BroadcastMessageClass => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::BroadcastMessageClass(value)
                    })?
            }
            BroadcastRequestTlvTag::BroadcastServiceGroup => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::BroadcastServiceGroup(value)
                    })?
            }
            BroadcastRequestTlvTag::CallbackNum => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::CallbackNum(value)
                    })?
            }
            BroadcastRequestTlvTag::CallbackNumAtag => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::CallbackNumAtag(value)
                    })?
            }
            BroadcastRequestTlvTag::CallbackNumPresInd => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::CallbackNumPresInd(value)
                    })?
            }
            BroadcastRequestTlvTag::DestAddrSubunit => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::DestAddrSubunit(value)
                    })?
            }
            BroadcastRequestTlvTag::DestSubaddress => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::DestSubaddress(value)
                    })?
            }
            BroadcastRequestTlvTag::DestPort => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| { BroadcastRequestTlvValue::DestPort(value) })?
            }
            BroadcastRequestTlvTag::DisplayTime => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::DisplayTime(value)
                    })?
            }
            BroadcastRequestTlvTag::LanguageIndicator => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::LanguageIndicator(value)
                    })?
            }
            BroadcastRequestTlvTag::MessagePayload => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::MessagePayload(value)
                    })?
            }
            BroadcastRequestTlvTag::MsValidity => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::MsValidity(value)
                    })?
            }
            BroadcastRequestTlvTag::PayloadType => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::PayloadType(value)
                    })?
            }
            BroadcastRequestTlvTag::PrivacyIndicator => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::PrivacyIndicator(value)
                    })?
            }
            BroadcastRequestTlvTag::SmsSignal => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| { BroadcastRequestTlvValue::SmsSignal(value) })?
            }
            BroadcastRequestTlvTag::SourceAddrSubunit => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::SourceAddrSubunit(value)
                    })?
            }
            BroadcastRequestTlvTag::SourcePort => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::SourcePort(value)
                    })?
            }
            BroadcastRequestTlvTag::SourceSubaddress => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::SourceSubaddress(value)
                    })?
            }
            BroadcastRequestTlvTag::UserMessageReference => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| {
                        BroadcastRequestTlvValue::UserMessageReference(value)
                    })?
            }
            BroadcastRequestTlvTag::Other(other) => {
                ::rusmpp::decode::DecodeWithLength::decode(src, length)
                    .map_decoded(|value| BroadcastRequestTlvValue::Other {
                        tag: BroadcastRequestTlvTag::Other(other),
                        value,
                    })?
            }
        };
        Ok((value, size))
    }
}
