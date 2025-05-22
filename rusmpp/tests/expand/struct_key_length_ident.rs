rusmpp::create! {
    /// Docs
    ///
    /// More docs
    #[derive(Debug)]
    pub struct Tlv {
        /// Docs
        ///
        /// More docs
        tag: TlvTag,
        value_length: u16,
        /// Docs
        ///
        /// More docs
        @[key = tag, length = value_length]
        value: Option<TlvValue>,
    }
}
