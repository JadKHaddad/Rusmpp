use std::time::Duration;

use futures::{SinkExt, StreamExt};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    codec::CommandCodec,
    pdus::{BindReceiverResp, BindTransceiverResp, BindTransmitterResp, SubmitSmResp},
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

/// A server that accepts bind requests and submit_sm requests. And responds with delay
pub async fn run_delay_server<S: AsyncRead + AsyncWrite + Send + Unpin + 'static>(
    stream: S,
    delay: Duration,
) {
    let mut framed = Framed::new(stream, CommandCodec::new());

    while let Some(Ok(command)) = framed.next().await {
        let pdu: Pdu = match command.id() {
            CommandId::BindTransmitter => BindTransmitterResp::default().into(),
            CommandId::BindReceiver => BindReceiverResp::default().into(),
            CommandId::BindTransceiver => BindTransceiverResp::default().into(),
            CommandId::SubmitSm => SubmitSmResp::default().into(),
            _ => {
                break;
            }
        };

        tokio::time::sleep(delay).await;

        let response = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(command.sequence_number())
            .pdu(pdu);

        framed
            .send(response)
            .await
            .expect("Failed to send response");
    }
}
