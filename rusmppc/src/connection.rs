use std::sync::Arc;

use futures::{Sink, Stream};
use rusmpp::session::SessionState;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::RwLock,
};

use crate::{Event, action::Action};

#[derive(Debug)]
pub struct ConnectionConfig {
    timeouts: ConnectionTimeouts,
}

#[derive(Debug)]
pub struct ConnectionTimeouts {}

#[derive(Debug)]
pub struct Connection<Socket, Sink, Stream> {
    socket: Socket,
    /// Send smpp events to the user.
    events_sink: Sink,
    /// Receive smpp actions from the client.
    actions_stream: Stream,
    session_state: Arc<RwLock<SessionState>>,
    config: ConnectionConfig,
}

impl<So, Si, St> Connection<So, Si, St> {
    pub const fn new(
        socket: So,
        events_sink: Si,
        actions_stream: St,
        session_state: Arc<RwLock<SessionState>>,
        config: ConnectionConfig,
    ) -> Self {
        Self {
            socket,
            events_sink,
            actions_stream,
            session_state,
            config,
        }
    }
}

impl<So, Si, St> Connection<So, Si, St>
where
    So: AsyncRead + AsyncWrite + Unpin + 'static,
    Si: Sink<Event> + Unpin + 'static,
    St: Stream<Item = Action> + Unpin + 'static,
{
    pub fn spawn(self) {
        tokio::spawn(async move {});
    }
}
