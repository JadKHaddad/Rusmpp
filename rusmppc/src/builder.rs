use std::time::Duration;

use futures::{FutureExt, Stream};

use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};

use crate::{Client, Connection, Event, MaybeTlsStream, error::Error};

/// Builder for creating a new `SMPP` connection.
#[derive(Debug)]
pub struct ConnectionBuilder {
    pub(crate) max_command_length: usize,
    pub(crate) enquire_link_interval: Option<Duration>,
    /// Timeout for waiting for a an enquire link response from the server.
    pub(crate) enquire_link_response_timeout: Duration,
    /// Timeout for waiting for a response from the server.
    pub(crate) response_timeout: Option<Duration>,
    pub(crate) check_interface_version: bool,
    /// TLS configurations provided by the user. If None, default configurations will be used.
    #[cfg(feature = "rustls")]
    rustls_config: Option<rustls::ClientConfig>,
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionBuilder {
    /// Creates a new [`ConnectionBuilder`] with default configurations.
    ///
    /// # Defaults
    /// - `max_command_length`: 4096 bytes
    /// - `enquire_link_interval`: 30 seconds
    /// - `enquire_link_response_timeout`: 5 seconds
    /// - `response_timeout`: 5 seconds
    /// - `check_interface_version`: true
    /// - `rustls_config`: default configuration will be used if TLS is enabled. See [`rustls_config`](Self::rustls_config) for more details.
    pub fn new() -> Self {
        Self {
            max_command_length: 4096,
            enquire_link_interval: Some(Duration::from_secs(30)),
            enquire_link_response_timeout: Duration::from_secs(5),
            response_timeout: Some(Duration::from_secs(5)),
            check_interface_version: true,
            #[cfg(feature = "rustls")]
            rustls_config: None,
        }
    }

    /// Does not spawn the connection in the background.
    ///
    /// It is your responsibility to run the connection to completion.
    pub fn no_spawn(self) -> NoSpawnConnectionBuilder {
        NoSpawnConnectionBuilder { builder: self }
    }

    /// Connects to the `SMPP` server.
    ///
    /// Opens and manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    ///
    /// # Example
    ///
    /// Connect to an `SMPP` server running on localhost at port 2775.
    ///
    /// ```
    /// # use rusmppc::ConnectionBuilder;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (client, events) = ConnectionBuilder::new()
    ///     .connect("smpp://localhost:2775")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Connect to an `SMPP` server running on localhost at port 2775 using TLS.
    ///
    /// ```
    /// # use rusmppc::ConnectionBuilder;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (client, events) = ConnectionBuilder::new()
    ///     .connect("smpps://localhost:2775")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Supported URL schemes
    /// - `smpp`: Connect using plain TCP.
    /// - `ssmpp` or `smpps`: Connect using TLS. Requires the `rustls` feature to be enabled.
    ///
    /// # Notes
    /// - If no port is specified in the URL, the default port `2775` will be used.
    /// - Path and query parameters in the URL are ignored silently.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    ///
    /// - If the URL is invalid. See [`url::Url::parse`] for more details.
    /// - If the URL scheme is not supported. Supported schemes are `smpp`, `ssmpp`, and `smpps`.
    /// - If the URL does not have a host.
    /// - If DNS resolution fails.
    /// - If the connection to the server fails.
    /// - If TLS is enabled (when using `ssmpp` or `smpps` schemes) but the `rustls` feature is not enabled.
    /// - If TLS handshake fails.
    pub async fn connect(
        self,
        url: impl AsRef<str>,
    ) -> Result<(Client, impl Stream<Item = Event> + Unpin + 'static), Error> {
        let (client, events, connection) = self.no_spawn().connect(url).await?;

        tokio::spawn(connection);

        Ok((client, events))
    }

    /// Creates a client from an existing connection.
    ///
    /// Manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    pub fn connected<S>(self, stream: S) -> (Client, impl Stream<Item = Event> + Unpin + 'static)
    where
        S: AsyncRead + AsyncWrite + Send + 'static,
    {
        let (client, events, connection) = self.no_spawn().connected(stream);

        tokio::spawn(connection);

        (client, events)
    }
}

impl ConnectionBuilder {
    /// Sets the maximum command length for incoming commands.
    pub const fn max_command_length(mut self, max_command_length: usize) -> Self {
        self.max_command_length = max_command_length;
        self
    }

    /// Sets the enquire link interval.
    ///
    /// Used to determine how often an enquire link command should be sent to the server.
    pub fn enquire_link_interval(mut self, enquire_link_interval: Duration) -> Self {
        self.enquire_link_interval = Some(enquire_link_interval);
        self
    }

    /// Disables the enquire link interval.
    ///
    /// When disabled, no enquire link commands will be sent to the server.
    pub fn no_enquire_link_interval(mut self) -> Self {
        self.enquire_link_interval = None;
        self
    }

    /// Sets the enquire link interval.
    ///
    /// If set to `None`, no enquire link commands will be sent to the server.
    pub fn with_enquire_link_interval(mut self, enquire_link_interval: Option<Duration>) -> Self {
        self.enquire_link_interval = enquire_link_interval;
        self
    }

    /// Sets the enquire link response timeout.
    ///
    /// This timeout is used to determine how long the connection should wait for an enquire link response from the server.
    pub fn enquire_link_response_timeout(
        mut self,
        enquire_link_response_timeout: Duration,
    ) -> Self {
        self.enquire_link_response_timeout = enquire_link_response_timeout;
        self
    }

