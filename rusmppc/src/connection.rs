use std::{collections::HashMap, sync::Arc};

use futures::{Sink, SinkExt, Stream, StreamExt, channel::mpsc};
use rusmpp::{Command, CommandId, CommandStatus, Pdu, codec::CommandCodec, session::SessionState};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::oneshot,
};
use tokio_util::{
    codec::{FramedRead, FramedWrite},
    sync::CancellationToken,
};

use crate::{
    Event,
    action::{Action, SendCommandAction},
    builder::ConnectionTimeouts,
    error::Error,
    session_state::SessionStateHolder,
};

#[derive(Debug, Default)]
pub struct ConnectionConfig {
    timeouts: ConnectionTimeouts,
}

impl ConnectionConfig {
    pub const fn new(timeouts: ConnectionTimeouts) -> Self {
        Self { timeouts }
    }
}

type Responses = Arc<parking_lot::Mutex<HashMap<u32, oneshot::Sender<Result<Command, Error>>>>>;

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
    Si: Sink<Event> + Send + Clone + Unpin + 'static,
    St: Stream<Item = Action> + Send + Unpin + 'static,
{
    pub fn spawn(self) {
        let (reader, writer) = tokio::io::split(self.socket);
        let (mut intern_tx, mut intern_rx) = mpsc::unbounded::<Command>();
        let (mut intern_actions_tx, mut intern_actions_rx) = mpsc::unbounded::<Action>();

        let mut smpp_reader = FramedRead::new(reader, CommandCodec::new());
        let mut smpp_writer = FramedWrite::new(writer, CommandCodec::new());

        let writer_session_state_holder = self.session_state_holder.clone();
        let enquire_link_session_state_holder = self.session_state_holder;

        let mut actions_stream = self.actions_stream;
        let mut reader_events_sink = self.events_sink.clone();
        let mut writer_events_sink = self.events_sink.clone();
        let mut enquire_link_events_sink = self.events_sink;

        let cancellation_token = CancellationToken::new();
        let reader_token = cancellation_token.clone();
        let writer_token = cancellation_token.clone();
        let enquire_link_token = cancellation_token;

        let enquire_link_timeout = self.config.timeouts.enquire_link;

        let responses: Responses = Arc::new(parking_lot::Mutex::new(HashMap::new()));
        let reader_responses = responses.clone();
        let writer_responses = responses;

        tokio::spawn(async move {
            const TARGET: &str = "rusmppc::connection::enquire_link";

            tracing::trace!(target: TARGET, "Enquire link task started");

            loop {
                tokio::select! {
                    _ = enquire_link_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Enquire link task cancelled");

                        break;
                    },
                    _ = tokio::time::sleep(self.config.timeouts.enquire_link) => {
                        tracing::trace!(target: TARGET, "Sending enquire link");

                        let sequence_number = enquire_link_session_state_holder.next_sequence_number();

                        let command = Command::builder()
                            .status(CommandStatus::EsmeRok)
                            .sequence_number(sequence_number)
                            .pdu(Pdu::EnquireLink);

                        let (action, response) = SendCommandAction::new(command);

                        let _ = intern_actions_tx.send(Action::SendCommand(action)).await;

                        match tokio::time::timeout(enquire_link_timeout, response)
                            .await {
                                Err(timeout) => {
                                    tracing::error!(target: TARGET, ?timeout, "Enquire link timeout");

                                    let _ = enquire_link_events_sink
                                            .send(Event::Error(Error::EnquireLinkTimeout { timeout: enquire_link_timeout }))
                                            .await;

                                    break;
                                },
                                Ok(result) => match result {
                                    Ok(Ok(command)) => {
                                        if let Some(Pdu::EnquireLinkResp) = command.pdu() {
                                            if let CommandStatus::EsmeRok = command.status() {
                                                tracing::trace!(target: TARGET, "Enquire link response received");

                                                continue;
                                            }
                                        }

                                        tracing::error!(target: TARGET, ?command, "Unexpected enquire link response");

                                        let _ = enquire_link_events_sink
                                            .send(Event::Error(Error::EnquireLinkFailed{response: Box::new(command)}))
                                            .await;

                                        break;
                                    },
                                    Ok(Err(_)) => {
                                        unreachable!();
                                    }
                                    Err(_) => {
                                       unreachable!();
                                    }
                                }
                            }

                    }
                }
            }

            enquire_link_token.cancel();

            tracing::debug!(target: TARGET, "Enquire link task terminated");
        });

        tokio::spawn(async move {
            const TARGET: &str = "rusmppc::connection::reader";

            tracing::trace!(target: TARGET, "Reader task started");

            loop {
                tokio::select! {
                    _ = reader_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Reader task cancelled");

                        break;
                    },
                    command = smpp_reader.next() => {
                        let Some(command) = command else {
                            tracing::debug!(target: TARGET, "End of stream");

                            break;
                        };

                        match command {
                            Ok(command) => {
                                tracing::trace!(target: TARGET, ?command, "Received command");

                                if let CommandId::EnquireLink = command.id() {
                                    let command = Command::builder()
                                        .status(command.status())
                                        .sequence_number(command.sequence_number())
                                        .pdu(Pdu::EnquireLinkResp);

                                    let _ = intern_tx.send(command).await;
                                }

                                let sequence_number = command.sequence_number();

                                let response = reader_responses.lock().remove(&sequence_number);
                                match response {
                                    None => {
                                        tracing::trace!(target: TARGET, ?command, "No response for command");

                                        let _ = reader_events_sink.send(Event::Command(command)).await;
                                    },
                                    Some(response) => {
                                        let _ = response.send(Ok(command));
                                    }
                                }

                            }
                            Err(err) => {
                                let err = Error::from(err);

                                tracing::error!(target: TARGET, ?err, "Error reading command");

                                let _ = reader_events_sink.send(Event::Error(err)).await;

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
                    command = intern_rx.next() => {
                        let Some(command) = command else {
                            tracing::debug!(target: TARGET, "intern_tx dropped");

                            break;
                        };

                        tracing::trace!(target: TARGET, ?command, "Sending command");

                        if let Err(err) = smpp_writer.send(command).await {
                            let err = Error::from(err);

                            tracing::error!(target: TARGET, ?err, "Error sending command");

                            let _ = writer_events_sink.send(Event::Error(err)).await;

                            break;
                        }
                    }
                    action = actions_stream.next() => {
                        let Some(action) = action else {
                            tracing::debug!(target: TARGET, "No more client actions");

                            break;
                        };

                        match action {
                            Action::SendCommand(SendCommandAction {command, response}) => {
                                tracing::trace!(target: TARGET, ?command, "Sending command");

                                if let Err(err) = smpp_writer.send(&command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, ?err, "Error sending command");

                                    let _ = response.send(Err(err));

                                    break;
                                }

                                let sequence_number = command.sequence_number();

                                writer_responses.lock().insert(sequence_number, response);
                            },
                        }
                    }
                    action = intern_actions_rx.next() => {
                        let Some(action) = action else {
                            tracing::debug!(target: TARGET, "intern_actions_tx dropped");

                            break;
                        };

                        match action {
                            // TODO: this is duplicated code
                            Action::SendCommand(SendCommandAction {command, response}) => {
                                tracing::trace!(target: TARGET, ?command, "Sending command");

                                if let Err(err) = smpp_writer.send(&command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, ?err, "Error sending command");

                                    let _ = response.send(Err(err));

                                    break;
                                }

                                let sequence_number = command.sequence_number();

                                writer_responses.lock().insert(sequence_number, response);
                            },
                        }
                    }
                }
            }

            writer_token.cancel();

            writer_session_state_holder.set_session_state(SessionState::Closed);

            tracing::debug!(target: TARGET, "Writer task terminated");
        });
    }
}
