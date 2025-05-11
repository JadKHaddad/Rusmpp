pub struct Tlv {
    tag: TlvTag,
    value_length: u16,
    value: Option<TlvValue>,
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
        let (tag, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (value_length, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (value, size) = ::rusmpp::decode::DecodeWithKeyExt::optional_length_checked_decode_move(
                tag,
                src,
                value_length as usize,
                size,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((Self { tag, value_length, value }, size))
    }
}
