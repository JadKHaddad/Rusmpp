use std::time::Duration;

use futures::{SinkExt, StreamExt};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    codec::CommandCodec,
    pdus::{BindReceiverResp, BindTransceiverResp, BindTransmitterResp, SubmitSmResp},
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

#[derive(Debug)]
pub struct Server {
    bind_delay: Duration,
    enquire_link_delay: Duration,
    response_delay: Duration,
    close_connection_delay: Duration,
}

impl Server {
    pub fn new() -> Self {
        Self {
            bind_delay: Duration::from_millis(500),
            enquire_link_delay: Duration::from_millis(500),
            response_delay: Duration::from_millis(500),
            close_connection_delay: Duration::from_secs(10),
        }
    }

    pub fn bind_delay(mut self, delay: Duration) -> Self {
        self.bind_delay = delay;
        self
    }

    pub fn enquire_link_delay(mut self, delay: Duration) -> Self {
        self.enquire_link_delay = delay;
        self
    }

    pub fn response_delay(mut self, delay: Duration) -> Self {
        self.response_delay = delay;
        self
    }

    pub fn close_connection_delay(mut self, delay: Duration) -> Self {
        self.close_connection_delay = delay;
        self
    }

    pub async fn run<S: AsyncRead + AsyncWrite + Send + Unpin + 'static>(self, stream: S) {
        let mut framed = Framed::new(stream, CommandCodec::new());

        let future = async move {
            while let Some(Ok(command)) = framed.next().await {
                let pdu: Pdu = match command.id() {
                    CommandId::EnquireLink => {
                        tokio::time::sleep(self.enquire_link_delay).await;

                        Pdu::EnquireLinkResp
                    }
                    CommandId::BindTransmitter => {
                        tokio::time::sleep(self.bind_delay).await;

                        BindTransmitterResp::default().into()
                    }
                    CommandId::BindReceiver => {
                        tokio::time::sleep(self.bind_delay).await;

                        BindReceiverResp::default().into()
                    }
                    CommandId::BindTransceiver => {
                        tokio::time::sleep(self.bind_delay).await;

                        BindTransceiverResp::default().into()
                    }
                    CommandId::SubmitSm => {
                        tokio::time::sleep(self.response_delay).await;

                        SubmitSmResp::default().into()
                    }
                    CommandId::Unbind => {
                        tokio::time::sleep(self.response_delay).await;

                        Pdu::UnbindResp
                    }
                    CommandId::GenericNack => {
                        tracing::warn!("Received GenericNack. Crashing");

                        break;
                    }
                    _ => {
                        continue;
                    }
                };

                let response = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(pdu);

                framed
                    .send(response)
                    .await
                    .expect("Failed to send response");
            }
        };

        let _ = tokio::time::timeout(self.close_connection_delay, future).await;
    }
}

/// A server that only issues an unbind after a delay.
#[derive(Debug)]
pub struct UnbindServer {
    delay: Duration,
}

impl UnbindServer {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }

    pub async fn run<S: AsyncRead + AsyncWrite + Send + Unpin + 'static>(self, stream: S) {
        let mut framed = Framed::new(stream, CommandCodec::new());

        let future = async {
            while let Some(Ok(command)) = framed.next().await {
                let pdu: Pdu = match command.id() {
                    CommandId::EnquireLink => Pdu::EnquireLinkResp,
                    CommandId::BindTransmitter => BindTransmitterResp::default().into(),
                    CommandId::BindReceiver => BindReceiverResp::default().into(),
                    CommandId::BindTransceiver => BindTransceiverResp::default().into(),
                    _ => {
                        continue;
                    }
                };

                let response = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(pdu);

                framed
                    .send(response)
                    .await
                    .expect("Failed to send response");
            }
        };

        tokio::select! {
            _ = future => {

            },
            _ = tokio::time::sleep(self.delay) => {
                let unbind = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(1)
                    .pdu(Pdu::Unbind);

                framed
                    .send(unbind)
                    .await
                    .expect("Failed to send unbind response");
            }
        }
    }
}
