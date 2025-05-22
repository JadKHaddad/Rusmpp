rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct SubmitMulti {
        /// Docs
        ///
        /// More docs
        pub other: u8,
        number_of_dests: u8,
        /// Docs
        ///
        /// More docs
        @[count = number_of_dests]
        dest_address: ::alloc::vec::Vec<DestAddress>,
    }
}
