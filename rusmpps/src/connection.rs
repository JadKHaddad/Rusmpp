use std::{str::FromStr, sync::Arc};

use futures::{SinkExt, StreamExt, TryStreamExt, future};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    codec::CommandCodec,
    pdus::{BindReceiverResp, BindTransceiverResp},
    types::COctetString,
    values::InterfaceVersion,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::mpsc,
};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{bind_mode::BindMode, client::ConnectedClient, config::Config};

#[derive(Debug)]
pub struct Connection {
    config: Arc<Config>,
}

impl Connection {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn run<S>(self, stream: S)
    where
        S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    {
        let (reader, writer) = tokio::io::split(stream);

        let mut reader = FramedRead::new(reader, CommandCodec::new().with_max_length(1024));
        let mut writer = FramedWrite::new(writer, CommandCodec::new().with_max_length(1024));

        tracing::debug!("Awaiting bind operation");

        let mut filtered = (&mut reader).try_filter(|command| {
            future::ready(matches!(
                command.id(),
                CommandId::BindTransmitter | CommandId::BindReceiver | CommandId::BindTransceiver
            ))
        });

        let (sequence_number, system_id, password, bind_mode) = tokio::select! {
            _ = tokio::time::sleep(self.config.session_timeout) => {
                tracing::warn!("Session timeout reached, closing connection");

                return;
            },
            command = filtered.next() => {
                match command {
                    None => {
                        tracing::warn!("Connection closed before bind command was received");

                        return;
                    },
                    Some(Err(err)) => {
                        tracing::error!(?err, "Failed read command");

                        return;
                    },
                    Some(Ok(command)) => {
                        tracing::debug!(?command, "Received bind command");

                        let (_, status, sequence_number, pdu) = command.into_parts();

                        if !(matches!(status, CommandStatus::EsmeRok)) {
                            // Really?

                            tracing::error!(?status, "Received bind command with non-OK status");

                            return;
                        }

                        let (system_id, password, bind_mode) = match pdu {
                            None => {
                                tracing::error!("Received bind command without PDU");

                                return;
                            }
                            Some(Pdu::BindTransmitter(bind)) => {
                                (bind.system_id, bind.password, BindMode::Tx)
                            }
                            Some(Pdu::BindReceiver(bind)) => {
                                (bind.system_id, bind.password, BindMode::Rx)
                            }
                            Some(Pdu::BindTransceiver(bind)) => {
                                (bind.system_id, bind.password, BindMode::Trx)
                            },
                            _ => {
                                // Should not happen

                                return;
                            }
                        };

                        (sequence_number, system_id, password, bind_mode)
                    }
                }
            }
        };

        // TODO: Do the verification of system_id and password here

        let mc_system_id = COctetString::from_str("Rusmpps").expect("Must be valid system ID");
        let sc_interface_version = Some(InterfaceVersion::Smpp5_0);

        let pdu: Pdu = match bind_mode {
            BindMode::Tx => BindTransceiverResp::builder()
                .system_id(mc_system_id)
                .sc_interface_version(sc_interface_version)
                .build()
                .into(),
            BindMode::Rx => BindReceiverResp::builder()
                .system_id(mc_system_id)
                .sc_interface_version(sc_interface_version)
                .build()
                .into(),
            BindMode::Trx => BindTransceiverResp::builder()
                .system_id(mc_system_id)
                .sc_interface_version(sc_interface_version)
                .build()
                .into(),
        };

        let command = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(pdu);

        tokio::time::sleep(self.config.bind_delay).await;

        tracing::debug!(?command, "Sending bind response");

        if let Err(err) = writer.send(command).await {
            tracing::error!(?err, "Failed to send bind response");

            return;
        }

        let (tx, rx) = mpsc::channel(100);

        let system_id = system_id.to_string();
        let connected_client = ConnectedClient::new(tx, bind_mode.into());

        self.config
            .connected_clients
            .write()
            .await
            .insert(system_id, connected_client);

        while let Some(command) = reader.next().await {}
    }
}
