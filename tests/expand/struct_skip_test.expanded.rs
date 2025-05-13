pub struct DistributionListName {
    dest_flag: DestFlag,
    pub dl_name: COctetString<1, 21>,
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
        let (dest_flag, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        let (dl_name, size) = ::rusmpp::decode::DecodeExt::decode_move(src, size)?;
        Ok((Self { dest_flag, dl_name }, size))
    }
}
