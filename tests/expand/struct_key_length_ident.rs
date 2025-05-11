rusmpp::create! {
    pub struct Tlv {
        tag: TlvTag,
        value_length: u16,
        @[key = tag, length = value_length]
        value: Option<TlvValue>,
    }
}
