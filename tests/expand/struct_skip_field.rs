rusmpp::create! {
    pub struct DistributionListName {
        @[skip]
        dest_flag: DestFlag,
        pub dl_name: COctetString<1, 21>,
    }
}
