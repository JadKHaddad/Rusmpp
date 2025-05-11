rusmpp::create! {
    pub struct BroadcastAreaIdentifier {
        pub format: BroadcastAreaFormat,
        @[length = unchecked]
        pub area: OctetString<0, 100>,
    }
}
