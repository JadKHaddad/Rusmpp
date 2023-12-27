use crate::{
    io::{
        length::IoLength,
        read::{
            AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadWithLength, AsyncIoReadable,
            IoReadError,
        },
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::command_id::CommandId,
    types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

use super::bodies::{
    alert_notification::AlertNotification, bind::Bind, bind_resp::BindResp,
    broadcast_sm::BroadcastSm, broadcast_sm_resp::BroadcastSmResp,
    cancel_broadcast_sm::CancelBroadcastSm, cancel_sm::CancelSm, data_sm::DataSm,
    deliver_sm::DeliverSm, deliver_sm_resp::DeliverSmResp, outbind::Outbind,
    query_broadcast_sm::QueryBroadcastSm, query_broadcast_sm_resp::QueryBroadcastSmResp,
    query_sm::QuerySm, query_sm_resp::QuerySmResp, replace_sm::ReplaceSm,
    submit_multi::SubmitMulti, submit_or_data_sm_resp::SubmitOrDataSmResp, submit_sm::SubmitSm,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PduBody {
    BindTransmitter(Bind),
    BindTransmitterResp(BindResp),
    BindReceiver(Bind),
    BindReceiverResp(BindResp),
    BindTransceiver(Bind),
    BindTransceiverResp(BindResp),
    Outbind(Outbind),
    AlertNotification(AlertNotification),
    SubmitSm(SubmitSm),
    SubmitSmResp(SubmitOrDataSmResp),
    QuerySm(QuerySm),
    QuerySmResp(QuerySmResp),
    DeliverSm(DeliverSm),
    DeliverSmResp(DeliverSmResp),
    DataSm(DataSm),
    DataSmResp(SubmitOrDataSmResp),
    CancelSm(CancelSm),
    ReplaceSm(ReplaceSm),
    SubmitMulti(SubmitMulti),
    SubmitMultiResp(SubmitOrDataSmResp),
    BroadcastSm(BroadcastSm),
    BroadcastSmResp(BroadcastSmResp),
    QueryBroadcastSm(QueryBroadcastSm),
    QueryBroadcastSmResp(QueryBroadcastSmResp),
    CancelBroadcastSm(CancelBroadcastSm),
    Other {
        command_id: CommandId,
        body: NoFixedSizeOctetString,
    },
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
            PduBody::Outbind(_) => CommandId::Outbind,
            PduBody::AlertNotification(_) => CommandId::AlertNotification,
            PduBody::SubmitSm(_) => CommandId::SubmitSm,
            PduBody::SubmitSmResp(_) => CommandId::SubmitSmResp,
            PduBody::QuerySm(_) => CommandId::QuerySm,
            PduBody::QuerySmResp(_) => CommandId::QuerySmResp,
            PduBody::DeliverSm(_) => CommandId::DeliverSm,
            PduBody::DeliverSmResp(_) => CommandId::DeliverSmResp,
            PduBody::DataSm(_) => CommandId::DataSm,
            PduBody::DataSmResp(_) => CommandId::DataSmResp,
            PduBody::CancelSm(_) => CommandId::CancelSm,
            PduBody::ReplaceSm(_) => CommandId::ReplaceSm,
            PduBody::SubmitMulti(_) => CommandId::SubmitMulti,
            PduBody::SubmitMultiResp(_) => CommandId::SubmitMultiResp,
            PduBody::BroadcastSm(_) => CommandId::BroadcastSm,
            PduBody::BroadcastSmResp(_) => CommandId::BroadcastSmResp,
            PduBody::QueryBroadcastSm(_) => CommandId::QueryBroadcastSm,
            PduBody::QueryBroadcastSmResp(_) => CommandId::QueryBroadcastSmResp,
            PduBody::CancelBroadcastSm(_) => CommandId::CancelBroadcastSm,
            PduBody::Other { command_id, .. } => *command_id,
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
            PduBody::Outbind(b) => b.length(),
            PduBody::AlertNotification(b) => b.length(),
            PduBody::SubmitSm(b) => b.length(),
            PduBody::SubmitSmResp(b) => b.length(),
            PduBody::QuerySm(b) => b.length(),
            PduBody::QuerySmResp(b) => b.length(),
            PduBody::DeliverSm(b) => b.length(),
            PduBody::DeliverSmResp(b) => b.length(),
            PduBody::DataSm(b) => b.length(),
            PduBody::DataSmResp(b) => b.length(),
            PduBody::CancelSm(b) => b.length(),
            PduBody::ReplaceSm(b) => b.length(),
            PduBody::SubmitMulti(b) => b.length(),
            PduBody::SubmitMultiResp(b) => b.length(),
            PduBody::BroadcastSm(b) => b.length(),
            PduBody::BroadcastSmResp(b) => b.length(),
            PduBody::QueryBroadcastSm(b) => b.length(),
            PduBody::QueryBroadcastSmResp(b) => b.length(),
            PduBody::CancelBroadcastSm(b) => b.length(),
            PduBody::Other { body, .. } => body.length(),
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for PduBody {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        match self {
            PduBody::BindTransmitter(b) => b.async_io_write(buf).await,
            PduBody::BindTransmitterResp(b) => b.async_io_write(buf).await,
            PduBody::BindReceiver(b) => b.async_io_write(buf).await,
            PduBody::BindReceiverResp(b) => b.async_io_write(buf).await,
            PduBody::BindTransceiver(b) => b.async_io_write(buf).await,
            PduBody::BindTransceiverResp(b) => b.async_io_write(buf).await,
            PduBody::Outbind(b) => b.async_io_write(buf).await,
            PduBody::AlertNotification(b) => b.async_io_write(buf).await,
            PduBody::SubmitSm(b) => b.async_io_write(buf).await,
            PduBody::SubmitSmResp(b) => b.async_io_write(buf).await,
            PduBody::QuerySm(b) => b.async_io_write(buf).await,
            PduBody::QuerySmResp(b) => b.async_io_write(buf).await,
            PduBody::DeliverSm(b) => b.async_io_write(buf).await,
            PduBody::DeliverSmResp(b) => b.async_io_write(buf).await,
            PduBody::DataSm(b) => b.async_io_write(buf).await,
            PduBody::DataSmResp(b) => b.async_io_write(buf).await,
            PduBody::CancelSm(b) => b.async_io_write(buf).await,
            PduBody::ReplaceSm(b) => b.async_io_write(buf).await,
            PduBody::SubmitMulti(b) => b.async_io_write(buf).await,
            PduBody::SubmitMultiResp(b) => b.async_io_write(buf).await,
            PduBody::BroadcastSm(b) => b.async_io_write(buf).await,
            PduBody::BroadcastSmResp(b) => b.async_io_write(buf).await,
            PduBody::QueryBroadcastSm(b) => b.async_io_write(buf).await,
            PduBody::QueryBroadcastSmResp(b) => b.async_io_write(buf).await,
            PduBody::CancelBroadcastSm(b) => b.async_io_write(buf).await,
            PduBody::Other { body, .. } => body.async_io_write(buf).await,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithKeyOptional for PduBody {
    type Key = CommandId;

    async fn async_io_read(
        key: Self::Key,
        buf: &mut AsyncIoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError> {
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
            CommandId::Outbind => PduBody::Outbind(Outbind::async_io_read(buf).await?),
            CommandId::AlertNotification => {
                PduBody::AlertNotification(AlertNotification::async_io_read(buf, length).await?)
            }
            CommandId::SubmitSm => PduBody::SubmitSm(SubmitSm::async_io_read(buf, length).await?),
            CommandId::SubmitSmResp => {
                PduBody::SubmitSmResp(SubmitOrDataSmResp::async_io_read(buf, length).await?)
            }
            CommandId::QuerySm => PduBody::QuerySm(QuerySm::async_io_read(buf).await?),
            CommandId::QuerySmResp => PduBody::QuerySmResp(QuerySmResp::async_io_read(buf).await?),
            CommandId::DeliverSm => {
                PduBody::DeliverSm(DeliverSm::async_io_read(buf, length).await?)
            }
            CommandId::DeliverSmResp => {
                PduBody::DeliverSmResp(DeliverSmResp::async_io_read(buf, length).await?)
            }
            CommandId::DataSm => PduBody::DataSm(DataSm::async_io_read(buf, length).await?),
            CommandId::DataSmResp => {
                PduBody::DataSmResp(SubmitOrDataSmResp::async_io_read(buf, length).await?)
            }
            CommandId::CancelSm => PduBody::CancelSm(CancelSm::async_io_read(buf).await?),
            CommandId::ReplaceSm => {
                PduBody::ReplaceSm(ReplaceSm::async_io_read(buf, length).await?)
            }
            CommandId::SubmitMulti => {
                PduBody::SubmitMulti(SubmitMulti::async_io_read(buf, length).await?)
            }
            CommandId::SubmitMultiResp => {
                PduBody::SubmitMultiResp(SubmitOrDataSmResp::async_io_read(buf, length).await?)
            }
            CommandId::BroadcastSm => {
                PduBody::BroadcastSm(BroadcastSm::async_io_read(buf, length).await?)
            }
            CommandId::BroadcastSmResp => {
                PduBody::BroadcastSmResp(BroadcastSmResp::async_io_read(buf, length).await?)
            }
            CommandId::QueryBroadcastSm => {
                PduBody::QueryBroadcastSm(QueryBroadcastSm::async_io_read(buf, length).await?)
            }
            CommandId::QueryBroadcastSmResp => PduBody::QueryBroadcastSmResp(
                QueryBroadcastSmResp::async_io_read(buf, length).await?,
            ),
            CommandId::CancelBroadcastSm => {
                PduBody::CancelBroadcastSm(CancelBroadcastSm::async_io_read(buf, length).await?)
            }
            CommandId::Other(_) => PduBody::Other {
                command_id: key,
                body: NoFixedSizeOctetString::async_io_read(buf, length).await?,
            },
            CommandId::Unbind
            | CommandId::UnbindResp
            | CommandId::EnquireLink
            | CommandId::EnquireLinkResp
            | CommandId::GenericNack
            | CommandId::CancelSmResp
            | CommandId::ReplaceSmResp
            | CommandId::CancelBroadcastSmResp => return Ok(None),
        };

        Ok(Some(read))
    }
}
