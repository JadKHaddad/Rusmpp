use std::time::Duration;

use futures::Stream;

use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpStream, ToSocketAddrs},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{Action, Client, Connection, Event, error::Error};

/// Builder for creating a new `SMPP` client.
#[derive(Debug)]
pub struct ClientBuilder {
    max_command_length: usize,
    enquire_link_interval: Duration,
    /// Timeout for waiting for a an enquire link response from the server.
    enquire_link_response_timeout: Duration,
    /// Timeout for waiting for a response from the server.
    response_timeout: Option<Duration>,
    check_interface_version: bool,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    /// Creates a new [`ConnectionBuilder`].
    pub fn new() -> Self {
        Self {
            max_command_length: 4096,
            enquire_link_interval: Duration::from_secs(30),
            enquire_link_response_timeout: Duration::from_secs(5),
            response_timeout: Some(Duration::from_secs(5)),
            check_interface_version: true,
        }
    }

    /// Connects to the `SMPP` server and performs the bind operation.
    ///
    /// Opens and manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    pub async fn connect(
        self,
        host: impl ToSocketAddrs,
    ) -> Result<(Client, impl Stream<Item = Event> + Unpin + 'static), Error> {
        tracing::debug!(target: "rusmppc::connection", "DNS resolution");

        let socket_addr = tokio::net::lookup_host(host)
            .await
            .map_err(Error::Dns)?
            .next()
            .ok_or_else(|| {
                Error::Dns(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No addresses found for the given host",
                ))
            })?;

        tracing::debug!(target: "rusmppc::connection", %socket_addr, "Connecting");

        let stream = TcpStream::connect(socket_addr)
            .await
            .map_err(Error::Connect)?;

        tracing::debug!(target: "rusmppc::connection", %socket_addr, "Connected");

        Ok(self.connected(stream))
    }

    /// Performs the bind operation on an already connected stream.
    ///
    /// Manages a connection in the background and returns a client and an event stream.
    ///
    /// - The client is used as a handle to communicate with the server through the managed connection.
    /// - The event stream is used to receive events from the server, such as incoming messages or errors.
    pub fn connected<S>(self, stream: S) -> (Client, impl Stream<Item = Event> + Unpin + 'static)
    where
        S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    {
        let (events_tx, events_rx) = tokio::sync::mpsc::unbounded_channel::<Event>();
        let (actions_tx, actions_rx) = tokio::sync::mpsc::unbounded_channel::<Action>();
        let (watch_tx, watch_rx) = tokio::sync::watch::channel(());

        let connection = Connection::new(
            stream,
            self.max_command_length,
            events_tx,
            UnboundedReceiverStream::new(actions_rx),
            self.enquire_link_interval,
            self.enquire_link_response_timeout,
            watch_rx,
        );

        let client = Client::new(
            actions_tx,
            self.response_timeout,
            self.check_interface_version,
            watch_tx,
        );

        // If you don't want to spawn the connection and give it to the user to spawn it.
        // Check the comments on `Connection` first. You might want to fuse it first.
        tokio::spawn(connection);

        (client, UnboundedReceiverStream::new(events_rx))
    }
}

impl ClientBuilder {
    /// Sets the maximum command length for incoming commands.
    pub const fn max_command_length(mut self, max_command_length: usize) -> Self {
        self.max_command_length = max_command_length;
        self
    }

    /// Sets the enquire link interval.
    ///
    /// Used to determine how often an enquire link command should be sent to the server.
    pub fn enquire_link_interval(mut self, enquire_link_interval: Duration) -> Self {
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
    pub fn response_timeout(mut self, response_timeout: Duration) -> Self {
        self.response_timeout = Some(response_timeout);
        self
    }

    /// Disables the response timeout.
    pub fn no_response_timeout(mut self) -> Self {
        self.response_timeout = None;
        self
    }

    /// Disables the interface version check.
    ///
    /// This library uses an `SMPP` v5 implementation to encode and decode commands.
    ///
    /// Binding to a server with another `SMPP` may case issues encoding and decoding commands.
    /// Disable interface version check to allow binding to servers with any `SMPP` version.
    pub fn disable_interface_version_check(mut self) -> Self {
        self.check_interface_version = false;
        self
    }
}
