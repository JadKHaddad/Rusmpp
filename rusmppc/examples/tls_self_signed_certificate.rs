//! See `tls_manual` example to create a self-signed certificate and private key.
//!
//! Run the `tls_manual` example to run the tls server.
//!
//! ```not_rust
//! cargo run -p rusmppc --example tls_manual -- --cert cert.pem --key key.pem
//! ```
//!
//! In another terminal, run this example to connect to the server using the self-signed certificate.
//!
//! ```not_rust
//! cargo run -p rusmppc --example tls_self_signed_certificate -- --cert cert.pem
//! ```
//!

use std::str::FromStr;

use argh::FromArgs;
use rusmpp::{
    pdus::BindTransceiver,
    types::COctetString,
    values::{Npi, Ton},
};
use rusmppc::ConnectionBuilder;
use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::{CertificateDer, pem::PemObject};

/// Command line options
#[derive(FromArgs)]
struct Options {
    /// path to the certificate file in PEM format
    #[argh(option, default = "String::from(\"cert.pem\")")]
    cert: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmpp=off,rusmppc=debug")
        .init();

    let options: Options = argh::from_env();

    let mut root_cert_store = RootCertStore::empty();

    for cert in CertificateDer::pem_file_iter(&options.cert)? {
        root_cert_store.add(cert?)?;
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let (client, _) = ConnectionBuilder::new()
        /*
        If the `rustls-tls-native-roots` feature is enabled, native root certificates are used by default.
        If the `rustls-tls-webpki-roots` feature is enabled, webpki root certificates are used by default.
        Here we provide a custom configuration that uses only the self-signed certificate.
        */
        .rustls_config(config)
        .connect("ssmpp://localhost:2775")
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
