#[repr(u8)]
/// Docs
///
/// More docs
pub enum DestFlag {
    /// Docs
    ///
    /// More docs
    SmeAddress = 0x01,
    DistributionListName = 0x02,
    /// Docs
    ///
    /// More docs
    Other(u8),
}
#[automatically_derived]
impl ::core::fmt::Debug for DestFlag {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DestFlag::SmeAddress => ::core::fmt::Formatter::write_str(f, "SmeAddress"),
            DestFlag::DistributionListName => {
                ::core::fmt::Formatter::write_str(f, "DistributionListName")
            }
            DestFlag::Other(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Other", &__self_0)
            }
        }
    }
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
