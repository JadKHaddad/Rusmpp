rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct CancelSm {
        /// Docs
        ///
        /// More docs
        pub service_type: ServiceType,
        pub message_id: COctetString<1, 65>,
        pub other: u8,
    }
}
