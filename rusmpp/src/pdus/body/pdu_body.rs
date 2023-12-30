use super::bodies::{
    alert_notification::AlertNotification, bind::Bind, bind_resp::BindResp,
    broadcast_sm::BroadcastSm, broadcast_sm_resp::BroadcastSmResp,
    cancel_broadcast_sm::CancelBroadcastSm, cancel_sm::CancelSm, data_sm::DataSm,
    deliver_sm::DeliverSm, deliver_sm_resp::DeliverSmResp, outbind::Outbind,
    query_broadcast_sm::QueryBroadcastSm, query_broadcast_sm_resp::QueryBroadcastSmResp,
    query_sm::QuerySm, query_sm_resp::QuerySmResp, replace_sm::ReplaceSm,
    submit_multi::SubmitMulti, submit_or_data_sm_resp::SubmitOrDataSmResp, submit_sm::SubmitSm,
};
use crate::{
    pdus::types::command_id::CommandId, types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

mod length;
mod read;
mod write;

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
