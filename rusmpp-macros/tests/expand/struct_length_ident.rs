/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned)]
pub struct SubmitSm {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    sm_length: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = sm_length)]
    short_message: OctetString<0, 255>,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed)]
pub struct SubmitSm<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    sm_length: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = sm_length)]
    short_message: OctetString<'a, 0, 255>,
}
