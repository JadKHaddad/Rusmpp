use std::{ops::Deref, sync::Arc};

use futures::{Sink, channel::mpsc::Sender};
use rusmpp::{
    Command,
    pdus::{BindReceiver, BindTransceiver, BindTransmitter},
    session::SessionState,
};

use crate::{action::Action, error::Error, session_state::SessionStateHolder};

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
        session_state_holder: SessionStateHolder,
    ) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(actions_sink, session_state_holder)),
        }
    }

    pub fn session_state(&self) -> SessionState {
        self.session_state_holder.session_state()
    }

    pub fn sequence_number(&self) -> u32 {
        self.session_state_holder.sequence_number()
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ClientInner<Sink> {
    actions_sink: Sink,
    session_state_holder: SessionStateHolder,
}

impl<Si> ClientInner<Si> {
    const fn new(actions_sink: Si, session_state_holder: SessionStateHolder) -> Self {
        Self {
            actions_sink,
            session_state_holder,
        }
    }
}

impl<Si> ClientInner<Si>
where
    Si: Sink<Action> + Unpin + 'static,
{
    pub(crate) async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<Command, Error> {
        todo!()
    }

    pub(crate) async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<Command, Error> {
        todo!()
    }

    pub(crate) async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<Command, Error> {
        todo!()
    }
}
