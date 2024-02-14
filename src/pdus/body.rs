use super::types::command_id::{CommandId, HasCommandId};
use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithKey, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

pub mod bind;
pub mod bind_resp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Body {
    BindTransmitter(bind::Bind),
    BindTransmitterResp(bind_resp::BindResp),
    BindReceiver(bind::Bind),
    BindReceiverResp(bind_resp::BindResp),
    BindTransceiver(bind::Bind),
    BindTransceiverResp(bind_resp::BindResp),
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
    Other {
        command_id: CommandId,
        body: NoFixedSizeOctetString,
    },
    Unbind,
    UnbindResp,
    EnquireLink,
    EnquireLinkResp,
    GenericNack,
    CancelSmResp,
    ReplaceSmResp,
    CancelBroadcastSmResp,
}

impl HasCommandId for Body {
    fn command_id(&self) -> CommandId {
        match self {
            Body::BindTransmitter(_) => CommandId::BindTransmitter,
            Body::BindTransmitterResp(_) => CommandId::BindTransmitterResp,
            Body::BindReceiver(_) => CommandId::BindReceiver,
            Body::BindReceiverResp(_) => CommandId::BindReceiverResp,
            Body::BindTransceiver(_) => CommandId::BindTransceiver,
            Body::BindTransceiverResp(_) => CommandId::BindTransceiverResp,
            // Body::Outbind(_) => CommandId::Outbind,
            // Body::AlertNotification(_) => CommandId::AlertNotification,
            // Body::SubmitSm(_) => CommandId::SubmitSm,
            // Body::SubmitSmResp(_) => CommandId::SubmitSmResp,
            // Body::QuerySm(_) => CommandId::QuerySm,
            // Body::QuerySmResp(_) => CommandId::QuerySmResp,
            // Body::DeliverSm(_) => CommandId::DeliverSm,
            // Body::DeliverSmResp(_) => CommandId::DeliverSmResp,
            // Body::DataSm(_) => CommandId::DataSm,
            // Body::DataSmResp(_) => CommandId::DataSmResp,
            // Body::CancelSm(_) => CommandId::CancelSm,
            // Body::ReplaceSm(_) => CommandId::ReplaceSm,
            // Body::SubmitMulti(_) => CommandId::SubmitMulti,
            // Body::SubmitMultiResp(_) => CommandId::SubmitMultiResp,
            // Body::BroadcastSm(_) => CommandId::BroadcastSm,
            // Body::BroadcastSmResp(_) => CommandId::BroadcastSmResp,
            // Body::QueryBroadcastSm(_) => CommandId::QueryBroadcastSm,
            // Body::QueryBroadcastSmResp(_) => CommandId::QueryBroadcastSmResp,
            // Body::CancelBroadcastSm(_) => CommandId::CancelBroadcastSm,
            Body::Other { command_id, .. } => *command_id,
            // These are empty bodies
            // The reason they exist it to force the creation of a pdu with the correct command_id using a body
            Body::Unbind => CommandId::Unbind,
            Body::UnbindResp => CommandId::UnbindResp,
            Body::EnquireLink => CommandId::EnquireLink,
            Body::EnquireLinkResp => CommandId::EnquireLinkResp,
            Body::GenericNack => CommandId::GenericNack,
            Body::CancelSmResp => CommandId::CancelSmResp,
            Body::ReplaceSmResp => CommandId::ReplaceSmResp,
            Body::CancelBroadcastSmResp => CommandId::CancelBroadcastSmResp,
        }
    }
}

impl Length for Body {
    fn length(&self) -> usize {
        match self {
            Body::BindTransmitter(body) => body.length(),
            Body::BindTransmitterResp(body) => body.length(),
            Body::BindReceiver(body) => body.length(),
            Body::BindReceiverResp(body) => body.length(),
            Body::BindTransceiver(body) => body.length(),
            Body::BindTransceiverResp(body) => body.length(),
            Body::Other { body, .. } => body.length(),
            Body::Unbind => 0,
            Body::UnbindResp => 0,
            Body::EnquireLink => 0,
            Body::EnquireLinkResp => 0,
            Body::GenericNack => 0,
            Body::CancelSmResp => 0,
            Body::ReplaceSmResp => 0,
            Body::CancelBroadcastSmResp => 0,
        }
    }
}

impl Encode for Body {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Body::BindTransmitter(body) => body.encode_to(writer),
            Body::BindTransmitterResp(body) => body.encode_to(writer),
            Body::BindReceiver(body) => body.encode_to(writer),
            Body::BindReceiverResp(body) => body.encode_to(writer),
            Body::BindTransceiver(body) => body.encode_to(writer),
            Body::BindTransceiverResp(body) => body.encode_to(writer),
            Body::Other { body, .. } => body.encode_to(writer),
            Body::Unbind => Ok(()),
            Body::UnbindResp => Ok(()),
            Body::EnquireLink => Ok(()),
            Body::EnquireLinkResp => Ok(()),
            Body::GenericNack => Ok(()),
            Body::CancelSmResp => Ok(()),
            Body::ReplaceSmResp => Ok(()),
            Body::CancelBroadcastSmResp => Ok(()),
        }
    }
}

impl DecodeWithKey for Body {
    type Key = CommandId;

    fn decode_from<R: std::io::Read>(
        key: Self::Key,
        reader: &mut R,
        length: usize,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let body = match key {
            CommandId::BindTransmitter => {
                Body::BindTransmitter(tri!(bind::Bind::decode_from(reader)))
            }
            CommandId::BindTransmitterResp => {
                Body::BindTransmitterResp(tri!(bind_resp::BindResp::decode_from(reader, length)))
            }
            CommandId::BindReceiver => Body::BindReceiver(tri!(bind::Bind::decode_from(reader))),
            CommandId::BindReceiverResp => {
                Body::BindReceiverResp(tri!(bind_resp::BindResp::decode_from(reader, length)))
            }
            CommandId::BindTransceiver => {
                Body::BindTransceiver(tri!(bind::Bind::decode_from(reader)))
            }
            CommandId::BindTransceiverResp => {
                Body::BindTransceiverResp(tri!(bind_resp::BindResp::decode_from(reader, length)))
            }
            CommandId::Other(_) => Body::Other {
                command_id: key,
                body: tri!(NoFixedSizeOctetString::decode_from(reader, length)),
            },
            CommandId::Unbind => Body::Unbind,
            CommandId::UnbindResp => Body::UnbindResp,
            CommandId::EnquireLink => Body::EnquireLink,
            CommandId::EnquireLinkResp => Body::EnquireLinkResp,
            CommandId::GenericNack => Body::GenericNack,
            CommandId::CancelSmResp => Body::CancelSmResp,
            CommandId::ReplaceSmResp => Body::ReplaceSmResp,
            CommandId::CancelBroadcastSmResp => Body::CancelBroadcastSmResp,
            _ => unimplemented!(),
        };

        Ok(body)
    }
}
