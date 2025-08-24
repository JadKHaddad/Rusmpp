//! If we panic!, we lose.
//!
//! ```not_rust
//! cargo +nightly fuzz run decode
//! ```

#![no_main]

extern crate alloc;

use arbitrary::Unstructured;
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use rusmpp::{
    codec::CommandCodec,
    decode::DecodeWithLength,
    encode::{Encode, Length},
    Command,
};
use tokio_util::codec::Decoder;

fuzz_target!(|data: &[u8]| {
    let mut codec = CommandCodec::new().with_max_length(1024);

    // Garbage
    let _ = Command::decode(data, data.len());

    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(data);

    // Garbage with tokio's Decoder
    let _ = codec.decode(&mut bytes);

    // Unstructured garbage
    let mut u = Unstructured::new(data);

    let command = u
        .arbitrary::<Command>()
        .expect("Failed to generate Command");

    let mut buf = ::alloc::vec![0u8; command.length()];

    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(&buf);

    // Encode the garbage
    let size = command.encode(&mut buf);

    // Decode the garbage
    let _ = Command::decode(&buf[..size], command.length());

    // Decode the garbage with tokio's Decoder
    let _ = codec.decode(&mut bytes);
});
