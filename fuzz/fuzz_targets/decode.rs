//! If we panic!, we lose.
//! 
//! ```not_rust
//! cargo fuzz run decode
//! ```

#![no_main]

use arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;
use rusmpp::{
    decode::DecodeWithLength, encode::{Encode, Length}, Command
};

fuzz_target!(|data: &[u8]| {
    // Garbage
    let _ = Command::decode(data, data.len());

    // Unstructured garbage
    let mut u = Unstructured::new(data);

    let command = u
        .arbitrary::<Command>()
        .expect("Failed to generate Command");

    let mut buf = vec![0u8; command.length()];

    // Encode the garbage
    let size = command.encode(&mut buf);

    // Decode the garbage
    let _ = Command::decode(&buf[..size], command.length());
});
