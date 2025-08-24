/// Docs
///
/// More docs
pub struct BroadcastAreaIdentifier {
    /// Docs
    ///
    /// More docs
    pub format: BroadcastAreaFormat,
    /// Docs
    ///
    /// More docs
    pub area: OctetString<0, 100>,
}
#[automatically_derived]
impl ::core::fmt::Debug for BroadcastAreaIdentifier {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BroadcastAreaIdentifier",
            "format",
            &self.format,
            "area",
            &&self.area,
        )
    }
}
pub struct BroadcastAreaIdentifierParts {
    pub format: BroadcastAreaFormat,
    pub area: OctetString<0, 100>,
}
#[automatically_derived]
impl ::core::fmt::Debug for BroadcastAreaIdentifierParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BroadcastAreaIdentifierParts",
            "format",
            &self.format,
            "area",
            &&self.area,
        )
    }
}
impl BroadcastAreaIdentifierParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(format: BroadcastAreaFormat, area: OctetString<0, 100>) -> Self {
        Self { format, area }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (BroadcastAreaFormat, OctetString<0, 100>) {
        (self.format, self.area)
    }
}
impl BroadcastAreaIdentifier {
    #[inline]
    pub fn into_parts(self) -> BroadcastAreaIdentifierParts {
        BroadcastAreaIdentifierParts {
            format: self.format,
            area: self.area,
        }
    }
}
impl ::rusmpp::encode::Length for BroadcastAreaIdentifier {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.format);
        length += ::rusmpp::encode::Length::length(&self.area);
        length
    }
}
impl ::rusmpp::encode::Encode for BroadcastAreaIdentifier {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.format, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.area, dst, size);
        size
    }
}
impl ::rusmpp::decode::DecodeWithLength for BroadcastAreaIdentifier {
    fn decode(
        src: &[u8],
        length: usize,
    ) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (format, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::format,
        )?;
        let (area, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeWithLengthExt::decode_move(
                src,
                length.saturating_sub(size),
                size,
            ),
            ::rusmpp::fields::SmppField::area,
        )?;
        Ok((Self { format, area }, size))
    }
}
