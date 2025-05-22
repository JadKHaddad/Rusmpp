use std::{net::SocketAddr, sync::Arc};

use futures::{
    Stream,
    channel::mpsc::{channel, unbounded},
};
use rusmpp::session::SessionState;
use tokio::{net::TcpStream, sync::RwLock};

use crate::{
    Client, Event,
    action::Action,
    bind::BindMode,
    connection::{self, Connection, ConnectionConfig},
    error::Error,
};

#[derive(Debug)]
pub struct ConnectionBuilder {
    socket_addr: SocketAddr,
    bind_mode: BindMode,
    // TODO: Add BindAny pdu to rusmpp.
    connection_config: ConnectionConfig,
}

impl ConnectionBuilder {
    async fn connect(self) -> Result<(Client, impl Stream<Item = Event> + Unpin + 'static), Error> {
        let stream = TcpStream::connect(self.socket_addr)
            .await
            .map_err(Error::Connect)?;

        let (events_tx, events_rx) = unbounded::<Event>();
        let (actions_tx, actions_rx) = channel::<Action>(100);

        let session_state = Arc::new(RwLock::new(SessionState::Open));

        let connection = Connection::new(
            stream,
            events_tx,
            actions_rx,
            session_state.clone(),
            self.connection_config,
        );

        connection.spawn();

        let client = Client::new(actions_tx, session_state);

        // Send the bind and wait for it here before returning the client.

        Ok((client, events_rx))
    }
}

impl ConnectionBuilder {
    pub fn socket_addr(mut self, socket_addr: impl Into<SocketAddr>) -> Self {
        self.socket_addr = socket_addr.into();
        self
    }

    pub const fn bind_mode(mut self, bind_mode: BindMode) -> Self {
        self.bind_mode = bind_mode;
        self
    }

    pub const fn bind_transmitter(mut self) -> Self {
        self.bind_mode = BindMode::Tx;
        self
    }

    pub const fn bind_receiver(mut self) -> Self {
        self.bind_mode = BindMode::Rx;
        self
    }

    pub const fn bind_transceiver(mut self) -> Self {
        self.bind_mode = BindMode::TxRx;
        self
    }
}
