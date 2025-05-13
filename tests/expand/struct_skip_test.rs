rusmpp::create! {
    @[skip_test]
    pub struct DistributionListName {
        dest_flag: DestFlag,
        pub dl_name: COctetString<1, 21>,
    }
}
