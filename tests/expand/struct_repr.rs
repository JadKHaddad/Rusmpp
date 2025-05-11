rusmpp::create! {
    @[repr = u8]
    pub struct CallbackNumPresInd {
        pub presentation: Presentation,
        pub screening: Screening,
    }
}
