use std::{
    net::SocketAddr,
    ops::Deref,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use parking_lot::Mutex;
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, SubmitSm, SubmitSmResp},
    session::SessionState,
};
use tokio::sync::mpsc::{Receiver, UnboundedSender};

use crate::{
    CommandExt, ConnectionBuilder,
    action::{Action, SendCommand, SendCommandNoResponse},
    error::Error,
    session_state::SessionStateHolder,
};

const TARGET: &str = "rusmppc::client";

/// `SMPP` Client.
///
/// The client is a handle to communicate with the `SMPP` server through a managed connection in the background.
///
/// When all clients are dropped, an `unbind` command is sent to the server, and the connection is closed.
#[derive(Debug)]
pub struct Client {
    inner: Arc<ClientInner>,
}

// TODO: remove the deref impl and move the methods to the `Client` struct.
// They must appear in the public api of the `Client`.
impl Deref for Client {
    type Target = ClientInner;

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
    /// Creates a new `SMPP` client.
    pub(crate) fn new(
        actions_sink: UnboundedSender<Action>,
        response_timeout: Duration,
        session_state_holder: SessionStateHolder,
        termination_rx: Receiver<()>,
    ) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(
                actions_sink,
                response_timeout,
                session_state_holder,
                termination_rx,
            )),
        }
    }

    /// Creates a new `SMPP` client builder.
    pub fn builder(socket_addr: impl Into<SocketAddr>) -> ConnectionBuilder {
        ConnectionBuilder::new(socket_addr)
    }

    /// Returns the current session state of the client.
    pub fn session_state(&self) -> SessionState {
        self.session_state_holder.session_state()
    }

    /// Returns the current sequence number of the client.
    pub fn sequence_number(&self) -> u32 {
        self.session_state_holder.sequence_number()
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ClientInner {
    /// Must be unbounded or else we need `AsyncDrop`. See [`RequestFuture`].
    actions_sink: UnboundedSender<Action>,
    response_timeout: Duration,
    session_state_holder: SessionStateHolder,
    /// Await the termination receiver to ensure that the connection tasks were terminated
    termination_rx: Mutex<Option<Receiver<()>>>,
}

impl ClientInner {
    const fn new(
        actions_sink: UnboundedSender<Action>,
        response_timeout: Duration,
        session_state_holder: SessionStateHolder,
        termination_rx: Receiver<()>,
    ) -> Self {
        Self {
            actions_sink,
            response_timeout,
            session_state_holder,
            termination_rx: Mutex::new(Some(termination_rx)),
        }
    }
}

impl ClientInner {
    fn next_sequence_number(&self) -> u32 {
        self.session_state_holder.next_sequence_number()
    }

    fn session_state(&self) -> SessionState {
        self.session_state_holder.session_state()
    }

    fn set_session_state(&self, session_state: SessionState) {
        self.session_state_holder.set_session_state(session_state)
    }

    pub(crate) async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<Command, Error> {
        let response = self.bind(bind.into()).await?;

        let response = response
            .ok_and_matches(CommandId::BindTransmitterResp)
            .map_err(Error::unexpected_response)?;

        self.set_session_state(SessionState::BoundTx);

        Ok(response)
    }

    pub(crate) async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<Command, Error> {
        let response = self.bind(bind.into()).await?;

        let response = response
            .ok_and_matches(CommandId::BindReceiverResp)
            .map_err(Error::unexpected_response)?;

        self.set_session_state(SessionState::BoundRx);

        Ok(response)
    }

    pub(crate) async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<Command, Error> {
        let response = self.bind(bind.into()).await?;

        let response = response
            .ok_and_matches(CommandId::BindTransceiverResp)
            .map_err(Error::unexpected_response)?;

        self.set_session_state(SessionState::BoundTrx);

        Ok(response)
    }

    // TODO: bind is same
    // TODO: do we want to check or save the interface version of the server?
    async fn bind(&self, pdu: impl Into<Pdu>) -> Result<Command, Error> {
        let sequence_number = self.next_sequence_number();

        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let (action, response) = SendCommand::new(command);

        self.actions_sink
            .send(Action::SendCommand(action))
            .map_err(|_| Error::ConnectionClosed)?;

        tokio::time::timeout(self.response_timeout, response)
            .await
            .map_err(|_| Error::Timeout)?
            .map_err(|_| Error::ConnectionClosed)?
    }

    fn request(&self, pdu: impl Into<Pdu>) -> impl Future<Output = Result<Command, Error>> {
        let sequence_number = self.next_sequence_number();

        let future = async move {
            let command = Command::builder()
                .status(CommandStatus::EsmeRok)
                .sequence_number(sequence_number)
                .pdu(pdu.into());

            let (action, response) = SendCommand::new(command);

            self.actions_sink
                .send(Action::SendCommand(action))
                .map_err(|_| Error::ConnectionClosed)?;

            tokio::time::timeout(self.response_timeout, response)
                .await
                .map_err(|_| Error::Timeout)
                .inspect_err(|_| {
                    tracing::warn!(target: TARGET, sequence_number, "Request timed out");

                    let _ = self
                        .actions_sink
                        .send(Action::RemoveSequenceNumber(sequence_number));
                })?
                .map_err(|_| Error::ConnectionClosed)?
        };

        RequestFuture::new(&self.actions_sink, sequence_number, future)
    }

    async fn request_without_response(&self, pdu: impl Into<Pdu>) -> Result<(), Error> {
        let sequence_number = self.next_sequence_number();

        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let (action, response) = SendCommandNoResponse::new(command);

        self.actions_sink
            .send(Action::SendCommandNoResponse(action))
            .map_err(|_| Error::ConnectionClosed)?;

        // No need to timeout here, since we are not waiting for a response from the server.
        response.await.map_err(|_| Error::ConnectionClosed)?
    }

    /// Sends an [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`] response.
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        let session_state = self.session_state();

        let response = match session_state {
            SessionState::BoundTx | SessionState::BoundTrx => {
                self.request(submit_sm.into()).await?
            }
            SessionState::Closed => {
                return Err(Error::ConnectionClosed);
            }
            session_state => return Err(Error::InvalidSessionState { session_state }),
        };

        response
            .ok()
            .map_err(Error::unexpected_response)
            .map(Command::into_parts)
            .map(|(id, status, sequence_number, pdu)| match pdu {
                Some(Pdu::SubmitSmResp(response)) => Ok(response),
                _ => Err(Command::from_parts(id, status, sequence_number, pdu)),
            })?
            .map_err(Error::unexpected_response)
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for an [`UnbindResp`](Pdu::UnbindResp) response and terminates the connection.
    ///
    /// - The [`UnbindResp`](Pdu::UnbindResp) status is not checked, the connection is closed regardless of the response status.
    /// - If the [`UnbindResp`](Pdu::UnbindResp) times out, the connection is closed anyway.
    pub async fn unbind(&self) -> Result<(), Error> {
        let session_state = self.session_state();

        let response = match session_state {
            SessionState::BoundTx | SessionState::BoundRx | SessionState::BoundTrx => {
                self.request(Pdu::Unbind).await?
            }
            SessionState::Closed => {
                return Err(Error::ConnectionClosed);
            }
            session_state => return Err(Error::InvalidSessionState { session_state }),
        };

        response
            .ok_and_matches(CommandId::UnbindResp)
            .map(|_| ())
            .map_err(Error::unexpected_response)
    }

    /// Sends a [`GenericNack`](Pdu::GenericNack) command to the server.
    pub async fn generic_nack(&self) -> Result<(), Error> {
        let session_state = self.session_state();

        match session_state {
            SessionState::Closed => Err(Error::ConnectionClosed),
            _ => self.request_without_response(Pdu::GenericNack).await,
        }
    }

    // TODO: cancel safety. If this function was called and immediately dropped. The next call to it will resolve immediately.
    // Which is obviously wrong.
    // Its not even clone safe, since the a cloned client call to this function will resolve immediately.
    // Use a cancellation token and cancel it in the connection when every task is terminated.

    /// Wait for the connection to be terminated.
    pub async fn terminated(&self) {
        let termination_rx = self.termination_rx.lock().take();

        if let Some(mut termination_rx) = termination_rx {
            // wait for the termination signal
            let _ = termination_rx.recv().await;
        }
    }
}

use pin_project_lite::pin_project;

pin_project! {
    /// The [`RequestFuture`] is used to wrap a pending request future and remove it's corresponding sequence number
    /// from the pending responses if the future got dropped.
    pub struct RequestFuture<'a, F> {
        done: bool,
        sequence_number: u32,
        actions_sink: &'a UnboundedSender<Action>,
        #[pin]
        fut: F,
    }

    impl<F> PinnedDrop for RequestFuture<'_, F> {
        fn drop(this: Pin<&mut Self>) {
            let this = this.project();

            if !*this.done {
                let sequence_number = *this.sequence_number;

                tracing::debug!(target: TARGET, sequence_number, "Request was cancelled");

                let _ = this.actions_sink.send(Action::RemoveSequenceNumber(*this.sequence_number));
            }
        }
    }
}

impl<'a, F> RequestFuture<'a, F> {
    pub fn new(actions_sink: &'a UnboundedSender<Action>, sequence_number: u32, fut: F) -> Self {
        Self {
            done: false,
            sequence_number,
            actions_sink,
            fut,
        }
    }
}

impl<'a, F: Future> Future for RequestFuture<'a, F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx) {
            Poll::Ready(result) => {
                // Mark as done to prevent removing the sequence number on drop
                *this.done = true;

                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
