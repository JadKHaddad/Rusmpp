#[repr(u8)]
pub enum DestFlag {
    SmeAddress = 0x01,
    DistributionListName = 0x02,
    Other(u8),
}
impl ::rusmpp::encode::Length for DestFlag {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}
impl ::rusmpp::encode::Encode for DestFlag {
    fn encode(&self, dst: &mut [u8]) -> usize {
        u8::from(*self).encode(dst)
    }
}
impl ::rusmpp::decode::Decode for DestFlag {
    fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        u8::decode(src).map(|(this, size)| (Self::from(this), size))
    }
}
