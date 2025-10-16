# Rusmppc

![Build Status](https://github.com/JadKHaddad/Rusmpp/actions/workflows/build-and-test.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/rusmppc.svg)](https://crates.io/crates/rusmppc)
[![Crates.io (MSRV)](https://img.shields.io/crates/msrv/rusmppc)](https://crates.io/crates/rusmppc)
[![docs.rs](https://docs.rs/rusmpp/badge.svg)](https://docs.rs/rusmppc)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/rusmppc)](https://crates.io/crates/rusmppc)
[![Crates.io (License)](https://img.shields.io/crates/l/rusmppc)](https://crates.io/crates/rusmppc)

A [`tokio`](https://docs.rs/tokio/latest/tokio/) based [SMPP v5](https://smpp.org/SMPP_v5.pdf) client.

## Features

- `rustls`:  Enables TLS support via [`rustls`](https://docs.rs/rustls/latest/rustls/). Enabled by default.
- `rustls-tls-native-roots`: Enables [`rustls`](https://docs.rs/rustls/latest/rustls/) to use the platform's native root certificates through [`rustls-native-certs`](https://docs.rs/rustls-native-certs/latest/rustls_native_certs/) while using default configuration. Enables the `rustls` feature and is enabled by default.
- `rustls-tls-webpki-roots`: Enables [`rustls`](https://docs.rs/rustls/latest/rustls/) to use the [`webpki-roots`](https://docs.rs/webpki-roots/latest/webpki_roots/) crate's root certificates while using default configuration. Enables the `rustls` feature and is enabled by default.

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](LICENSE-APACHE) or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
