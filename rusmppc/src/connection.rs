use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use futures::{Sink, SinkExt, Stream, StreamExt};
use rusmpp::{Command, CommandId, CommandStatus, Pdu, codec::CommandCodec, session::SessionState};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::oneshot,
};
use tokio_util::{codec::Framed, sync::CancellationToken};
use tracing::Instrument;

use crate::{
    Event,
    action::{Action, SendCommand, SendCommandNoResponse},
    builder::ConnectionTimeouts,
    error::Error,
    session_state::SessionStateHolder,
    timer::Timer,
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
    config: ManagedConnectionConfig,
}

impl<So, Si, St> Connection<So, Si, St> {
    pub const fn new(
        socket: So,
        events_sink: Si,
        actions_stream: St,
        termination_token: CancellationToken,
        session_state_holder: SessionStateHolder,
        config: ManagedConnectionConfig,
    ) -> Self {
        Self {
            socket,
            events_sink,
            termination_token,
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
        let id = next_id();

        tokio::spawn(self.run().instrument(tracing::info_span!("connection", id)));
    }
}

impl<So, Si, St> Connection<So, Si, St>
where
    So: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    Si: Sink<Event> + Send + Clone + Unpin + 'static,
    St: Stream<Item = Action> + Send + Unpin + 'static,
{
    pub async fn run(self) {
        let mut framed = Framed::new(
            self.socket,
            CommandCodec::new().with_max_length(self.config.max_command_length),
        );

        let session_state_holder = self.session_state_holder;

        let mut actions_stream = self.actions_stream;

        let mut events_sink = self.events_sink;

        let enquire_link_timeout = self.config.timeouts.enquire_link;
        let response_timeout = self.config.timeouts.response;

        let mut pending_responses: HashMap<u32, oneshot::Sender<Result<Command, Error>>> =
            HashMap::new();

        // When this is cancelled, the client knows that the connection is closed
        let termination_token = self.termination_token;

        let mut last_enquire_link_sequence_number: Option<u32> = None;

        let enquire_link_resp_timer = Timer::new();

        tokio::pin!(enquire_link_resp_timer);

        loop {
            tokio::select! {
                _ = &mut enquire_link_resp_timer => {
                    const TARGET: &str = "rusmppc::connection::enquire_link::timer";

                    tracing::error!(target: TARGET, "EnquireLink timeout");

                    break
                }
                _ = tokio::time::sleep(enquire_link_timeout) => {
                    const TARGET: &str = "rusmppc::connection::enquire_link";

                    let sequence_number = session_state_holder.next_sequence_number();

                    tracing::trace!(target: TARGET, sequence_number, "Sending EnquireLink");

                    let command = Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(Pdu::EnquireLink);

                    if let Err(err) = framed.send(command).await {
                        tracing::error!(target: TARGET, ?err, "Failed to send EnquireLink command");

                        break
                    }

                    last_enquire_link_sequence_number = Some(sequence_number);

                    enquire_link_resp_timer.as_mut().activate(response_timeout);

                    tracing::trace!(target: TARGET, "EnquireLink timer activated");
                }
                command = framed.next() => {
                    const TARGET: &str = "rusmppc::connection::read";

                    let Some(command) = command else {
                        tracing::debug!(target: TARGET, "End of stream");

                        tracing::trace!(target: TARGET, session_state=?SessionState::Closed, "Setting session state");

                        session_state_holder.set_session_state(SessionState::Closed);

                        break
                    };

                    match command {
                        Ok(command) => {
                            let sequence_number = command.sequence_number();

                            tracing::debug!(target: TARGET, sequence_number, id=?command.id(), "Received command");
                            tracing::trace!(target: TARGET, sequence_number, ?command, "Received command");

                            if let CommandId::EnquireLink = command.id() {
                                tracing::trace!(target: TARGET, sequence_number, "EnquireLink received");

                                let command = Command::builder()
                                    .status(CommandStatus::EsmeRok)
                                    .sequence_number(command.sequence_number())
                                    .pdu(Pdu::EnquireLinkResp);

                                if let Err(err) = framed.send(command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, ?err, "Failed to send EnquireLink response");

                                    let _ = events_sink.send(Event::Error(err)).await;

                                    break
                                }

                                continue
                            }

                            if let CommandId::Unbind = command.id() {
                                tracing::trace!(target: TARGET, sequence_number, "Unbind received");

                                tracing::trace!(target: TARGET, sequence_number, session_state=?SessionState::Unbound, "Setting session state");

                                session_state_holder.set_session_state(SessionState::Unbound);

                                let command = Command::builder()
                                    .status(CommandStatus::EsmeRok)
                                    .sequence_number(command.sequence_number())
                                    .pdu(Pdu::UnbindResp);

                                if let Err(err) = framed.send(command).await {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, ?err, "Failed to send Unbind response");

                                    let _ = events_sink.send(Event::Error(err)).await;

                                    break
                                }

                                break
                            }

                            if let CommandId::EnquireLinkResp = command.id() {
                                tracing::trace!(target: TARGET, sequence_number,  "Received EnquireLinkResp");

                                match last_enquire_link_sequence_number {
                                    Some(seq) => {
                                        if sequence_number != seq {
                                            tracing::warn!(target: TARGET, sequence_number, expected=seq, got=sequence_number, "Received EnquireLinkResp with unexpected sequence number");
                                        }

                                        last_enquire_link_sequence_number = None;

                                        enquire_link_resp_timer.as_mut().disable();

                                        tracing::trace!(target: TARGET, sequence_number,  "EnquireLink timer disabled");
                                    }
                                    None => {
                                        tracing::warn!(target: TARGET, sequence_number,  "Received EnquireLinkResp without a previous EnquireLink");
                                    }
                                }

                                continue
                            }

                            let command_id = command.id();

                            // The server may send us a request like DeliverSm, No clients are waiting for it so we pipe it to the events stream
                            if command_id.is_operation() {
                                let _ = events_sink.send(Event::Command(command)).await;

                                continue;
                            }

                            if command_id.is_response() {
                                let response = pending_responses.remove(&sequence_number);

                                match response {
                                    None => {
                                        tracing::warn!(target: TARGET, sequence_number, "No client waiting for response");

                                        let _ = events_sink.send(Event::Command(command)).await;
                                    },
                                    Some(response) => {
                                        tracing::trace!(target: TARGET, sequence_number, "Found client waiting for response");

                                        if let Err(command) = response.send(Ok(command)){
                                            tracing::warn!(target: TARGET, sequence_number, "Failed to send response to client");
                                            tracing::trace!(target: TARGET, sequence_number, "Piping command through events stream");

                                            let _ = events_sink.send(Event::Command(command.expect("Must be ok"))).await;
                                        }
                                    }
                                }
                            }

                            // we sent an Unbind request and now waiting for the response
                            if let CommandId::UnbindResp = command_id {
                                tracing::trace!(target: TARGET, sequence_number, "Unbind response received");

                                let session_state = session_state_holder.session_state();

                                if !matches!(session_state, SessionState::Unbound) {
                                    tracing::warn!(target: TARGET, session_state=?session_state, "Received UnbindResp in unexpected session state");

                                    continue
                                }

                                break
                            }
                        }
                        Err(err) => {
                            let err = Error::from(err);

                            tracing::error!(target: TARGET, ?err, "Error reading command");

                            let _ = events_sink.send(Event::Error(err)).await;

                            break
                        },
                    }
                }
                action = actions_stream.next() => {
                    const TARGET: &str = "rusmppc::connection::actions";

                    let Some(action) = action else {
                        tracing::debug!(target: TARGET, "No more client actions");

                        break
                    };

                    match action {
                        Action::RemovePendingResponse(sequence_number) => {
                            tracing::trace!(target: TARGET, sequence_number, "Removing pending response");

                            if pending_responses.remove(&sequence_number).is_none() {
                                tracing::warn!(target: TARGET, sequence_number, "No client waiting for response");
                            }
                        },
                        Action::SendCommand(SendCommand { command, response }) => {
                            let sequence_number = command.sequence_number();

                            tracing::debug!(target: TARGET, sequence_number, id=?command.id(), "Sending command");
                            tracing::trace!(target: TARGET, sequence_number, ?command, "Sending command");


                            if let Err(err) = framed.send(&command).await {
                                let err = Error::from(err);

                                tracing::error!(target: TARGET, sequence_number, ?err, "Error sending command");

                                let _ = response.send(Err(err));

                                break
                            }

                            if let CommandId::Unbind = command.id() {
                                tracing::debug!(target: TARGET, "Client requested unbind");

                                tracing::trace!(target: TARGET, sequence_number, session_state=?SessionState::Unbound, "Setting session state");

                                session_state_holder.set_session_state(SessionState::Unbound);
                            }

                            pending_responses.insert(sequence_number, response);

                            tracing::trace!(target: TARGET, sequence_number, "Client registered for response");
                        },
                        Action::SendCommandNoResponse(SendCommandNoResponse { command, response }) => {
                            let sequence_number = command.sequence_number();

                            tracing::debug!(target: TARGET, sequence_number, id=?command.id(), "Sending command");
                            tracing::trace!(target: TARGET, sequence_number, ?command, "Sending command");

                            if let Err(err) = framed.send(&command).await {
                                let err = Error::from(err);

                                tracing::error!(target: TARGET, sequence_number, ?err, "Error sending command");

                                let _ = response.send(Err(err));

                                break
                            }

                            let _ = response.send(Ok(()));
                        },
                    }
                }
            }
        }

