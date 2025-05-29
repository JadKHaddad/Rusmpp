use std::time::Duration;

use crate::client::{Client, ConnectedClients};

#[derive(Debug)]
pub struct Config {
    pub connected_clients: ConnectedClients,
    pub clients: Vec<Client>,
    pub enquire_link_interval: Duration,
    pub response_timeout: Duration,
    pub session_timeout: Duration,
    pub bind_delay: Duration,
    pub response_delay: Duration,
}
