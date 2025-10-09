/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned)]
pub struct SubmitMulti {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    number_of_dests: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(count = number_of_dests)]
    dest_address: ::alloc::vec::Vec<DestAddress>,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed)]
pub struct SubmitMulti<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    pub other: u8,
    number_of_dests: u8,
    /// Docs
    ///
    /// More docs
    #[rusmpp(count = number_of_dests)]
    dest_address: ::heapless::vec::Vec<DestAddress<'a>, N>,
}
