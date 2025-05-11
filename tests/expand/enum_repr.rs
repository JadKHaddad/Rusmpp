rusmpp::create! {
    #[repr(u8)]
    pub enum DestFlag {
        SmeAddress = 0x01,
        DistributionListName = 0x02,
        Other(u8),
    }
}
