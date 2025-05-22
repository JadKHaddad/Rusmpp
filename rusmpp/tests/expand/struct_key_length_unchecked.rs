rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct Command {
        /// Docs
        ///
        /// More docs
        command_id: CommandId,
        pub command_status: CommandStatus,
        pub sequence_number: u32,
        /// Docs
        ///
        /// More docs
        @[key = command_id, length = unchecked]
        pdu: Option<Pdu>,
    }
}
