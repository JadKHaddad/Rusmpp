rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct SubmitSm {
        /// Docs
        ///
        /// More docs
        pub other: u8,
        sm_length: u8,
        /// Docs
        ///
        /// More docs
        @[length = sm_length]
        short_message: OctetString<0, 255>,
    }
}
