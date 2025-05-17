/// Docs
///
/// More docs
pub struct MsValidity {
    /// Docs
    ///
    /// More docs
    pub validity_behavior: MsValidityBehavior,
    /// Docs
    ///
    /// More docs
    pub validity_information: Option<MsValidityInformation>,
}
#[automatically_derived]
impl ::core::fmt::Debug for MsValidity {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "MsValidity",
            "validity_behavior",
            &self.validity_behavior,
            "validity_information",
            &&self.validity_information,
        )
    }
}
impl ::rusmpp::encode::Length for MsValidity {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.validity_behavior);
        length += ::rusmpp::encode::Length::length(&self.validity_information);
        length
    }
}
impl ::rusmpp::encode::Encode for MsValidity {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.validity_behavior,
            dst,
            size,
        );
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.validity_information,
            dst,
            size,
        );
        size
    }
}
impl ::rusmpp::decode::DecodeWithLength for MsValidity {
    fn decode(
        src: &[u8],
        length: usize,
    ) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (validity_behavior, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            "validity_behavior",
        )?;
        let (validity_information, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
                ::rusmpp::decode::DecodeExt::length_checked_decode_move(
                    src,
                    length.saturating_sub(size),
                    size,
                ),
                "validity_information",
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((
            Self {
                validity_behavior,
                validity_information,
            },
            size,
        ))
    }
}
