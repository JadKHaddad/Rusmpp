rusmpp::create! {
    pub struct MsValidity {
        pub validity_behavior: MsValidityBehavior,
        @[length = checked]
        pub validity_information: Option<MsValidityInformation>,
    }
}
