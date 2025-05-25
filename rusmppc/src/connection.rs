use std::sync::atomic::{AtomicUsize, Ordering};

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
use tracing::Instrument;

use crate::{
    CommandExt, Event, PendingResponses,
    action::{Action, SendCommand, SendCommandNoResponse},
    builder::ConnectionTimeouts,
    error::Error,
    session_state::SessionStateHolder,
};

static ID: AtomicUsize = AtomicUsize::new(1);

fn next_id() -> usize {
    ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Default)]
pub struct ManagedConnectionConfig {
    max_command_length: usize,
    timeouts: ConnectionTimeouts,
}

impl ManagedConnectionConfig {
    pub const fn new(max_command_length: usize, timeouts: ConnectionTimeouts) -> Self {
        Self {
            max_command_length,
            timeouts,
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
    termination_token: CancellationToken,
    session_state_holder: SessionStateHolder,
    pending_responses: PendingResponses,
    config: ManagedConnectionConfig,
}

impl<So, Si, St> Connection<So, Si, St> {
    pub const fn new(
        socket: So,
        events_sink: Si,
        actions_stream: St,
        termination_token: CancellationToken,
        session_state_holder: SessionStateHolder,
        pending_responses: PendingResponses,
        config: ManagedConnectionConfig,
    ) -> Self {
        Self {
            socket,
            events_sink,
            termination_token,
            actions_stream,
            session_state_holder,
            pending_responses,
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
        let (enquire_link, reader, writer) = self.futures();

        let id = next_id();
        let _span = tracing::info_span!("connection", id).entered();

        tokio::spawn(enquire_link.instrument(tracing::info_span!("enquire_link")));
        tokio::spawn(reader.instrument(tracing::info_span!("reader")));
        tokio::spawn(writer.instrument(tracing::info_span!("writer")));
    }

    fn futures(
        self,
    ) -> (
        impl Future<Output = ()>,
        impl Future<Output = ()>,
        impl Future<Output = ()>,
    ) {
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

        let pending_responses = self.pending_responses;
        let reader_pending_responses = pending_responses.clone();
        let writer_pending_responses = pending_responses;

        // When this is cancelled, the client knows that the connection is closed
        let termination_token = self.termination_token;

        let enquire_link = async move {
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

                        let (action, response) = SendCommand::new(command);

                        let _ = intern_actions_tx.send(Action::SendCommand(action)).await;

                        match tokio::time::timeout(enquire_link_timeout, response)
                            .await {
                                Err(_) => {
                                    tracing::error!(target: TARGET, sequence_number, "Enquire link timeout");

                                    let _ = enquire_link_events_sink
                                            .send(Event::Error(Error::EnquireLinkTimeout { timeout: enquire_link_timeout }))
                                            .await;

                                    break
                                },
                                Ok(result) => match result {
                                    Ok(Ok(command)) => {
                                        let sequence_number = command.sequence_number();

                                        match command.ok_and_matches(CommandId::EnquireLinkResp) {
                                            Ok(_) => {
                                                tracing::trace!(target: TARGET, sequence_number, "Enquire link response received");

                                                continue
                                            },
                                            Err(command) => {
                                                tracing::error!(target: TARGET, sequence_number, ?command, "Unexpected enquire link response");

                                                let _ = enquire_link_events_sink
                                                    .send(Event::Error(Error::EnquireLinkFailed{ response: command }))
                                                    .await;

                                                break
                                            },
                                        }
                                    },
                                    Ok(Err(_)) => {
                                        // Failed to send enquire link

                                        break
                                    }
                                    Err(_) => {
                                        // Reader dropped and writer dropped
                                        // responses map is dropped

                                        break
                                    }
                                }
                            }
                    }
                }
            }

            enquire_link_token.cancel();

            tracing::debug!(target: TARGET, "Terminated");
        };

        let reader = async move {
            const TARGET: &str = "rusmppc::connection::reader";

            tracing::trace!(target: TARGET, "Started");

            loop {
                tokio::select! {
                    _ = reader_token.cancelled() => {
                        tracing::debug!(target: TARGET, "Cancelled");

                        break;
                    },
                    command = smpp_reader.next() => {
                        const TARGET: &str = "rusmppc::connection::reader::incoming";

                        let Some(command) = command else {
                            tracing::debug!(target: TARGET, "End of stream");

                            tracing::trace!(target: TARGET, session_state=?SessionState::Closed, "Setting session state");

                            reader_session_state_holder.set_session_state(SessionState::Closed);

                            break
                        };

                        match command {
                            Ok(command) => {
                                let sequence_number = command.sequence_number();

                                tracing::trace!(target: TARGET, sequence_number, ?command, "Received command");

                                if let CommandId::EnquireLink = command.id() {
                                    tracing::trace!(target: TARGET, sequence_number, "Enquire link received");

                                    let command = Command::builder()
                                        .status(CommandStatus::EsmeRok)
                                        .sequence_number(command.sequence_number())
                                        .pdu(Pdu::EnquireLinkResp);

                                    let _ = intern_tx.send(command).await;

                                    continue
                                }

                                if let CommandId::Unbind = command.id() {
                                    tracing::trace!(target: TARGET, sequence_number, "Unbind received");

                                    tracing::trace!(target: TARGET, sequence_number, session_state=?SessionState::Unbound, "Setting session state");

                                    reader_session_state_holder.set_session_state(SessionState::Unbound);

                                    let command = Command::builder()
                                        .status(CommandStatus::EsmeRok)
                                        .sequence_number(command.sequence_number())
                                        .pdu(Pdu::UnbindResp);

                                    let _ = intern_tx.send(command).await;

                                    continue
                                }

                                let command_id = command.id();

                                // The server may send us a request like DeliverSm, No clients are waiting for it so we pipe it to the events stream
                                if command_id.is_operation() {
                                    let _ = reader_events_sink.send(Event::Command(command)).await;

                                    continue;
                                }

                                if command_id.is_response() {
                                    let response = reader_pending_responses.lock().remove(&sequence_number);

                                    match response {
                                        None => {
                                            tracing::warn!(target: TARGET, sequence_number, "No client waiting for response");

                                            let _ = reader_events_sink.send(Event::Command(command)).await;
                                        },
                                        Some(response) => {
                                            tracing::trace!(target: TARGET, sequence_number, "Found client waiting for response");

                                            if let Err(command) = response.send(Ok(command)){
                                                tracing::warn!(target: TARGET, sequence_number, "Failed to send response to client");
                                                tracing::trace!(target: TARGET, sequence_number, "Piping command through events stream");

                                                let _ = reader_events_sink.send(Event::Command(command.expect("Must be ok"))).await;
                                            }
                                        }
                                    }
                                }

                                // The writer has sent an unbind and now waiting for the response
                                // The unbound response status is not checked
                                if let CommandId::UnbindResp = command_id {
                                    tracing::trace!(target: TARGET, sequence_number, "Unbind response received");

                                    // The writer is waiting for this to terminate gracefully
                                    let _ = intern_unbind_tx.send(());

                                    break
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
        };

        let writer = async move {
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

                        let sequence_number = command.sequence_number();

                        tracing::trace!(target: TARGET, sequence_number, ?command, "Sending command");

                        if let Err(err) = smpp_writer.send(&command).await {
                            let err = Error::from(err);

                            tracing::error!(target: TARGET, sequence_number, ?err, "Error sending command");

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
                            Action::SendCommand(SendCommand {command, response}) => {
                                let sequence_number = command.sequence_number();

                                tracing::trace!(target: TARGET, sequence_number, ?command, "Sending command");

                                if let Err(err) = smpp_writer.send(&command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, sequence_number, ?err, "Error sending command");

                                    let _ = response.send(Err(err));

                                    break
                                }

                                if let CommandId::Unbind = command.id() {
                                    tracing::debug!(target: TARGET, "Client requested unbind");

                                    tracing::trace!(target: TARGET, sequence_number, session_state=?SessionState::Unbound, "Setting session state");

                                    writer_session_state_holder.set_session_state(SessionState::Unbound);
                                }

                                writer_pending_responses.lock().insert(sequence_number, response);

                                tracing::trace!(target: TARGET, sequence_number, "Client registered for response");
                            },
                            Action::SendCommandNoResponse(SendCommandNoResponse {command, response}) => {
                                let sequence_number = command.sequence_number();

                                tracing::trace!(target: TARGET, sequence_number, ?command, "Sending command");

                                if let Err(err) = smpp_writer.send(&command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, sequence_number, ?err, "Error sending command");

                                    let _ = response.send(Err(err));

                                    break
                                }

                                let _ = response.send(Ok(()));
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
                            Action::SendCommand(SendCommand {command, response, ..}) => {
                                let sequence_number = command.sequence_number();

                                tracing::trace!(target: TARGET, sequence_number, ?command, "Sending command");

                                if let Err(err) = smpp_writer.send(&command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, sequence_number, ?err, "Error sending command");

                                    let _ = response.send(Err(err));

                                    break
                                }

                                let sequence_number = command.sequence_number();

                                writer_pending_responses.lock().insert(sequence_number, response);
                            },
                            _ => {
                                // Internal actions always wait for a response
                            }
                        }
                    }
                }
            }

            let session_state = writer_session_state_holder.session_state();

            match session_state {
                SessionState::Closed => {
                    // End of stream was reached
                    // Session state was set to closed and the token has been cancelled
                }
                SessionState::Unbound | SessionState::Open => {
                    // Already received unbind request from server
                    // Or
                    // We were not bound to begin with

                    // Terminating normally

                    writer_token.cancel();

                    tracing::trace!(target: TARGET, session_state=?SessionState::Closed, "Setting session state");

                    writer_session_state_holder.set_session_state(SessionState::Closed);
                }
                _ => {
                    // We unbind here

                    tracing::trace!(target: TARGET, session_state=?SessionState::Unbound, "Setting session state");

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

                    // The unbound response status is not checked
                    match tokio::time::timeout(response_timeout, intern_unbind_rx).await {
                        Ok(Ok(_)) => {
                            tracing::debug!(target: TARGET, "Unbound successfully");
                        }
                        Ok(Err(_)) | Err(_) => {
                            tracing::warn!(target: TARGET, "Unbind response timed out");
                        }
                    }

                    writer_token.cancel();

                    writer_session_state_holder.set_session_state(SessionState::Closed);
                }
            }

            tracing::debug!(target: TARGET, "Terminated");

            termination_token.cancel();
        };

        (enquire_link, reader, writer)
    }
}
