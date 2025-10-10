#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}

// TODO: must be removed after migrating to rusmpp-core
