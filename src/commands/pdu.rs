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
pub enum Pdu {
    /// Authentication PDU used by a transmitter ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information and an access password for the ESME.
    BindTransmitter(bind::Bind),
    /// Message Centre response to a bind_transmitter PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transmitter
    BindTransmitterResp(bind_resp::BindResp),
    /// Authentication PDU used by a receiver ESME to bind to the
    /// Message Centre. The PDU contains identification information,
    /// an access password for the ESME and may also contain
    /// routing information specifying the range of addresses
    /// serviced by the ESME.
    BindReceiver(bind::Bind),
    /// Message Centre response to a bind_receiver PDU. This PDU
    /// indicates the success or failure of the ESME’s attempt to bind
    /// as a receiver
    BindReceiverResp(bind_resp::BindResp),
    /// Authentication PDU used by a transceiver ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information, an access password for the ESME and may also
    /// contain routing information specifying the range of addresses
    /// serviced by the ESME.
    BindTransceiver(bind::Bind),
    /// Message Centre response to a bind_transceiver PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transceiver
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
    /// This PDU can be sent by the ESME or MC as a means of
    /// initiating the termination of a SMPP session.
    Unbind,
    /// This PDU can be sent by the ESME or MC as a means of
    /// acknowledging the receipt of an unbind request. After
    /// sending this PDU the MC typically closes the network
    /// connection.
    UnbindResp,
    /// This PDU can be sent by the ESME or MC to test the network
    /// connection. The receiving peer is expected to acknowledge
    /// the PDU as a means of verifying the test.
    EnquireLink,
    /// This PDU is used to acknowledge an enquire_link request
    /// sent by an ESME or MC.
    EnquireLinkResp,
    /// This PDU can be sent by an ESME or MC as a means of
    /// indicating the receipt of an invalid PDU. The receipt of a
    /// generic_nack usually indicates that the remote peer either
    /// cannot identify the PDU or has deemed it an invalid PDU due
    /// to its size or content.
    GenericNack,
    /// The MC returns this PDU to indicate the success or failure of
    /// a cancel_sm PDU.
    CancelSmResp,
    /// The replace_sm_resp PDU indicates the success or failure of
    /// a replace_sm PDU.
    ReplaceSmResp,
    /// The MC returns a query_broadcast_sm_resp PDU as a
    /// means of indicating the result of a broadcast query
    /// attempt. The PDU will indicate the success or failure of the
    /// attempt and for successful attempts will also include the
    /// current state of the message.
    CancelBroadcastSmResp,
}

impl HasCommandId for Pdu {
    fn command_id(&self) -> CommandId {
        match self {
            Pdu::BindTransmitter(_) => CommandId::BindTransmitter,
            Pdu::BindTransmitterResp(_) => CommandId::BindTransmitterResp,
            Pdu::BindReceiver(_) => CommandId::BindReceiver,
            Pdu::BindReceiverResp(_) => CommandId::BindReceiverResp,
            Pdu::BindTransceiver(_) => CommandId::BindTransceiver,
            Pdu::BindTransceiverResp(_) => CommandId::BindTransceiverResp,
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
            Pdu::Other { command_id, .. } => *command_id,
            // These are empty bodies
            // The reason they exist it to force the creation of a pdu with the correct command_id using a body
            Pdu::Unbind => CommandId::Unbind,
            Pdu::UnbindResp => CommandId::UnbindResp,
            Pdu::EnquireLink => CommandId::EnquireLink,
            Pdu::EnquireLinkResp => CommandId::EnquireLinkResp,
            Pdu::GenericNack => CommandId::GenericNack,
            Pdu::CancelSmResp => CommandId::CancelSmResp,
            Pdu::ReplaceSmResp => CommandId::ReplaceSmResp,
            Pdu::CancelBroadcastSmResp => CommandId::CancelBroadcastSmResp,
        }
    }
}

impl Length for Pdu {
    fn length(&self) -> usize {
        match self {
            Pdu::BindTransmitter(body) => body.length(),
            Pdu::BindTransmitterResp(body) => body.length(),
            Pdu::BindReceiver(body) => body.length(),
            Pdu::BindReceiverResp(body) => body.length(),
            Pdu::BindTransceiver(body) => body.length(),
            Pdu::BindTransceiverResp(body) => body.length(),
            Pdu::Other { body, .. } => body.length(),
            Pdu::Unbind => 0,
            Pdu::UnbindResp => 0,
            Pdu::EnquireLink => 0,
            Pdu::EnquireLinkResp => 0,
            Pdu::GenericNack => 0,
            Pdu::CancelSmResp => 0,
            Pdu::ReplaceSmResp => 0,
            Pdu::CancelBroadcastSmResp => 0,
        }
    }
}

impl Encode for Pdu {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Pdu::BindTransmitter(body) => body.encode_to(writer),
            Pdu::BindTransmitterResp(body) => body.encode_to(writer),
            Pdu::BindReceiver(body) => body.encode_to(writer),
            Pdu::BindReceiverResp(body) => body.encode_to(writer),
            Pdu::BindTransceiver(body) => body.encode_to(writer),
            Pdu::BindTransceiverResp(body) => body.encode_to(writer),
            Pdu::Other { body, .. } => body.encode_to(writer),
            Pdu::Unbind => Ok(()),
            Pdu::UnbindResp => Ok(()),
            Pdu::EnquireLink => Ok(()),
            Pdu::EnquireLinkResp => Ok(()),
            Pdu::GenericNack => Ok(()),
            Pdu::CancelSmResp => Ok(()),
            Pdu::ReplaceSmResp => Ok(()),
            Pdu::CancelBroadcastSmResp => Ok(()),
        }
    }
}

impl DecodeWithKey for Pdu {
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
                Pdu::BindTransmitter(tri!(bind::Bind::decode_from(reader)))
            }
            CommandId::BindTransmitterResp => {
                Pdu::BindTransmitterResp(tri!(bind_resp::BindResp::decode_from(reader, length)))
            }
            CommandId::BindReceiver => Pdu::BindReceiver(tri!(bind::Bind::decode_from(reader))),
            CommandId::BindReceiverResp => {
                Pdu::BindReceiverResp(tri!(bind_resp::BindResp::decode_from(reader, length)))
            }
            CommandId::BindTransceiver => {
                Pdu::BindTransceiver(tri!(bind::Bind::decode_from(reader)))
            }
            CommandId::BindTransceiverResp => {
                Pdu::BindTransceiverResp(tri!(bind_resp::BindResp::decode_from(reader, length)))
            }
            CommandId::Other(_) => Pdu::Other {
                command_id: key,
                body: tri!(NoFixedSizeOctetString::decode_from(reader, length)),
            },
            CommandId::Unbind => Pdu::Unbind,
            CommandId::UnbindResp => Pdu::UnbindResp,
            CommandId::EnquireLink => Pdu::EnquireLink,
            CommandId::EnquireLinkResp => Pdu::EnquireLinkResp,
            CommandId::GenericNack => Pdu::GenericNack,
            CommandId::CancelSmResp => Pdu::CancelSmResp,
            CommandId::ReplaceSmResp => Pdu::ReplaceSmResp,
            CommandId::CancelBroadcastSmResp => Pdu::CancelBroadcastSmResp,
            _ => unimplemented!(),
        };

        Ok(body)
    }
}
