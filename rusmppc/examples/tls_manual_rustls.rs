//! Create the certificate and key files with:
//!
//! ```sh
//! openssl req -x509 -newkey rsa:4096 \
//!   -keyout key.pem -out cert.pem \
//!   -days 365 -nodes \
//!   -subj "/CN=localhost" \
//!   -addext "basicConstraints=CA:FALSE" \
//!   -addext "keyUsage = digitalSignature, keyEncipherment" \
//!   -addext "extendedKeyUsage = serverAuth, clientAuth" \
//!   -addext "subjectAltName=DNS:localhost"
//! ```
//!
//! Run the server with:
//!
//! ```not_rust
//! cargo run -p rusmppc --example tls_manual_rustls --features rustls -- --cert cert.pem --key key.pem --host localhost:2775 --server
//! ```
//!
//! In another terminal, run the client with:
//!
//! ```not_rust
//! cargo run -p rusmppc --example tls_manual_rustls --features rustls -- --cert cert.pem --host localhost:2775 --client
//! ```
//!

use std::{str::FromStr, sync::Arc, time::Duration};

use argh::FromArgs;
use futures::{SinkExt, TryStreamExt};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{BindReceiverResp, BindTransceiver, BindTransceiverResp, BindTransmitterResp},
    tokio_codec::CommandCodec,
    types::COctetString,
};
use rusmppc::ConnectionBuilder;
use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, ServerName, pem::PemObject},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, TlsConnector};
use tokio_util::codec::Framed;

/// Command line options
#[derive(FromArgs)]
struct Options {
    /// path to the certificate file in PEM format
    #[argh(option, default = "String::from(\"cert.pem\")")]
    cert: String,

    /// path to the private key file in PEM format
    #[argh(option, default = "String::from(\"key.pem\")")]
    key: String,

    /// host to connect to or bind to
    #[argh(option, default = "String::from(\"localhost:2775\")")]
    host: String,

    /// run the server only
    #[argh(switch)]
    server: bool,

    /// run the client only
    #[argh(switch)]
    client: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let options: Options = argh::from_env();

    tracing_subscriber::fmt()
        .with_env_filter("client=info,server=info,rusmpp=off,rusmppc=debug")
        .init();

    match (options.server, options.client) {
        (true, true) | (false, false) => {
            tracing::error!("You must specify either --server or --client");
            std::process::exit(1);
        }
        (true, false) => {
            tracing::info!("Running in server mode");
            server(&options.cert, &options.key, &options.host).await?;
        }
        (false, true) => {
            tracing::info!("Running in client mode");
            client(&options.cert, &options.host).await?;
        }
    }

    Ok(())
}

async fn client(cert: &str, host: &str) -> Result<(), Box<dyn core::error::Error>> {
    let mut root_cert_store = RootCertStore::empty();

    for cert in CertificateDer::pem_file_iter(cert)? {
        root_cert_store.add(cert?)?;
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    tracing::info!(target: "client", "Connecting to server at {}", host);

    let stream = TcpStream::connect(host).await?;

    tracing::info!(target: "client", "Connected to server");

    let domain = ServerName::try_from("localhost")?;

    let stream = connector.connect(domain, stream).await?;

    tracing::info!(target: "client", "TLS handshake completed");

    let (client, _) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(10))
        .response_timeout(Duration::from_secs(2))
        .connected(stream);

    tracing::info!(target: "client", "Sending bind request");

    let response = client
        .bind_transceiver(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .build(),
        )
        .await?;

    tracing::info!(target: "client", ?response, "Bound as transceiver");

    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}

async fn server(cert: &str, key: &str, host: &str) -> Result<(), Box<dyn core::error::Error>> {
    let certs: Vec<CertificateDer> =
        CertificateDer::pem_file_iter(cert)?.collect::<Result<Vec<_>, _>>()?;

    let key = PrivateKeyDer::from_pem_file(key)?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(host).await?;

    tracing::info!(target: "server", "Listening on {}", host);

    loop {
        let (stream, addr) = listener.accept().await?;

        tracing::info!(target: "server", %addr, "Accepted connection");

        let acceptor = acceptor.clone();

        let fut = async move {
            let stream = acceptor.accept(stream).await?;

            tracing::info!(target: "server", %addr, "TLS handshake completed");

            let mut framed = Framed::new(stream, CommandCodec::new().with_max_length(1024));

            let mut bind_response = (&mut framed).try_filter_map(|command| {
                let sequence_number = command.sequence_number();

                tracing::info!(
                    target: "server",
                    sequence_number,
                    id = ?command.id(),
                    "Received command"
                );

                let system_id = COctetString::from_str("Rusmpp").expect("Valid COctetString");

                let pdu: Option<Pdu> = match command.id() {
                    CommandId::BindTransmitter => Some(
                        BindTransmitterResp::builder()
                            .system_id(system_id)
                            .build()
                            .into(),
                    ),
                    CommandId::BindReceiver => Some(
                        BindReceiverResp::builder()
                            .system_id(system_id)
                            .build()
                            .into(),
                    ),
                    CommandId::BindTransceiver => Some(
                        BindTransceiverResp::builder()
                            .system_id(system_id)
                            .build()
                            .into(),
                    ),
                    _ => None, // filtered out
                };

                let response = pdu.map(|pdu| {
                    Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(pdu)
                });

                futures::future::ok(response)
            });

            let bind_response = match bind_response.try_next().await? {
                Some(response) => response,
                None => {
                    return Ok(());
                }
            };

            tracing::info!(
                target: "server",
                sequence_number = bind_response.sequence_number(),
                id = ?bind_response.id(),
                "Sending bind response"
            );

            framed.send(bind_response).await?;

            loop {
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(5)) => {
                        break;
                    },
                    command = framed.try_next() => {
                        match command {
                            Ok(Some(command)) => {
                                tracing::info!(
                                    target: "server",
                                    sequence_number = command.sequence_number(),
                                    id = ?command.id(),
                                    "Received command"
                                );
                            },

                            Ok(None) => {
                                tracing::info!(target: "server", %addr, "Connection closed by peer");

                                break;
                            },

                            Err(err) => {
                                return Err(err.into());
                            }
                        }
                    },
                }
            }

            Result::<(), Box<dyn core::error::Error>>::Ok(())
        };

        tokio::spawn(async move {
            if let Err(err) = fut.await {
                tracing::error!(target: "server", %err, "Connection error");
            }
        });
    }
}
