use std::{ops::Deref, sync::Arc};

use futures::{Sink, channel::mpsc::Sender};

use crate::action::Action;

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
    pub(crate) fn new(actions_sink: Sender<Action>) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(actions_sink)),
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ClientInner<Sink> {
    actions_sink: Sink,
}

impl<Si> ClientInner<Si> {
    const fn new(actions_sink: Si) -> Self {
        Self { actions_sink }
    }
}

impl<Si> ClientInner<Si> where Si: Sink<Action> + Unpin + 'static {}