    /// Sets the response timeout.
    ///
    /// This timeout is used to determine how long the client should wait for a response from the server.
    ///
    /// The timer is started after the command has been sent to the server.
    pub fn response_timeout(mut self, response_timeout: Duration) -> Self {
        self.response_timeout = Some(response_timeout);
        self
    }

    /// Disables the response timeout.
    pub fn no_response_timeout(mut self) -> Self {
        self.response_timeout = None;
        self
    }

    /// Sets the response timeout.
    ///
    /// If set to `None`, no timeout will be used and the client will wait indefinitely for a response from the server.
    pub fn with_response_timeout(mut self, response_timeout: Option<Duration>) -> Self {
        self.response_timeout = response_timeout;
        self
    }

    /// Disables the interface version check.
    ///
    /// This library uses `SMPP v5` implementation to encode and decode commands.
    ///
    /// Binding to a server with another `SMPP` version may cause issues encoding and decoding commands.
    /// Disable interface version check to allow binding to servers with any `SMPP` version.
    pub fn disable_interface_version_check(mut self) -> Self {
        self.check_interface_version = false;
        self
    }

    /// Sets a custom `rustls` client configuration.
    ///
    /// If not set, a default configuration will be used.
    ///  - If the `rustls-tls-native-roots` feature is enabled, native root certificates are used.
    ///  - If the `rustls-tls-webpki-roots` feature is enabled, webpki root certificates are used.
    #[cfg(feature = "rustls")]
    pub fn rustls_config(mut self, config: rustls::ClientConfig) -> Self {
        self.rustls_config = Some(config);
        self
    }
}

/// Builder for creating a new `SMPP` connection without spawning it in the background.
#[derive(Debug)]
pub struct NoSpawnConnectionBuilder {
    builder: ConnectionBuilder,
}

impl NoSpawnConnectionBuilder {
    /// Connects to the `SMPP` server without spawning the connection in the background.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    ///
    /// - If the URL is invalid. See [`url::Url::parse`] for more details.
    /// - If the URL scheme is not supported. Supported schemes are `smpp`, `ssmpp`, and `smpps`.
    /// - If the URL does not have a host.
    /// - If DNS resolution fails.
    /// - If the connection to the server fails.
    /// - If TLS is enabled (when using `ssmpp` or `smpps` schemes) but the `rustls` feature is not enabled.
    /// - If TLS handshake fails.
    #[allow(unused_mut)]
    pub async fn connect(
        mut self,
        url: impl AsRef<str>,
    ) -> Result<
        (
            Client,
            impl Stream<Item = Event> + Unpin + 'static,
            impl Future<Output = ()> + 'static,
        ),
        Error,
    > {
        enum Scheme {
            Smpp,
            Ssmpp,
        }

        let url = url::Url::parse(url.as_ref()).map_err(|err| {
            Error::Connect(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid URL: {err}"),
            ))
        })?;

        let scheme = match url.scheme() {
            "smpp" => Scheme::Smpp,
            "ssmpp" | "smpps" => Scheme::Ssmpp,
            scheme => {
                return Err(Error::Connect(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "Unsupported URL scheme: {scheme}, supported schemes are smpp and ssmpp/smpps"
                    ),
                )));
            }
        };

        let domain = url.host_str().ok_or_else(|| {
            Error::Connect(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "URL must have a host",
            ))
        })?;

        let port = url.port().unwrap_or(2775);

        let host = format!("{domain}:{port}");

        tracing::debug!(target: "rusmppc::connection::dns", host, "Resolving host");

        let socket_addr = tokio::net::lookup_host(host)
            .await
            .map_err(Error::Connect)?
            .next()
            .ok_or_else(|| {
                Error::Connect(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No addresses found for the given host",
                ))
            })?;

        tracing::debug!(target: "rusmppc::connection::tcp", %socket_addr, "Connecting");

        let stream = TcpStream::connect(socket_addr)
            .await
            .map_err(Error::Connect)?;

        tracing::debug!(target: "rusmppc::connection::tcp", %socket_addr, "Connected");

        let stream = match scheme {
            Scheme::Smpp => MaybeTlsStream::plain(stream),
            Scheme::Ssmpp => {
                #[cfg(feature = "rustls")]
                {
                    MaybeTlsStream::rustls(stream, domain, self.builder.rustls_config.take())
                        .await?
                }
                #[cfg(not(feature = "rustls"))]
                {
                    return Err(Error::Connect(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "TLS support is not enabled, enable the `rustls` feature to use ssmpp/smpps",
                    )));
                }
            }
        };

        Ok(self.connected(stream))
    }

    /// Creates a client from an existing connection without spawning the connection in the background.
    pub fn connected<S>(
        self,
        stream: S,
    ) -> (
        Client,
        impl Stream<Item = Event> + Unpin + 'static,
        impl Future<Output = ()>,
    )
    where
        S: AsyncRead + AsyncWrite + Send + 'static,
    {
        let (connection, watch, actions, events) = Connection::new(
            self.builder.max_command_length,
            self.builder.enquire_link_interval,
            self.builder.enquire_link_response_timeout,
        );

        let client = Client::new(
            actions,
            self.builder.response_timeout,
            self.builder.check_interface_version,
            watch,
        );

        (client, events, async move {
            let mut stream = std::pin::pin!(stream);

            let connection = connection.with_stream(&mut stream);

            // See comments on Connection struct to understand why we fuse the connection future.
            connection.fuse().await;

            tracing::debug!(target: "rusmppc::connection::tcp", "Shutting down stream");

            if let Err(err) = stream.shutdown().await {
                tracing::error!(target: "rusmppc::connection::tcp", ?err, "Failed to shutdown stream");
            }
        })
    }
}
