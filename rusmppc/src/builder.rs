use std::{fmt, net::SocketAddr, time::Duration};

use futures::{Stream, channel::mpsc::unbounded};
use rusmpp::{
    pdus::builders::BindAnyBuilder,
    session::SessionState,
    types::COctetString,
    values::{InterfaceVersion, Npi, Ton},
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
    sync::mpsc::channel,
};
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    Client, Event,
    action::Action,
    bind::BindMode,
    connection::{Connection, ConnectionConfig},
    error::Error,
    session_state::SessionStateHolder,
};

/// Builder for creating a new `SMPP` connection.
#[derive(Debug)]
pub struct ConnectionBuilder {
    socket_addr: SocketAddr,
    bind_mode: BindMode,
    bind_builder: BindAnyBuilder,
    max_command_length: usize,
    timeouts: ConnectionTimeouts,
}

#[derive(Debug)]
pub struct ConnectionTimeouts {
    /// Not used.
    _session: Duration,
    /// Timeout for sending an enquire link command.
    ///
    /// When this timeout is reached, an enquire link command is sent to the server.
    pub enquire_link: Duration,
    /// Not used.
    _inactivity: Duration,
    /// Timeout for waiting for a response from the server.
    pub response: Duration,
}

impl Default for ConnectionTimeouts {
    fn default() -> Self {
        Self {
            _session: Duration::from_secs(5),
            enquire_link: Duration::from_secs(30),
            _inactivity: Duration::from_secs(60),
            response: Duration::from_secs(5),
        }
    }
}

impl ConnectionBuilder {
    /// Creates a new [`ConnectionBuilder`] with the given socket address.
    pub fn new(socket_addr: impl Into<SocketAddr>) -> Self {
        Self {
            socket_addr: socket_addr.into(),
            bind_mode: Default::default(),
            bind_builder: BindAnyBuilder::default().interface_version(InterfaceVersion::Smpp5_0),
            max_command_length: 4096,
            timeouts: Default::default(),
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
    ) -> Result<
        (
            Client,
            impl Stream<Item = Event> + Unpin + fmt::Debug + 'static,
        ),
        Error,
    > {
        tracing::debug!(target: "rusmppc::connection", socket_addr=%self.socket_addr, "Connecting");

        let stream = TcpStream::connect(self.socket_addr)
            .await
            .map_err(Error::Connect)?;

        tracing::trace!(target: "rusmppc::connection", socket_addr=%self.socket_addr, "Connected");

        self.assume_connected(stream).await
    }

    /// Takes a connected stream and performs the bind operation.
    ///
    /// This function is separated from [`Self::connect`] to test the library
    /// without actually connecting to a server.
    pub(crate) async fn assume_connected<S>(
        self,
        stream: S,
    ) -> Result<
        (
            Client,
            impl Stream<Item = Event> + Unpin + fmt::Debug + 'static,
        ),
        Error,
    >
    where
        S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    {
        let (events_tx, events_rx) = unbounded::<Event>();
        let (actions_tx, actions_rx) = channel::<Action>(100);
        let (termination_tx, termination_rx) = channel::<()>(1);

        let session_state_holder = SessionStateHolder::new(SessionState::Open);

        let response_timeout = self.timeouts.response;

        let connection = Connection::new(
            stream,
            events_tx,
            ReceiverStream::new(actions_rx),
            termination_tx,
            session_state_holder.clone(),
            ConnectionConfig::new(self.max_command_length, self.timeouts),
        );

        connection.spawn();

        let client = Client::new(
            actions_tx,
            response_timeout,
            session_state_holder,
            termination_rx,
        );

        match self.bind_mode {
            BindMode::Tx => {
                client.bind_transmitter(self.bind_builder.build()).await?;
            }
            BindMode::Rx => {
                client.bind_receiver(self.bind_builder.build()).await?;
            }
            BindMode::TxRx => {
                client.bind_transceiver(self.bind_builder.build()).await?;
            }
        }

        Ok((client, events_rx))
    }
}

impl ConnectionBuilder {
    /// Sets the socket address to connect to.
    pub fn socket_addr(mut self, socket_addr: impl Into<SocketAddr>) -> Self {
        self.socket_addr = socket_addr.into();
        self
    }

    /// Sets the maximum command length for incoming commands.
    pub const fn max_command_length(mut self, max_command_length: usize) -> Self {
        self.max_command_length = max_command_length;
        self
    }

    /// Sets the bind mode for the connection.
    pub const fn bind_mode(mut self, bind_mode: BindMode) -> Self {
        self.bind_mode = bind_mode;
        self
    }

    /// Sets the bind mode to transmitter.
    pub const fn transmitter(mut self) -> Self {
        self.bind_mode = BindMode::Tx;
        self
    }

    /// Sets the bind mode to receiver.
    pub const fn receiver(mut self) -> Self {
        self.bind_mode = BindMode::Rx;
        self
    }

    /// Sets the bind mode to transceiver (both transmitter and receiver).
    pub const fn transceiver(mut self) -> Self {
        self.bind_mode = BindMode::TxRx;
        self
    }

    /// Sets the system ID.
    pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
        self.bind_builder = self.bind_builder.system_id(system_id);
        self
    }

    /// Sets the password.
    pub fn password(mut self, password: COctetString<1, 9>) -> Self {
        self.bind_builder = self.bind_builder.password(password);
        self
    }

    /// Sets the system type.
    pub fn system_type(mut self, system_type: COctetString<1, 13>) -> Self {
        self.bind_builder = self.bind_builder.system_type(system_type);
        self
    }

    /// Sets the address TON (Type of Number).
    pub fn addr_ton(mut self, addr_ton: Ton) -> Self {
        self.bind_builder = self.bind_builder.addr_ton(addr_ton);
        self
    }

    /// Sets the address NPI (Numbering Plan Indicator).
    pub fn addr_npi(mut self, addr_npi: Npi) -> Self {
        self.bind_builder = self.bind_builder.addr_npi(addr_npi);
        self
    }

    /// Sets the address range.
    pub fn address_range(mut self, address_range: COctetString<1, 41>) -> Self {
        self.bind_builder = self.bind_builder.address_range(address_range);
        self
    }

    /// Sets the enquire link timeout.
    ///
    /// This timeout is used to determine how often an enquire link command should be sent to the server.
    pub fn enquire_link_timeout(mut self, enquire_link_timeout: Duration) -> Self {
        self.timeouts.enquire_link = enquire_link_timeout;
        self
    }

    /// Sets the response timeout.
    ///
    /// This timeout is used to determine how long the client should wait for a response from the server.
    pub fn response_timeout(mut self, response_timeout: Duration) -> Self {
        self.timeouts.response = response_timeout;
        self
    }
}
