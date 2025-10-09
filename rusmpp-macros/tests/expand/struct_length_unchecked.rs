/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned)]
pub struct BroadcastAreaIdentifier {
    /// Docs
    ///
    /// More docs
    pub format: BroadcastAreaFormat,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = "unchecked")]
    pub area: AnyOctetString,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed)]
pub struct BroadcastAreaIdentifier<'a> {
    /// Docs
    ///
    /// More docs
    pub format: BroadcastAreaFormat,
    /// Docs
    ///
    /// More docs
    #[rusmpp(length = "unchecked")]
    pub area: AnyOctetString<'a>,
}
