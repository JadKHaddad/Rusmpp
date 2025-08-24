/// Docs
///
/// More docs
pub struct SubmitMulti {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    number_of_dests: u8,
    /// Docs
    ///
    /// More docs
    dest_address: ::alloc::vec::Vec<DestAddress>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMulti {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMulti",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
pub struct SubmitMultiParts {
    pub other: u8,
    pub number_of_dests: u8,
    pub dest_address: ::alloc::vec::Vec<DestAddress>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubmitMultiParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SubmitMultiParts",
            "other",
            &self.other,
            "number_of_dests",
            &self.number_of_dests,
            "dest_address",
            &&self.dest_address,
        )
    }
}
impl SubmitMultiParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        other: u8,
        number_of_dests: u8,
        dest_address: ::alloc::vec::Vec<DestAddress>,
    ) -> Self {
        Self {
            other,
            number_of_dests,
            dest_address,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (u8, u8, ::alloc::vec::Vec<DestAddress>) {
        (self.other, self.number_of_dests, self.dest_address)
    }
}
impl SubmitMulti {
    #[inline]
    pub fn into_parts(self) -> SubmitMultiParts {
        SubmitMultiParts {
            other: self.other,
            number_of_dests: self.number_of_dests,
            dest_address: self.dest_address,
        }
    }
}
impl ::rusmpp::encode::Length for SubmitMulti {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.other);
        length += ::rusmpp::encode::Length::length(&self.number_of_dests);
        length += ::rusmpp::encode::Length::length(&self.dest_address);
        length
    }
}
impl ::rusmpp::encode::Encode for SubmitMulti {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.number_of_dests,
            dst,
            size,
        );
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.dest_address,
            dst,
            size,
        );
        size
    }
}
impl ::rusmpp::decode::DecodeWithLength for SubmitMulti {
    fn decode(
        src: &[u8],
        length: usize,
    ) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (other, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::other,
        )?;
        let (number_of_dests, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::number_of_dests,
        )?;
        let (dest_address, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::counted_move(
                src,
                number_of_dests as usize,
                size,
            ),
            ::rusmpp::fields::SmppField::dest_address,
        )?;
        Ok((
            Self {
                other,
                number_of_dests,
                dest_address,
            },
            size,
        ))
    }
}
