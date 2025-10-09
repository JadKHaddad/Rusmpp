/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = owned)]
pub struct Command {
    /// Docs
    ///
    /// More docs
    id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = "unchecked")]
    pdu: Option<Pdu>,
}

/// Docs
///
/// More docs
#[derive(Debug, rusmpp_macros::Rusmpp)]
#[rusmpp(decode = borrowed)]
pub struct Command<'a, const N: usize> {
    /// Docs
    ///
    /// More docs
    id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    /// Docs
    ///
    /// More docs
    #[rusmpp(key = id, length = "unchecked")]
    pdu: Option<Pdu<'a, N>>,
}
