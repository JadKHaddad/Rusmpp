#![no_main]

use std::io::Cursor;

use libfuzzer_sys::fuzz_target;
use rusmpp::prelude::*;

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data.to_vec());
    let _ = Pdu::io_read(&mut cursor);
});