        const TARGET: &str = "rusmppc::connection";

        let session_state = session_state_holder.session_state();

        match session_state {
            SessionState::Closed => {
                // End of stream was reached
                // Session state was set to closed
            }
            SessionState::Unbound | SessionState::Open => {
                // Already received unbind request from server
                // Or
                // Client requested unbind
                // Or
                // We were not bound to begin with

                // Terminating normally

                tracing::trace!(target: TARGET, session_state=?SessionState::Closed, "Setting session state");

                session_state_holder.set_session_state(SessionState::Closed);
            }
            _ => {
                // We unbind here

                tracing::trace!(target: TARGET, session_state=?SessionState::Unbound, "Setting session state");

                session_state_holder.set_session_state(SessionState::Unbound);

                let sequence_number = session_state_holder.next_sequence_number();

                let unbind = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(sequence_number)
                    .pdu(Pdu::Unbind);

                tracing::trace!(target: TARGET, "Sending unbind");

                if let Err(err) = framed.send(unbind).await {
                    let err = Error::from(err);

                    tracing::error!(target: TARGET, ?err, "Error sending command");

                    let _ = events_sink.send(Event::Error(err)).await;
                }

                // Wait for an unbind response to terminate gracefully
                tracing::trace!(target: TARGET, "Waiting for unbind response");

                tokio::select! {
                    _ = tokio::time::sleep(response_timeout) => {
                        tracing::warn!(target: TARGET, "Unbind response timed out");
                    },
                    _ = async {
                        while let Some(command) = framed.next().await {
                            match command {
                                Err(err) => {
                                    let err = Error::from(err);

                                    tracing::error!(target: TARGET, ?err, "Error reading unbind response");

                                    let _ = events_sink.send(Event::Error(err)).await;

                                    break
                                },
                                Ok(command) => {
                                    let sequence_number=command.sequence_number();

                                    match command.id() {
                                        CommandId::UnbindResp => {
                                            tracing::trace!(target: TARGET, sequence_number, "Received unbind response");

                                            tracing::debug!(target: TARGET, "Unbound successfully");

                                            break;
                                        },
                                        _ => {
                                            let _ = events_sink.send(Event::Command(command)).await;
                                        }
                                    }
                                }
                            }
                        }
                    } => {}
                }

                session_state_holder.set_session_state(SessionState::Closed);
            }
        }

        tracing::debug!(target: TARGET, "Terminated");

        termination_token.cancel();
    }
}
