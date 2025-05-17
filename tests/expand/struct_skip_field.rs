rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct DistributionListName {
        @[skip]
        /// Docs
        ///
        /// More docs
        dest_flag: DestFlag,
        pub dl_name: COctetString<1, 21>,
    }
}
