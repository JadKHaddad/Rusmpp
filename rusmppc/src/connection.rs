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
    max_command_length: usize,
    timeouts: ConnectionTimeouts,
}

impl ConnectionConfig {
    pub const fn new(max_command_length: usize, timeouts: ConnectionTimeouts) -> Self {
        Self {
            max_command_length,
            timeouts,
        }
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
        let (intern_unbind_tx, intern_unbind_rx) = oneshot::channel::<()>();

        let mut smpp_reader = FramedRead::new(
            reader,
            CommandCodec::new().with_max_length(self.config.max_command_length),
        );
        let mut smpp_writer = FramedWrite::new(
            writer,
            CommandCodec::new().with_max_length(self.config.max_command_length),
        );

        let reader_session_state_holder = self.session_state_holder.clone();
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
        let response_timeout = self.config.timeouts.response;

        let responses: Responses = Arc::new(parking_lot::Mutex::new(HashMap::new()));
        let reader_responses = responses.clone();
        let writer_responses = responses;

        tokio::spawn(async move {
            const TARGET: &str = "rusmppc::connection::enquire_link";

            tracing::trace!(target: TARGET, "Started");

            loop {
                tokio::select! {
                    _ = enquire_link_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Cancelled");

                        break
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

                                    break
                                },
                                Ok(result) => match result {
                                    Ok(Ok(command)) => {
                                        match command.is_ok_and_matches(CommandId::EnquireLinkResp) {
                                            Ok(_) => {
                                                tracing::trace!(target: TARGET, "Enquire link response received");

                                                continue
                                            },
                                            Err(command) => {
                                                tracing::error!(target: TARGET, ?command, "Unexpected enquire link response");

                                                let _ = enquire_link_events_sink
                                                    .send(Event::Error(Error::EnquireLinkFailed{response: Box::new(command)}))
                                                    .await;

                                                break
                                            },
                                        }
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

            tracing::debug!(target: TARGET, "Terminated");
        });

        tokio::spawn(async move {
            const TARGET: &str = "rusmppc::connection::reader";

            tracing::trace!(target: TARGET, "Started");

            loop {
                tokio::select! {
                    _ = reader_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Cancelled");

                        break;
                    },
                    command = smpp_reader.next() => {
                        let Some(command) = command else {
                            tracing::debug!(target: TARGET, "End of stream");

                            break
                        };

                        match command {
                            Ok(command) => {
                                tracing::trace!(target: TARGET, ?command, "Received command");

                                if let CommandId::EnquireLink = command.id() {
                                    tracing::trace!(target: TARGET, "Enquire link received");

                                    let command = Command::builder()
                                        .status(CommandStatus::EsmeRok)
                                        .sequence_number(command.sequence_number())
                                        .pdu(Pdu::EnquireLinkResp);

                                    let _ = intern_tx.send(command).await;

                                    continue
                                }

                                if let CommandId::Unbind = command.id() {
                                    tracing::trace!(target: TARGET, "Unbind received");

                                    reader_session_state_holder.set_session_state(SessionState::Unbound);

                                    let command = Command::builder()
                                        .status(CommandStatus::EsmeRok)
                                        .sequence_number(command.sequence_number())
                                        .pdu(Pdu::UnbindResp);

                                    let _ = intern_tx.send(command).await;

                                    continue
                                }

                                // The writer has sent an unbind and now waiting for the response
                                if let CommandId::UnbindResp = command.id() {
                                    tracing::trace!(target: TARGET, "Unbind response received");

                                    // The writer is waiting for this to terminate gracefully
                                    let _ = intern_unbind_tx.send(());

                                    break
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

                                break
                            },
                        }
                    }
                }
            }

            reader_token.cancel();

            tracing::debug!(target: TARGET, "Terminated");
        });

        tokio::spawn(async move {
            const TARGET: &str = "rusmppc::connection::writer";

            tracing::trace!(target: TARGET, "Started");

            loop {
                tokio::select! {
                    _ = writer_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Cancelled");

                        break
                    },
                    command = intern_rx.next() => {
                        const TARGET: &str = "rusmppc::connection::writer::intern";

                        let Some(command) = command else {
                            tracing::debug!(target: TARGET, "Tx dropped");

                            break
                        };

                        tracing::trace!(target: TARGET, ?command, "Sending command");

                        if let Err(err) = smpp_writer.send(&command).await {
                            let err = Error::from(err);

                            tracing::error!(target: TARGET, ?err, "Error sending command");

                            let _ = writer_events_sink.send(Event::Error(err)).await;

                            break
                        }

                        // We received an Unbind request from server and we responded with unbind response
                        // we should close the connection.
                        if let CommandId::UnbindResp = command.id() {
                            break
                        }
                    }
                    action = actions_stream.next() => {
                        const TARGET: &str = "rusmppc::connection::writer::actions";

                        let Some(action) = action else {
                            tracing::debug!(target: TARGET, "No more client actions");

                            break
                        };

                        match action {
                            Action::SendCommand(SendCommandAction {command, response}) => {
                                tracing::trace!(target: TARGET, ?command, "Sending command");

                                if let Err(err) = smpp_writer.send(&command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, ?err, "Error sending command");

                                    let _ = response.send(Err(err));

                                    break
                                }

                                let sequence_number = command.sequence_number();

                                writer_responses.lock().insert(sequence_number, response);
                            },
                        }
                    }
                    action = intern_actions_rx.next() => {
                        const TARGET: &str = "rusmppc::connection::writer::intern::actions";

                        let Some(action) = action else {
                            tracing::debug!(target: TARGET, "Tx dropped");

                            break
                        };

                        match action {
                            // TODO: this is duplicated code
                            Action::SendCommand(SendCommandAction {command, response}) => {
                                tracing::trace!(target: TARGET, ?command, "Sending command");

                                if let Err(err) = smpp_writer.send(&command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, ?err, "Error sending command");

                                    let _ = response.send(Err(err));

                                    break
                                }

                                let sequence_number = command.sequence_number();

                                writer_responses.lock().insert(sequence_number, response);
                            },
                        }
                    }
                }
            }

            let session_state = writer_session_state_holder.session_state();

            match session_state {
                SessionState::Unbound => {
                    // Already received unbind request from server
                    // Nothing to do here
                }
                _ => {
                    // We unbind here
                    writer_session_state_holder.set_session_state(SessionState::Unbound);

                    let sequence_number = writer_session_state_holder.next_sequence_number();
                    let unbind = Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(Pdu::Unbind);

                    tracing::trace!(target: TARGET, "Sending unbind");

                    if let Err(err) = smpp_writer.send(unbind).await {
                        let err = Error::from(err);

                        tracing::error!(target: TARGET, ?err, "Error sending command");

                        let _ = writer_events_sink.send(Event::Error(err)).await;

                        // At this point we don not really care if something wrong happens we are terminating
                    }

                    // Wait for an unbind response to terminate gracefully
                    tracing::trace!(target: TARGET, "Waiting for unbind response");

                    match tokio::time::timeout(response_timeout, intern_unbind_rx).await {
                        Ok(_) => {
                            tracing::debug!(target: TARGET, "Unbound successfully");
                        }
                        Err(_) => {
                            tracing::warn!(target: TARGET, "Unbind response timed out");
                        }
                    }
                }
            }

            writer_token.cancel();

            writer_session_state_holder.set_session_state(SessionState::Closed);

            tracing::debug!(target: TARGET, "Terminated");
        });
    }
}
