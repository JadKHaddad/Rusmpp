rusmpp::create! {
    pub struct CancelSm {
        pub service_type: ServiceType,
        pub message_id: COctetString<1, 65>,
        pub other: u8,
    }
}
