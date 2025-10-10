/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned)]
pub struct DistributionListName {
    /// Docs
    ///
    /// More docs
    #[rusmpp(skip_decode)]
    dest_flag: DestFlag,
    pub dl_name: COctetString<1, 21>,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed)]
pub struct DistributionListName<'a> {
    /// Docs
    ///
    /// More docs
    #[rusmpp(skip_decode)]
    dest_flag: DestFlag,
    pub dl_name: COctetString<'a, 1, 21>,
}
