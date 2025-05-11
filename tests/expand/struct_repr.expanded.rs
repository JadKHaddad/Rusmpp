pub struct CallbackNumPresInd {
    pub presentation: Presentation,
    pub screening: Screening,
}
impl ::rusmpp::encode::Length for CallbackNumPresInd {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}
impl ::rusmpp::encode::Encode for CallbackNumPresInd {
    fn encode(&self, dst: &mut [u8]) -> usize {
        u8::from(*self).encode(dst)
    }
}
impl ::rusmpp::decode::Decode for CallbackNumPresInd {
    fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        u8::decode(src).map(|(this, size)| (Self::from(this), size))
    }
}
