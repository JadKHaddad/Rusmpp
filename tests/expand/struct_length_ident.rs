rusmpp::create! {
    pub struct SubmitSm {
        pub other: u8,
        sm_length: u8,
        @[length = sm_length]
        short_message: OctetString<0, 255>,
    }
}
