pub struct SubmitSm {
    pub other: u8,
    sm_length: u8,
    short_message: OctetString<0, 255>,
}
impl ::rusmpp::encode::Length for SubmitSm {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.other);
        length += ::rusmpp::encode::Length::length(&self.sm_length);
        length += ::rusmpp::encode::Length::length(&self.short_message);
        length
    }
}
impl ::rusmpp::encode::Encode for SubmitSm {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.other, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.sm_length, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.short_message,
            dst,
            size,
        );
        size
    }
}
impl ::rusmpp::decode::DecodeWithLength for SubmitSm {
    fn decode(
        src: &[u8],
        length: usize,
    ) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (other, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (sm_length, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (short_message, size) = ::rusmpp::decode::DecodeWithLengthExt::decode_move(
            src,
            sm_length as usize,
            size,
        )?;
        Ok((
            Self {
                other,
                sm_length,
                short_message,
            },
            size,
        ))
    }
}
