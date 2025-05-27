use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use rusmpp::session::SessionState;
use tokio::sync::mpsc::Sender;

pub type ConnectedClients = HashMap<String, ConnectedClient>;

#[derive(Debug)]
pub struct Client {
    pub system_id: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Action {}

#[derive(Debug)]
pub struct ConnectedClient {
    pub tx: Sender<Action>,
    pub session_state: SessionState,
    sequence_number: AtomicU32,
}

impl ConnectedClient {
    pub fn new(sender: Sender<Action>, session_state: SessionState) -> Self {
        Self {
            tx: sender,
            session_state,
            sequence_number: AtomicU32::new(1),
        }
    }

    pub fn next_sequence_number(&self) -> u32 {
        self.sequence_number.fetch_add(1, Ordering::SeqCst)
    }
}
