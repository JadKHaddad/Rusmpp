rusmpp::create! {
    #[repr(u8)]
    @[skip_test]
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub enum DestFlag {
        /// Docs
        ///
        /// More docs
        SmeAddress = 0x01,
        DistributionListName = 0x02,
        /// Docs
        ///
        /// More docs
        Other(u8),
    }
}
