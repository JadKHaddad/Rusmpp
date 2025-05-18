rusmpp::create! {
    @[skip_test]
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct DistributionListName {
        /// Docs
        ///
        /// More docs
        dest_flag: DestFlag,
        pub dl_name: COctetString<1, 21>,
    }
}
