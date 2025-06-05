use std::time::Duration;

use futures::Stream;

use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpStream, ToSocketAddrs},
};

use crate::{Client, Connection, Event, ReconnectingConnection, ReconnectingEvent, error::Error};

/// Builder for creating a new `SMPP` connection.
#[derive(Debug)]
pub struct ConnectionBuilder {
    max_command_length: usize,
    enquire_link_interval: Duration,
    /// Timeout for waiting for a an enquire link response from the server.
    enquire_link_response_timeout: Duration,
    /// Timeout for waiting for a response from the server.
    response_timeout: Option<Duration>,
    check_interface_version: bool,
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionBuilder {
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
        let (connection, watch, actions, events) = Connection::new(
            stream,
            self.max_command_length,
            self.enquire_link_interval,
            self.enquire_link_response_timeout,
        );

        let client = Client::new(
            actions,
            self.response_timeout,
            self.check_interface_version,
            watch,
        );

        // If you don't want to spawn the connection and give it to the user to spawn it.
        // Check the comments on `Connection` first. You might want to fuse it first.
        tokio::spawn(connection);

        (client, events)
    }

    // TODO: add the ReconnectConnectionBuilder
    pub async fn reconnect<S, F, Fut>(
        self,
        connect: F,
    ) -> Result<
        (
            Client,
            impl Stream<Item = ReconnectingEvent> + Unpin + 'static,
        ),
        Error,
    >
    where
        S: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
        F: Fn() -> Fut + Send + Clone + 'static,
        Fut: Future<Output = Result<S, std::io::Error>> + Send,
    {
        let stream = connect().await.map_err(Error::Connect)?;

        let connected = Connection::new(
            stream,
            self.max_command_length,
            self.enquire_link_interval,
            self.enquire_link_response_timeout,
        );

        let (reconnecting_connection, watch, actions, events) = ReconnectingConnection::new(
            connected,
            move || {
                let connect = connect.clone();

                async move {
                    let stream = connect().await.map_err(Error::Connect)?;

                    Ok::<_, Error>(Connection::new(
                        stream,
                        self.max_command_length,
                        self.enquire_link_interval,
                        self.enquire_link_response_timeout,
                    ))
                }
            }, // TODO:
            Duration::from_secs(5),
            5,
        );

        let client = Client::new(
            actions,
            self.response_timeout,
            self.check_interface_version,
            watch,
        );

        tokio::spawn(reconnecting_connection.run());

        Ok((client, events))
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
