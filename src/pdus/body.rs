use super::types::command_id::CommandId;
use crate::{
    io::{
        decode::{Decode, DecodeError, DecodeWithKey},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

pub mod bind;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Body {
    BindTransmitter(bind::Bind),
    // BindTransmitterResp(BindResp),
    BindReceiver(bind::Bind),
    // BindReceiverResp(BindResp),
    BindTransceiver(bind::Bind),
    // BindTransceiverResp(BindResp),
    // Outbind(Outbind),
    // AlertNotification(AlertNotification),
    // SubmitSm(SubmitSm),
    // SubmitSmResp(SubmitOrDataSmResp),
    // QuerySm(QuerySm),
    // QuerySmResp(QuerySmResp),
    // DeliverSm(DeliverSm),
    // DeliverSmResp(DeliverSmResp),
    // DataSm(DataSm),
    // DataSmResp(SubmitOrDataSmResp),
    // CancelSm(CancelSm),
    // ReplaceSm(ReplaceSm),
    // SubmitMulti(SubmitMulti),
    // SubmitMultiResp(SubmitOrDataSmResp),
    // BroadcastSm(BroadcastSm),
    // BroadcastSmResp(BroadcastSmResp),
    // QueryBroadcastSm(QueryBroadcastSm),
    // QueryBroadcastSmResp(QueryBroadcastSmResp),
    // CancelBroadcastSm(CancelBroadcastSm),
}

impl Length for Body {
    fn length(&self) -> usize {
        match self {
            Body::BindTransmitter(body) => body.length(),
            Body::BindReceiver(body) => body.length(),
            Body::BindTransceiver(body) => body.length(),
        }
    }
}

impl Encode for Body {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Body::BindTransmitter(body) => body.encode_to(writer),
            Body::BindReceiver(body) => body.encode_to(writer),
            Body::BindTransceiver(body) => body.encode_to(writer),
        }
    }
}

impl DecodeWithKey for Body {
    type Key = CommandId;

    fn decode_from<R: std::io::Read>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Option<Self>, DecodeError>
    where
        Self: Sized,
    {
        let body = match key {
            CommandId::BindTransmitter => {
                Body::BindTransmitter(tri!(bind::Bind::decode_from(reader)))
            }
            CommandId::BindReceiver => Body::BindReceiver(tri!(bind::Bind::decode_from(reader))),
            CommandId::BindTransceiver => {
                Body::BindTransceiver(tri!(bind::Bind::decode_from(reader)))
            }
            CommandId::Unbind
            | CommandId::UnbindResp
            | CommandId::EnquireLink
            | CommandId::EnquireLinkResp
            | CommandId::GenericNack
            | CommandId::CancelSmResp
            | CommandId::ReplaceSmResp
            | CommandId::CancelBroadcastSmResp => return Ok(None),
            _ => return Ok(None),
        };

        Ok(Some(body))
    }
}
