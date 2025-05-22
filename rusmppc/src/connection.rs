use std::time::Duration;

use futures::{Sink, SinkExt, Stream, StreamExt, channel::oneshot::Cancellation};
use rusmpp::{codec::CommandCodec, session::SessionState};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::{
    codec::{FramedRead, FramedWrite},
    sync::CancellationToken,
};

use crate::{
    Event,
    action::{self, Action},
    error::Error,
    session_state::SessionStateHolder,
};

#[derive(Debug)]
pub struct ConnectionConfig {
    timeouts: ConnectionTimeouts,
}

#[derive(Debug)]
pub struct ConnectionTimeouts {
    session: Duration,
    enquire_link: Duration,
    inactivity: Duration,
    response: Duration,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeouts: ConnectionTimeouts {
                session: Duration::from_secs(5),
                enquire_link: Duration::from_secs(30),
                inactivity: Duration::from_secs(60),
                response: Duration::from_secs(5),
            },
        }
    }
}

#[derive(Debug)]
pub struct Connection<Socket, Sink, Stream> {
    socket: Socket,
    /// Send smpp events to the user.
    events_sink: Sink,
    /// Receive smpp actions from the client.
    actions_stream: Stream,
    session_state_holder: SessionStateHolder,
    config: ConnectionConfig,
}

impl<So, Si, St> Connection<So, Si, St> {
    pub const fn new(
        socket: So,
        events_sink: Si,
        actions_stream: St,
        session_state_holder: SessionStateHolder,
        config: ConnectionConfig,
    ) -> Self {
        Self {
            socket,
            events_sink,
            actions_stream,
            session_state_holder,
            config,
        }
    }
}

impl<So, Si, St> Connection<So, Si, St>
where
    So: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    Si: Sink<Event> + Send + Unpin + 'static,
    St: Stream<Item = Action> + Send + Unpin + 'static,
{
    pub fn spawn(self) {
        let (reader, writer) = tokio::io::split(self.socket);

        let mut smpp_reader = FramedRead::new(reader, CommandCodec::new());
        let smpp_writer = FramedWrite::new(writer, CommandCodec::new());

        let session_state_holder = self.session_state_holder;

        let mut actions_stream = self.actions_stream;
        let mut events_sink = self.events_sink;

        let cancellation_token = CancellationToken::new();
        let reader_token = cancellation_token.clone();
        let writer_token = cancellation_token.clone();

        tokio::spawn(async move {
            const TARGET: &str = "rusmppc::connection::reader";

            tracing::trace!(target: TARGET, "Reader task started");

            loop {
                tokio::select! {
                    _ = reader_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Reader task cancelled");
                    },
                    command = smpp_reader.next() => {
                        let Some(command) = command else {
                            tracing::debug!(target: TARGET, "End of stream");

                            break;
                        };

                        match command {
                            Ok(command) => {
                                tracing::trace!(target: TARGET, ?command, "Received command");
                            }
                            Err(err) => {
                                let err = Error::from(err);

                                tracing::error!(target: TARGET, ?err, "Error reading command");

                                let _ = events_sink.send(Event::Error(err)).await;

                                break;
                            },
                        }
                    }
                }
            }

            reader_token.cancel();

            tracing::debug!(target: TARGET, "Reader task terminated");
        });

        tokio::spawn(async move {
            const TARGET: &str = "rusmppc::connection::writer";

            tracing::trace!(target: TARGET, "Writer task started");

            loop {
                tokio::select! {
                    _ = writer_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Writer task cancelled");
                    },
                    action = actions_stream.next() => {
                        let Some(action) = action else {
                            tracing::debug!(target: TARGET, "No more actions");

                            break;
                        };

                        match action {
                            Action::SendCommand(send_command_action) => {},
                        }

                    }
                }
            }

            writer_token.cancel();

            session_state_holder.set(SessionState::Closed);

            tracing::debug!(target: TARGET, "Writer task terminated");
        });
    }
}
