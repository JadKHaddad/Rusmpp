/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(repr = "u8")]
pub struct CallbackNumPresInd {
    /// Docs
    ///
    /// More docs
    pub presentation: Presentation,
    pub screening: Screening,
}
