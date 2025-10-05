# Rusmpp-Core

## Features

- `alloc`:  Enables the `alloc` crate.
- `verbose`: Enables verbose error reports. Enables the `alloc` feature.
- `arbitrary`: Implements [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait for all SMPP types.
- `serde`: Implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait for all SMPP types.
- `serde-deserialize-unchecked`: Implements [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) trait for owned SMPP types, but does not check the validity of the data. Use with caution.
- `tokio-codec`: Implements [`tokio-util`](https://docs.rs/tokio-util/latest/tokio_util/index.html) [`Encoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html) and [`Decoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html) traits.
- `framez`: Implements [`framez`](https://docs.rs/framez/latest/framez/index.html) [`Encoder`](https://docs.rs/framez/latest/framez/encode/trait.Encoder.html) and [`Decoder`](https://docs.rs/framez/latest/framez/decode/trait.Decoder.html) traits.
- `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).
- `pretty-hex-fmt`: Logs byte slices like `[0x00, 0x00, 0x00, 0x6F]` instead of `[00, 00, 00, 6F]`, if `tracing` feature is enabled.
- `char-fmt`: Logs byte slices as characters, if `tracing` feature is enabled.

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](../LICENSE-APACHE) or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](../LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
