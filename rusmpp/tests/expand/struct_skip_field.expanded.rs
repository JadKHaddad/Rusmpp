/// Docs
///
/// More docs
pub struct DistributionListName {
    /// Docs
    ///
    /// More docs
    dest_flag: DestFlag,
    pub dl_name: COctetString<1, 21>,
}
#[automatically_derived]
impl ::core::fmt::Debug for DistributionListName {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "DistributionListName",
            "dest_flag",
            &self.dest_flag,
            "dl_name",
            &&self.dl_name,
        )
    }
}
impl ::rusmpp::encode::Length for DistributionListName {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.dest_flag);
        length += ::rusmpp::encode::Length::length(&self.dl_name);
        length
    }
}
impl ::rusmpp::encode::Encode for DistributionListName {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.dest_flag, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.dl_name, dst, size);
        size
    }
}
impl ::rusmpp::decode::Decode for DistributionListName {
    fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (dl_name, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::dl_name,
        )?;
        Ok((Self::new(dl_name), size))
    }
}
