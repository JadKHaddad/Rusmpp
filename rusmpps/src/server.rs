use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Context;

use tokio::net::TcpListener;

use crate::{
    client::{Client, ConnectedClients},
    connection::{Connection, ConnectionConfig},
};

#[derive(Debug)]
pub struct ServerParameters {
    pub clients: Vec<Client>,
    pub enquire_link_interval: Duration,
    pub enquire_link_response_timeout: Duration,
    pub enquire_link_response_delay: Duration,
    pub session_timeout: Duration,
    pub bind_delay: Duration,
    pub response_delay: Duration,
    pub socket_addr: SocketAddr,
}

#[derive(Debug)]
pub struct Server {
    config: Arc<ConnectionConfig>,
    socket_addr: SocketAddr,
    session_id: u64,
}

impl Server {
    pub fn new(parameters: ServerParameters) -> Self {
        let config = Arc::new(ConnectionConfig {
            connected_clients: ConnectedClients::new(),
            clients: parameters.clients,
            enquire_link_interval: parameters.enquire_link_interval,
            enquire_link_response_timeout: parameters.enquire_link_response_timeout,
            session_timeout: parameters.session_timeout,
            bind_delay: parameters.bind_delay,
            response_delay: parameters.response_delay,
            enquire_link_response_delay: parameters.enquire_link_response_delay,
        });

        Self {
            config,
            socket_addr: parameters.socket_addr,
            session_id: 0,
        }
    }

    fn next_session_id(&mut self) -> u64 {
        let session_id = self.session_id;

        self.session_id += 1;

        session_id
    }

    pub async fn run(mut self) -> Result<(), anyhow::Error> {
        let listener = TcpListener::bind(self.socket_addr)
            .await
            .context("Failed to bind")?;

        tracing::info!(socket_addr=%self.socket_addr, "Listening");

        loop {
            let (stream, addr) = listener
                .accept()
                .await
                .context("Failed to accept connection")?;

            let session_id = self.next_session_id();

            tracing::info!(%addr, session_id, "Accepted connection");

            let connection = Connection::new(session_id, self.config.clone());

            tokio::spawn(async move {
                connection.run(stream).await;

                tracing::info!(%addr, session_id, "Connection closed");
            });
        }
    }
}
