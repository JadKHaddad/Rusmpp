rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct MsValidity {
        /// Docs
        ///
        /// More docs
        pub validity_behavior: MsValidityBehavior,
        /// Docs
        ///
        /// More docs
        @[length = checked]
        pub validity_information: Option<MsValidityInformation>,
    }
}
