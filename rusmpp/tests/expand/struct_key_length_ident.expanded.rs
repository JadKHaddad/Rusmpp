/// Docs
///
/// More docs
pub struct Tlv {
    /// Docs
    ///
    /// More docs
    tag: TlvTag,
    value_length: u16,
    /// Docs
    ///
    /// More docs
    value: Option<TlvValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tlv {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Tlv",
            "tag",
            &self.tag,
            "value_length",
            &self.value_length,
            "value",
            &&self.value,
        )
    }
}
pub struct TlvParts {
    pub tag: TlvTag,
    pub value_length: u16,
    pub value: Option<TlvValue>,
}
#[automatically_derived]
impl ::core::fmt::Debug for TlvParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "TlvParts",
            "tag",
            &self.tag,
            "value_length",
            &self.value_length,
            "value",
            &&self.value,
        )
    }
}
impl TlvParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(tag: TlvTag, value_length: u16, value: Option<TlvValue>) -> Self {
        Self { tag, value_length, value }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (TlvTag, u16, Option<TlvValue>) {
        (self.tag, self.value_length, self.value)
    }
}
impl Tlv {
    #[inline]
    pub fn into_parts(self) -> TlvParts {
        TlvParts {
            tag: self.tag,
            value_length: self.value_length,
            value: self.value,
        }
    }
}
impl ::rusmpp::encode::Length for Tlv {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.tag);
        length += ::rusmpp::encode::Length::length(&self.value_length);
        length += ::rusmpp::encode::Length::length(&self.value);
        length
    }
}
impl ::rusmpp::encode::Encode for Tlv {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.tag, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.value_length,
            dst,
            size,
        );
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.value, dst, size);
        size
    }
}
impl ::rusmpp::decode::Decode for Tlv {
    fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (tag, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::tag,
        )?;
        let (value_length, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::value_length,
        )?;
        let (value, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
                ::rusmpp::decode::DecodeWithKeyExt::optional_length_checked_decode_move(
                    tag,
                    src,
                    value_length as usize,
                    size,
                ),
                ::rusmpp::fields::SmppField::value,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((Self { tag, value_length, value }, size))
    }
}
