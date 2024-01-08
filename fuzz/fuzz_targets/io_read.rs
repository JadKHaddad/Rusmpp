#![no_main]

use libfuzzer_sys::fuzz_target;
use rusmpp::prelude::*;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    let _ = Pdu::io_read(&mut cursor);
});
