use super::{
    super::bodies::{
        alert_notification::AlertNotification, bind::Bind, bind_resp::BindResp,
        broadcast_sm::BroadcastSm, broadcast_sm_resp::BroadcastSmResp,
        cancel_broadcast_sm::CancelBroadcastSm, cancel_sm::CancelSm, data_sm::DataSm,
        deliver_sm_resp::DeliverSmResp, outbind::Outbind, query_broadcast_sm::QueryBroadcastSm,
        query_broadcast_sm_resp::QueryBroadcastSmResp, query_sm::QuerySm,
        query_sm_resp::QuerySmResp, replace_sm::ReplaceSm, submit_multi::SubmitMulti,
        submit_or_data_sm_resp::SubmitOrDataSmResp, submit_or_deliver_sm::DeliverSm,
        submit_or_deliver_sm::SubmitSm,
    },
    PduBody,
};
use crate::pdus::types::command_id::CommandId;
use rusmpp_io::{
    io::read::{
        AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadWithLength, AsyncIoReadable, IoRead,
        IoReadError, IoReadWithKeyOptional, IoReadWithLength, IoReadable,
    },
    types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

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

impl IoReadWithKeyOptional for PduBody {
    type Key = CommandId;

    fn io_read(
        key: Self::Key,
        buf: &mut IoReadable,
        length: usize,
    ) -> Result<Option<Self>, IoReadError> {
        let read = match key {
            CommandId::BindTransmitter => PduBody::BindTransmitter(Bind::io_read(buf)?),
            CommandId::BindTransmitterResp => {
                PduBody::BindTransmitterResp(BindResp::io_read(buf, length)?)
            }
            CommandId::BindReceiver => PduBody::BindReceiver(Bind::io_read(buf)?),
            CommandId::BindReceiverResp => {
                PduBody::BindReceiverResp(BindResp::io_read(buf, length)?)
            }
            CommandId::BindTransceiver => PduBody::BindTransceiver(Bind::io_read(buf)?),
            CommandId::BindTransceiverResp => {
                PduBody::BindTransceiverResp(BindResp::io_read(buf, length)?)
            }
            CommandId::Outbind => PduBody::Outbind(Outbind::io_read(buf)?),
            CommandId::AlertNotification => {
                PduBody::AlertNotification(AlertNotification::io_read(buf, length)?)
            }
            CommandId::SubmitSm => PduBody::SubmitSm(SubmitSm::io_read(buf, length)?),
            CommandId::SubmitSmResp => {
                PduBody::SubmitSmResp(SubmitOrDataSmResp::io_read(buf, length)?)
            }
            CommandId::QuerySm => PduBody::QuerySm(QuerySm::io_read(buf)?),
            CommandId::QuerySmResp => PduBody::QuerySmResp(QuerySmResp::io_read(buf)?),
            CommandId::DeliverSm => PduBody::DeliverSm(DeliverSm::io_read(buf, length)?),
            CommandId::DeliverSmResp => {
                PduBody::DeliverSmResp(DeliverSmResp::io_read(buf, length)?)
            }
            CommandId::DataSm => PduBody::DataSm(DataSm::io_read(buf, length)?),
            CommandId::DataSmResp => PduBody::DataSmResp(SubmitOrDataSmResp::io_read(buf, length)?),
            CommandId::CancelSm => PduBody::CancelSm(CancelSm::io_read(buf)?),
            CommandId::ReplaceSm => PduBody::ReplaceSm(ReplaceSm::io_read(buf, length)?),
            CommandId::SubmitMulti => PduBody::SubmitMulti(SubmitMulti::io_read(buf, length)?),
            CommandId::SubmitMultiResp => {
                PduBody::SubmitMultiResp(SubmitOrDataSmResp::io_read(buf, length)?)
            }
            CommandId::BroadcastSm => PduBody::BroadcastSm(BroadcastSm::io_read(buf, length)?),
            CommandId::BroadcastSmResp => {
                PduBody::BroadcastSmResp(BroadcastSmResp::io_read(buf, length)?)
            }
            CommandId::QueryBroadcastSm => {
                PduBody::QueryBroadcastSm(QueryBroadcastSm::io_read(buf, length)?)
            }
            CommandId::QueryBroadcastSmResp => {
                PduBody::QueryBroadcastSmResp(QueryBroadcastSmResp::io_read(buf, length)?)
            }
            CommandId::CancelBroadcastSm => {
                PduBody::CancelBroadcastSm(CancelBroadcastSm::io_read(buf, length)?)
            }
            CommandId::Other(_) => PduBody::Other {
                command_id: key,
                body: NoFixedSizeOctetString::io_read(buf, length)?,
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
