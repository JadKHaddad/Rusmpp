use std::{net::SocketAddr, sync::Arc};

use futures::{
    Stream,
    channel::mpsc::{channel, unbounded},
};
use rusmpp::{
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, builders::BindAnyBuilder},
    session::SessionState,
    types::COctetString,
    values::{InterfaceVersion, Npi, Ton},
};
use tokio::{net::TcpStream, sync::RwLock};

use crate::{
    Client, Event,
    action::Action,
    bind::BindMode,
    connection::{Connection, ConnectionConfig},
    error::Error,
};

#[derive(Debug)]
pub struct ConnectionBuilder {
    socket_addr: SocketAddr,
    bind_mode: BindMode,
    bind_builder: BindAnyBuilder,
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

    pub fn interface_version(mut self, interface_version: InterfaceVersion) -> Self {
        self.bind_builder = self.bind_builder.interface_version(interface_version);
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
}
