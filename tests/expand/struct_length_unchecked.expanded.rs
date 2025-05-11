pub struct BroadcastAreaIdentifier {
    pub format: BroadcastAreaFormat,
    pub area: OctetString<0, 100>,
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
        let (format, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (area, size) = ::rusmpp::decode::DecodeWithLengthExt::decode_move(
            src,
            length.saturating_sub(size),
            size,
        )?;
        Ok((Self { format, area }, size))
    }
}
