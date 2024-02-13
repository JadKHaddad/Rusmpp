use super::types::command_id::CommandId;
use crate::{
    io::{
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
