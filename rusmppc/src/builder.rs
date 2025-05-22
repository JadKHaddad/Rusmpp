use std::net::SocketAddr;

use futures::{
    Stream,
    channel::mpsc::{channel, unbounded},
};
use tokio::net::TcpStream;

use crate::{
    Client, Event,
    action::Action,
    connection::{self, Connection, ConnectionConfig},
    error::Error,
};

#[derive(Debug)]
pub struct ConnectionBuilder {
    socket_addr: SocketAddr,
    connection_config: ConnectionConfig,
}

impl ConnectionBuilder {
    async fn connect(self) -> Result<(Client, impl Stream<Item = Event> + Unpin + 'static), Error> {
        let stream = TcpStream::connect(self.socket_addr)
            .await
            .map_err(Error::Connect)?;

        let (events_tx, events_rx) = unbounded::<Event>();
        let (actions_tx, actions_rx) = channel::<Action>(100);

        let client = Client::new(actions_tx);
        let connection = Connection::new(stream, events_tx, actions_rx, self.connection_config);

        connection.spawn();

        Ok((client, events_rx))
    }
}

impl ConnectionBuilder {
    pub fn socket_addr(mut self, socket_addr: impl Into<SocketAddr>) -> Self {
        self.socket_addr = socket_addr.into();
        self
    }
}
