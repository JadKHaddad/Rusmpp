use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A stream that might be protected with TLS.
#[non_exhaustive]
#[derive(Debug)]
pub enum MaybeTlsStream<S> {
    Plain(S),
    #[cfg(feature = "rustls")]
    Rustls(tokio_rustls::client::TlsStream<S>),
}

impl<S: AsyncRead + AsyncWrite + Unpin> MaybeTlsStream<S> {
    /// Creates a new [`MaybeTlsStream::Plain`].
    pub fn plain(stream: S) -> Self {
        Self::Plain(stream)
    }

    /// Creates a new [`MaybeTlsStream::Rustls`].
    #[cfg(feature = "rustls")]
    pub async fn rustls(
        stream: S,
        domain: &str,
        config: Option<rustls::ClientConfig>,
    ) -> Result<Self, crate::error::Error> {
        // Code section inspired by `tokio-tungstenite`.
        let config = match config {
            Some(config) => std::sync::Arc::new(config),
            None => {
                #[allow(unused_mut)]
                let mut root_store = rustls::RootCertStore::empty();
                #[cfg(feature = "rustls-tls-native-roots")]
                {
                    tracing::debug!(target: "rusmppc::connection::tls", "Loading native root CA certificates");

                    let rustls_native_certs::CertificateResult { certs, errors, .. } =
                        rustls_native_certs::load_native_certs();

                    if !errors.is_empty() {
                        tracing::warn!(target: "rusmppc::connection::tls",?errors, "Native root CA certificate loading errors");
                    }

                    // Not finding any native root CA certificates is not fatal if the
                    // "rustls-tls-webpki-roots" feature is enabled.
                    #[cfg(not(feature = "rustls-tls-webpki-roots"))]
                    if certs.is_empty() {
                        return Err(crate::error::Error::Connect(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            format!("No native root CA certificates found (errors: {errors:?})"),
                        )));
                    }

                    let total = certs.len();
                    let (added, ignored) = root_store.add_parsable_certificates(certs);

                    tracing::debug!(target: "rusmppc::connection::tls", total, added, ignored, "Added native root certificates");
                }
                #[cfg(feature = "rustls-tls-webpki-roots")]
                {
                    tracing::debug!(target: "rusmppc::connection::tls", "Loading webpki root CA certificates");

                    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
                }

                std::sync::Arc::new(
                    rustls::ClientConfig::builder()
                        .with_root_certificates(root_store)
                        .with_no_client_auth(),
                )
            }
        };

        let domain = rustls_pki_types::ServerName::try_from(domain)
            .map_err(|err| {
                crate::error::Error::Connect(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    err,
                ))
            })?
            .to_owned();

        let connector = tokio_rustls::TlsConnector::from(config);

        tracing::debug!(target: "rusmppc::connection::tls", "Establishing TLS connection");

        let stream = connector
            .connect(domain, stream)
            .await
            .map_err(crate::error::Error::Connect)?;

        Ok(Self::Rustls(stream))
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for MaybeTlsStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            MaybeTlsStream::Plain(s) => Pin::new(s).poll_read(cx, buf),
            #[cfg(feature = "rustls")]
            MaybeTlsStream::Rustls(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeTlsStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match self.get_mut() {
            MaybeTlsStream::Plain(s) => Pin::new(s).poll_write(cx, buf),
            #[cfg(feature = "rustls")]
            MaybeTlsStream::Rustls(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            MaybeTlsStream::Plain(s) => Pin::new(s).poll_flush(cx),
            #[cfg(feature = "rustls")]
            MaybeTlsStream::Rustls(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            MaybeTlsStream::Plain(s) => Pin::new(s).poll_shutdown(cx),
            #[cfg(feature = "rustls")]
            MaybeTlsStream::Rustls(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}
