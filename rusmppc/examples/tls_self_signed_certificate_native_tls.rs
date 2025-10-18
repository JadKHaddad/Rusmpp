//! See `tls_manual_rustls` example to create a self-signed certificate and private key.
//!
//! Run the `tls_manual_rustls` example to run the tls server.
//!
//! ```not_rust
//! cargo run -p rusmppc --example tls_manual_rustls --features rustls -- --cert cert.pem --key key.pem --host localhost:2775 --server
//! ```
//!
//! In another terminal, run this example to connect to the server using the self-signed certificate.
//!
//! ```not_rust
//! cargo run -p rusmppc --example tls_self_signed_certificate_native_tls --no-default-features --features native-tls -- --cert cert.pem --host localhost:2775
//! ```
//!

use std::str::FromStr;

use argh::FromArgs;
use native_tls::TlsConnector;
use rusmpp::{
    pdus::BindTransceiver,
    types::COctetString,
    values::{Npi, Ton},
};
use rusmppc::ConnectionBuilder;

/// Command line options
#[derive(FromArgs)]
struct Options {
    /// path to the certificate file in PEM format
    #[argh(option, default = "String::from(\"cert.pem\")")]
    cert: String,

    /// host to connect to
    #[argh(option, default = "String::from(\"localhost:2775\")")]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmpp=off,rusmppc=debug")
        .init();

    let options: Options = argh::from_env();

    let url = format!("smpps://{}", options.host);

    let cert = native_tls::Certificate::from_pem(&tokio::fs::read(&options.cert).await?)?;

    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;

    let (client, _) = ConnectionBuilder::new()
        .native_tls_connector(connector)
        .connect(url)
        .await?;

    client
        .bind_transceiver(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .system_type(COctetString::empty())
                .addr_ton(Ton::Unknown)
                .addr_npi(Npi::Unknown)
                .address_range(COctetString::empty())
                .build(),
        )
        .await?;

    client.close().await?;
    client.closed().await;

    Ok(())
}
