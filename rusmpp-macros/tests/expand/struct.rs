/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned)]
pub struct CancelSm {
    /// Docs
    ///
    /// More docs
    pub service_type: ServiceType,
    pub message_id: COctetString<1, 65>,
    pub other: u8,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed)]
pub struct CancelSm<'a> {
    /// Docs
    ///
    /// More docs
    pub service_type: ServiceType<'a>,
    pub message_id: COctetString<'a, 1, 65>,
    pub other: u8,
}
