use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, SubmitSm, SubmitSmResp},
    session::SessionState,
};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::{
    CommandExt, ConnectionBuilder, PendingResponses,
    action::{Action, SendCommand, SendCommandNoResponse},
    error::Error,
    session_state::SessionStateHolder,
};

/// `SMPP` Client.
///
/// The client is a handle to communicate with the `SMPP` server through a managed connection in the background.
///
/// When all clients are dropped, an [`Unbind`](Pdu::Unbind) command is sent to the server, and the connection is closed.
#[derive(Debug)]
pub struct Client {
    inner: Arc<ClientInner>,
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
        actions_sink: Sender<Action>,
        response_timeout: Duration,
        session_state_holder: SessionStateHolder,
        pending_responses: PendingResponses,
        termination_token: CancellationToken,
    ) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(
                actions_sink,
                response_timeout,
                session_state_holder,
                pending_responses,
                termination_token,
            )),
        }
    }

    /// Creates a new `SMPP` connection builder.
    pub fn builder() -> ConnectionBuilder {
        ConnectionBuilder::new()
    }

    /// Returns the current session state of the client.
    pub fn session_state(&self) -> SessionState {
        self.inner.session_state()
    }

    /// Returns the current sequence number of the client.
    pub fn sequence_number(&self) -> u32 {
        self.inner.sequence_number()
    }

    pub(crate) async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<Command, Error> {
        self.inner.bind_transmitter(bind).await
    }

    pub(crate) async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<Command, Error> {
        self.inner.bind_receiver(bind).await
    }

    pub(crate) async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<Command, Error> {
        self.inner.bind_transceiver(bind).await
    }

    /// Sends an [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`].
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        self.inner.submit_sm(submit_sm).await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for an [`UnbindResp`](Pdu::UnbindResp) and terminates the connection.
    ///
    /// - The [`UnbindResp`](Pdu::UnbindResp) status is not checked, the connection is closed regardless of the response status.
    /// - If the [`UnbindResp`](Pdu::UnbindResp) times out, the connection is closed anyway.
    pub async fn unbind(&self) -> Result<(), Error> {
        self.inner.unbind().await
    }

    /// Sends a [`GenericNack`](Pdu::GenericNack) command to the server.
    pub async fn generic_nack(&self, sequence_number: u32) -> Result<(), Error> {
        self.inner.generic_nack(sequence_number).await
    }

    /// Wait for the connection to be terminated.
    pub async fn terminated(&self) {
        self.inner.terminated().await;
    }
}

#[derive(Debug)]
struct ClientInner {
    actions_sink: Sender<Action>,
    response_timeout: Duration,
    session_state_holder: SessionStateHolder,
    pending_responses: PendingResponses,
    /// Await the termination token to ensure that the connection tasks were terminated
    termination_token: CancellationToken,
}

impl ClientInner {
    const fn new(
        actions_sink: Sender<Action>,
        response_timeout: Duration,
        session_state_holder: SessionStateHolder,
        pending_responses: PendingResponses,
        termination_token: CancellationToken,
    ) -> Self {
        Self {
            actions_sink,
            response_timeout,
            session_state_holder,
            pending_responses,
            termination_token,
        }
    }
}

impl ClientInner {
    fn sequence_number(&self) -> u32 {
        self.session_state_holder.sequence_number()
    }

    fn next_sequence_number(&self) -> u32 {
        self.session_state_holder.next_sequence_number()
    }

    fn session_state(&self) -> SessionState {
        self.session_state_holder.session_state()
    }

    fn set_session_state(&self, session_state: SessionState) {
        self.session_state_holder.set_session_state(session_state)
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
                .await
                .map_err(|_| Error::ConnectionClosed)?;

            tokio::time::timeout(self.response_timeout, response)
                .await
                .map_err(|_| Error::Timeout)
                .inspect_err(|_| {
                    self.pending_responses.lock().remove(&sequence_number);
                })?
                .map_err(|_| Error::ConnectionClosed)?
        };

        RequestFuture::new(&self.pending_responses, sequence_number, future)
    }

    async fn request_without_response(
        &self,
        pdu: impl Into<Pdu>,
        sequence_number: Option<u32>,
    ) -> Result<(), Error> {
        let sequence_number = sequence_number.unwrap_or(self.next_sequence_number());

        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let (action, response) = SendCommandNoResponse::new(command);

        self.actions_sink
            .send(Action::SendCommandNoResponse(action))
            .await
            .map_err(|_| Error::ConnectionClosed)?;

        // No need to timeout here, since we are not waiting for a response from the server.
        response.await.map_err(|_| Error::ConnectionClosed)?
    }

    async fn bind_transmitter(&self, bind: impl Into<BindTransmitter>) -> Result<Command, Error> {
        let response = self.request(bind.into()).await?;

        let response = response
            .ok_and_matches(CommandId::BindTransmitterResp)
            .map_err(Error::unexpected_response)?;

        self.set_session_state(SessionState::BoundTx);

        Ok(response)
    }

    async fn bind_receiver(&self, bind: impl Into<BindReceiver>) -> Result<Command, Error> {
        let response = self.request(bind.into()).await?;

        let response = response
            .ok_and_matches(CommandId::BindReceiverResp)
            .map_err(Error::unexpected_response)?;

        self.set_session_state(SessionState::BoundRx);

        Ok(response)
    }

    async fn bind_transceiver(&self, bind: impl Into<BindTransceiver>) -> Result<Command, Error> {
        let response = self.request(bind.into()).await?;

        let response = response
            .ok_and_matches(CommandId::BindTransceiverResp)
            .map_err(Error::unexpected_response)?;

        self.set_session_state(SessionState::BoundTrx);

        Ok(response)
    }

    async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
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

    async fn unbind(&self) -> Result<(), Error> {
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

    async fn generic_nack(&self, sequence_number: u32) -> Result<(), Error> {
        let session_state = self.session_state();

        match session_state {
            SessionState::Closed => Err(Error::ConnectionClosed),
            _ => {
                self.request_without_response(Pdu::GenericNack, Some(sequence_number))
                    .await
            }
        }
    }

    async fn terminated(&self) {
        self.termination_token.cancelled().await;
    }
}

use pin_project_lite::pin_project;

pin_project! {
    /// The [`RequestFuture`] is used to wrap a pending request future and remove it's corresponding sequence number
    /// from the pending responses if the future got dropped.
    struct RequestFuture<'a, F> {
        done: bool,
        sequence_number: u32,
        pending_responses: &'a PendingResponses,
        #[pin]
        fut: F,
    }

    impl<F> PinnedDrop for RequestFuture<'_, F> {
        fn drop(this: Pin<&mut Self>) {
            let this = this.project();

            if !*this.done {
                (*this.pending_responses).lock().remove(&*this.sequence_number);
            }
        }
    }
}

impl<'a, F> RequestFuture<'a, F> {
    pub fn new(pending_responses: &'a PendingResponses, sequence_number: u32, fut: F) -> Self {
        Self {
            done: false,
            sequence_number,
            pending_responses,
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
