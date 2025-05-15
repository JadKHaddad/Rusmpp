/// Docs
///
/// More docs
pub struct CancelSm {
    /// Docs
    ///
    /// More docs
    pub service_type: ServiceType,
    pub message_id: COctetString<1, 65>,
    pub other: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for CancelSm {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CancelSm",
            "service_type",
            &self.service_type,
            "message_id",
            &self.message_id,
            "other",
            &&self.other,
        )
    }
}
impl ::rusmpp::encode::Length for CancelSm {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.service_type);
        length += ::rusmpp::encode::Length::length(&self.message_id);
        length += ::rusmpp::encode::Length::length(&self.other);
        length
    }
}
impl ::rusmpp::encode::Encode for CancelSm {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.service_type,
            dst,
            size,
        );
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.message_id, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.other, dst, size);
        size
    }
}
impl ::rusmpp::decode::Decode for CancelSm {
    fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (service_type, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (message_id, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (other, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        Ok((
            Self {
                service_type,
                message_id,
                other,
            },
            size,
        ))
    }
}
