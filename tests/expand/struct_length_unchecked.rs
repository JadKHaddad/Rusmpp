rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct BroadcastAreaIdentifier {
        /// Docs
        ///
        /// More docs
        pub format: BroadcastAreaFormat,
        /// Docs
        ///
        /// More docs
        @[length = unchecked]
        pub area: OctetString<0, 100>,
    }
}
