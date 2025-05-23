use std::{net::SocketAddr, time::Duration};

use futures::{Stream, channel::mpsc::unbounded};
use rusmpp::{
    pdus::builders::BindAnyBuilder,
    session::SessionState,
    types::COctetString,
    values::{InterfaceVersion, Npi, Ton},
};
use tokio::{net::TcpStream, sync::mpsc::channel};
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    Client, Event,
    action::Action,
    bind::BindMode,
    connection::{Connection, ConnectionConfig},
    error::Error,
    session_state::SessionStateHolder,
};

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
    session: Duration,
    pub enquire_link: Duration,
    /// Not used.
    inactivity: Duration,
    pub response: Duration,
}

impl Default for ConnectionTimeouts {
    fn default() -> Self {
        Self {
            session: Duration::from_secs(5),
            enquire_link: Duration::from_secs(30),
            inactivity: Duration::from_secs(60),
            response: Duration::from_secs(5),
        }
    }
}

impl ConnectionBuilder {
    pub fn new(socket_addr: impl Into<SocketAddr>) -> Self {
        Self {
            socket_addr: socket_addr.into(),
            bind_mode: Default::default(),
            bind_builder: BindAnyBuilder::default().interface_version(InterfaceVersion::Smpp5_0),
            max_command_length: 4096,
            timeouts: Default::default(),
        }
    }

    pub async fn connect(
        self,
    ) -> Result<(Client, impl Stream<Item = Event> + Unpin + 'static), Error> {
        tracing::debug!(target: "rusmppc::connection", socket_addr=%self.socket_addr, "Connecting");

        let stream = TcpStream::connect(self.socket_addr)
            .await
            .map_err(Error::Connect)?;

        tracing::trace!(target: "rusmppc::connection", socket_addr=%self.socket_addr, "Connected");

        let (events_tx, events_rx) = unbounded::<Event>();
        let (actions_tx, actions_rx) = channel::<Action>(100);

        let session_state_holder = SessionStateHolder::new(SessionState::Open);

        let response_timeout = self.timeouts.response;

        let connection = Connection::new(
            stream,
            events_tx,
            ReceiverStream::new(actions_rx),
            session_state_holder.clone(),
            ConnectionConfig::new(self.max_command_length, self.timeouts),
        );

        connection.spawn();

        let client = Client::new(actions_tx, response_timeout, session_state_holder);

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
    pub fn socket_addr(mut self, socket_addr: impl Into<SocketAddr>) -> Self {
        self.socket_addr = socket_addr.into();
        self
    }

    pub fn max_command_length(mut self, max_command_length: usize) -> Self {
        self.max_command_length = max_command_length;
        self
    }

    pub const fn bind_mode(mut self, bind_mode: BindMode) -> Self {
        self.bind_mode = bind_mode;
        self
    }

    pub const fn transmitter(mut self) -> Self {
        self.bind_mode = BindMode::Tx;
        self
    }

    pub const fn receiver(mut self) -> Self {
        self.bind_mode = BindMode::Rx;
        self
    }

    pub const fn transceiver(mut self) -> Self {
        self.bind_mode = BindMode::TxRx;
        self
    }

    pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
        self.bind_builder = self.bind_builder.system_id(system_id);
        self
    }

    pub fn password(mut self, password: COctetString<1, 9>) -> Self {
        self.bind_builder = self.bind_builder.password(password);
        self
    }

    pub fn system_type(mut self, system_type: COctetString<1, 13>) -> Self {
        self.bind_builder = self.bind_builder.system_type(system_type);
        self
    }

    pub fn addr_ton(mut self, addr_ton: Ton) -> Self {
        self.bind_builder = self.bind_builder.addr_ton(addr_ton);
        self
    }

    pub fn addr_npi(mut self, addr_npi: Npi) -> Self {
        self.bind_builder = self.bind_builder.addr_npi(addr_npi);
        self
    }

    pub fn address_range(mut self, address_range: COctetString<1, 41>) -> Self {
        self.bind_builder = self.bind_builder.address_range(address_range);
        self
    }

    pub fn session_timeout(mut self, session_timeout: Duration) -> Self {
        self.timeouts.session = session_timeout;
        self
    }

    pub fn enquire_link_timeout(mut self, enquire_link_timeout: Duration) -> Self {
        self.timeouts.enquire_link = enquire_link_timeout;
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: Duration) -> Self {
        self.timeouts.inactivity = inactivity_timeout;
        self
    }

    pub fn response_timeout(mut self, response_timeout: Duration) -> Self {
        self.timeouts.response = response_timeout;
        self
    }
}
