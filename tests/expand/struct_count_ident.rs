rusmpp::create! {
    pub struct SubmitMulti {
        pub other: u8,
        number_of_dests: u8,
        @[count = number_of_dests]
        dest_address: Vec<DestAddress>,
    }
}
