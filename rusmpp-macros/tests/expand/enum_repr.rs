/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[repr(u8)]
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
