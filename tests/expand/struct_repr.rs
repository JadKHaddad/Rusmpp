rusmpp::create! {
    @[repr = u8]
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct CallbackNumPresInd {
        /// Docs
        ///
        /// More docs
        pub presentation: Presentation,
        pub screening: Screening,
    }
}
