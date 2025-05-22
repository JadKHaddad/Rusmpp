use std::{
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

use futures::{Sink, channel::mpsc::Sender};
use rusmpp::{
    Command,
    pdus::{BindReceiver, BindTransceiver, BindTransmitter},
    session::SessionState,
};
use tokio::sync::RwLock;

use crate::{action::Action, error::Error};

#[derive(Debug)]
pub struct Client {
    /// The client must not be generic over the sink type, because it should be easy to use.
    inner: Arc<ClientInner<Sender<Action>>>,
}

impl Deref for Client {
    type Target = ClientInner<Sender<Action>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Client {
    pub(crate) fn new(
        actions_sink: Sender<Action>,
        session_state: Arc<RwLock<SessionState>>,
    ) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(actions_sink, session_state)),
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ClientInner<Sink> {
    actions_sink: Sink,
    sequence_number: AtomicU32,
    session_state: Arc<RwLock<SessionState>>,
}

impl<Si> ClientInner<Si> {
    const fn new(actions_sink: Si, session_state: Arc<RwLock<SessionState>>) -> Self {
        Self {
            actions_sink,
            sequence_number: AtomicU32::new(0),
            session_state,
        }
    }
}

impl<Si> ClientInner<Si>
where
    Si: Sink<Action> + Unpin + 'static,
{
    fn next_sequence_number(&self) -> u32 {
        self.sequence_number.fetch_add(1, Ordering::Relaxed)
    }

    pub(crate) async fn bind_transmitter(&self, bind: BindTransmitter) -> Result<Command, Error> {
        todo!()
    }

    pub(crate) async fn bind_receiver(&self, bind: BindReceiver) -> Result<Command, Error> {
        todo!()
    }

    pub(crate) async fn bind_transceiver(&self, bind: BindTransceiver) -> Result<Command, Error> {
        todo!()
    }
}
