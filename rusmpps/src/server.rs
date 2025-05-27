use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Context;
use tokio::{net::TcpListener, sync::RwLock};

use crate::{client::Client, config::Config, connection::Connection};

#[derive(Debug)]
pub struct ServerParameters {
    pub clients: Vec<Client>,
    pub enquire_link_interval: Duration,
    pub response_timeout: Duration,
    pub session_timeout: Duration,
    pub bind_delay: Duration,
    pub response_delay: Duration,
    pub socket_addr: SocketAddr,
}

#[derive(Debug)]
pub struct Server {
    config: Arc<Config>,
    socket_addr: SocketAddr,
}

impl Server {
    pub fn new(parameters: ServerParameters) -> Self {
        let config = Arc::new(Config {
            connected_clients: Arc::new(RwLock::new(HashMap::new())),
            clients: parameters.clients,
            enquire_link_interval: parameters.enquire_link_interval,
            response_timeout: parameters.response_timeout,
            session_timeout: parameters.session_timeout,
            bind_delay: parameters.bind_delay,
            response_delay: parameters.response_delay,
        });

        Self {
            config,
            socket_addr: parameters.socket_addr,
        }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let listener = TcpListener::bind(self.socket_addr)
            .await
            .context("Failed to bind")?;

        tracing::info!(socket_addr=%self.socket_addr, "Listening");

        loop {
            let (stream, addr) = listener
                .accept()
                .await
                .context("Failed to accept connection")?;

            tracing::info!(%addr, "Accepted connection");

            let connection = Connection::new(self.config.clone());

            tokio::spawn(async move {
                connection.run(stream).await;

                tracing::info!(%addr, "Connection closed");
            });
        }
    }
}
