use std::{ops::Deref, sync::Arc, time::Duration};

use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, SubmitSm},
    session::SessionState,
};
use tokio::sync::mpsc::Sender;

use crate::{
    action::{Action, SendCommandAction},
    error::Error,
    session_state::SessionStateHolder,
};

#[derive(Debug)]
pub struct Client {
    inner: Arc<ClientInner>,
}

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
    pub(crate) fn new(
        actions_sink: Sender<Action>,
        session_timeout: Duration,
        response_timeout: Duration,
        session_state_holder: SessionStateHolder,
    ) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(
                actions_sink,
                session_timeout,
                response_timeout,
                session_state_holder,
            )),
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
pub struct ClientInner {
    actions_sink: Sender<Action>,
    session_timeout: Duration,
    response_timeout: Duration,
    session_state_holder: SessionStateHolder,
}

impl ClientInner {
    const fn new(
        actions_sink: Sender<Action>,
        session_timeout: Duration,
        response_timeout: Duration,
        session_state_holder: SessionStateHolder,
    ) -> Self {
        Self {
            actions_sink,
            session_timeout,
            response_timeout,
            session_state_holder,
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

    // TODO: bind is different than request just in the timeout
    async fn bind(&self, pdu: impl Into<Pdu>) -> Result<Command, Error> {
        let sequence_number = self.next_sequence_number();
        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let (action, response) = SendCommandAction::new(command);

        self.actions_sink
            .send(Action::SendCommand(action))
            .await
            .map_err(|_| Error::ConnectionClosed)?;

        // TODO: if session timed out, we should close the connection
        tokio::time::timeout(self.session_timeout, response)
            .await
            .map_err(|_| Error::Timeout)?
            .map_err(|_| Error::ConnectionClosed)?
    }

    async fn request(&self, pdu: impl Into<Pdu>) -> Result<Command, Error> {
        let sequence_number = self.next_sequence_number();
        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let (action, response) = SendCommandAction::new(command);

        self.actions_sink
            .send(Action::SendCommand(action))
            .await
            .map_err(|_| Error::ConnectionClosed)?;

        tokio::time::timeout(self.response_timeout, response)
            .await
            .map_err(|_| Error::Timeout)?
            .map_err(|_| Error::ConnectionClosed)?
    }

    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<Command, Error> {
        let session_state = self.session_state();

        let response = match session_state {
            SessionState::BoundTx | SessionState::BoundTrx => {
                self.request(submit_sm.into()).await?
            }
            session_state => return Err(Error::InvalidSessionState { session_state }),
        };

        response
            .ok_and_matches(CommandId::SubmitMultiResp)
            .map_err(Error::unexpected_response)
    }
}
