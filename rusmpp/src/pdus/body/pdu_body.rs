use crate::{
    io::{
        length::IoLength,
        read::{
            AsyncIoRead, AsyncIoReadWithKey, AsyncIoReadWithLength, AsyncIoReadable, IoReadError,
        },
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::command_id::CommandId,
};

use super::bodies::{bind::Bind, bind_resp::BindResp};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PduBody {
    BindTransmitter(Bind),
    BindTransmitterResp(BindResp),
    BindReceiver(Bind),
    BindReceiverResp(BindResp),
    BindTransceiver(Bind),
    BindTransceiverResp(BindResp),
}

impl PduBody {
    pub fn command_id(&self) -> CommandId {
        match self {
            PduBody::BindTransmitter(_) => CommandId::BindTransmitter,
            PduBody::BindTransmitterResp(_) => CommandId::BindTransmitterResp,
            PduBody::BindReceiver(_) => CommandId::BindReceiver,
            PduBody::BindReceiverResp(_) => CommandId::BindReceiverResp,
            PduBody::BindTransceiver(_) => CommandId::BindTransceiver,
            PduBody::BindTransceiverResp(_) => CommandId::BindTransceiverResp,
        }
    }
}

impl IoLength for PduBody {
    fn length(&self) -> usize {
        match self {
            PduBody::BindTransmitter(b) => b.length(),
            PduBody::BindTransmitterResp(b) => b.length(),
            PduBody::BindReceiver(b) => b.length(),
            PduBody::BindReceiverResp(b) => b.length(),
            PduBody::BindTransceiver(b) => b.length(),
            PduBody::BindTransceiverResp(b) => b.length(),
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for PduBody {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<usize> {
        match self {
            PduBody::BindTransmitter(b) => b.async_io_write(buf).await,
            PduBody::BindTransmitterResp(b) => b.async_io_write(buf).await,
            PduBody::BindReceiver(b) => b.async_io_write(buf).await,
            PduBody::BindReceiverResp(b) => b.async_io_write(buf).await,
            PduBody::BindTransceiver(b) => b.async_io_write(buf).await,
            PduBody::BindTransceiverResp(b) => b.async_io_write(buf).await,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithKey for PduBody {
    type Key = CommandId;

    async fn async_io_read(
        key: Self::Key,
        buf: &mut AsyncIoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError> {
        if !key.has_body() {
            return Ok(None);
        }

        let read = match key {
            CommandId::BindTransmitter => PduBody::BindTransmitter(Bind::async_io_read(buf).await?),
            CommandId::BindTransmitterResp => {
                PduBody::BindTransmitterResp(BindResp::async_io_read(buf, length).await?)
            }
            CommandId::BindReceiver => PduBody::BindReceiver(Bind::async_io_read(buf).await?),
            CommandId::BindReceiverResp => {
                PduBody::BindReceiverResp(BindResp::async_io_read(buf, length).await?)
            }
            CommandId::BindTransceiver => PduBody::BindTransceiver(Bind::async_io_read(buf).await?),
            CommandId::BindTransceiverResp => {
                PduBody::BindTransceiverResp(BindResp::async_io_read(buf, length).await?)
            }
            _ => {
                return Err(IoReadError::UnknownKey {
                    key: u32::from(key),
                })
            }
        };

        Ok(Some(read))
    }
}
