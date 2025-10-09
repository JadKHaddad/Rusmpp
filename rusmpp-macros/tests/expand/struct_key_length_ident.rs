/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned)]
pub struct Tlv {
    /// Docs
    ///
    /// More docs
    tag: TlvTag,
    value_length: u16,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue>,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed)]
pub struct Tlv<'a> {
    /// Docs
    ///
    /// More docs
    tag: TlvTag,
    value_length: u16,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue<'a>>,
}
