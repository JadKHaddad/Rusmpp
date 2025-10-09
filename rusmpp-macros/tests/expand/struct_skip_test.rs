/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
pub struct SubmitSmResp {
    /// Docs
    ///
    /// More docs
    message_id: COctetString<1, 65>,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
pub struct SubmitSmResp<'a> {
    /// Docs
    ///
    /// More docs
    message_id: COctetString<'a, 1, 65>,
}
