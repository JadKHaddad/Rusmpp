/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
pub struct MsValidity {
    /// Docs
    ///
    /// More docs
    pub validity_behavior: MsValidityBehavior,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = "checked")]
    pub validity_information: Option<MsValidityInformation>,
}
