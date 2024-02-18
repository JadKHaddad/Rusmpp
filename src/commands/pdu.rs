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

pub mod alert_notification;
pub mod bind;
pub mod bind_resp;
pub mod cancel_sm;
pub mod data_sm;
pub mod deliver_sm;
pub mod outbind;
pub mod query_sm;
pub mod query_sm_resp;
pub mod sm_resp;
pub mod submit_sm;
pub mod submit_sm_resp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pdu {
    /// Authentication PDU used by a transmitter ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information and an access password for the ESME.
    BindTransmitter(bind::Bind),
    /// Message Centre response to a bind_transmitter PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transmitter.
    BindTransmitterResp(bind_resp::BindResp),
    /// Authentication PDU used by a receiver ESME to bind to the
    /// Message Centre. The PDU contains identification information,
    /// an access password for the ESME and may also contain
    /// routing information specifying the range of addresses
    /// serviced by the ESME.
    BindReceiver(bind::Bind),
    /// Message Centre response to a bind_receiver PDU. This PDU
    /// indicates the success or failure of the ESME’s attempt to bind
    /// as a receiver.
    BindReceiverResp(bind_resp::BindResp),
    /// Authentication PDU used by a transceiver ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information, an access password for the ESME and may also
    /// contain routing information specifying the range of addresses
    /// serviced by the ESME.
    BindTransceiver(bind::Bind),
    /// Message Centre response to a bind_transceiver PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transceiver.
    BindTransceiverResp(bind_resp::BindResp),
    /// Authentication PDU used by a Message Centre to Outbind to
    /// an ESME to inform it that messages are present in the MC.
    /// The PDU contains identification, and access password for the
    /// ESME. If the ESME authenticates the request, it will respond
    /// with a bind_receiver or bind_transceiver to begin the process
    /// of binding into the MC.
    Outbind(outbind::Outbind),
    /// The alert_notification PDU is sent by the MC to the ESME across a Receiver or Transceiver
    /// session. It is sent when the MC has detected that a particular mobile subscriber has become
    /// available and a delivery pending flag had been previously set for that subscriber by means of
    /// the set_dpf TLV.
    ///
    /// A typical use of this operation is to trigger a data content ‘Push’ to the subscriber from a WAP
    /// Proxy Server.
    ///
    /// Note: There is no associated alert_notification_resp PDU.
    AlertNotification(alert_notification::AlertNotification),
    /// This operation is used by an ESME to submit a short message to the MC for onward
    /// transmission to a specified short message entity (SME).
    SubmitSm(submit_sm::SubmitSm),
    SubmitSmResp(submit_sm_resp::SubmitSmResp),
    /// This command is issued by the ESME to query the status of a previously submitted short
    /// message.
    /// The matching mechanism is based on the MC assigned message_id and source address.
    /// Where the original submit_sm, data_sm or submit_multi ‘source address’ was defaulted to
    /// NULL, then the source address in the query_sm command should also be set to NULL.
    QuerySm(query_sm::QuerySm),
    QuerySmResp(query_sm_resp::QuerySmResp),
    /// The deliver_sm is issued by the MC to send a message to an ESME. Using this command,
    /// the MC may route a short message to the ESME for delivery.
    DeliverSm(deliver_sm::DeliverSm),
    DeliverSmResp(sm_resp::SmResp),
    /// The data_sm operation is similar to the submit_sm in that it provides a means to submit a
    /// mobile-terminated message. However, data_sm is intended for packet-based applications
    /// such as WAP in that it features a reduced PDU body containing fields relevant to WAP or
    /// packet-based applications.
    DataSm(data_sm::DataSm),
    DataSmResp(sm_resp::SmResp),
    /// This command is issued by the ESME to cancel one or more previously submitted short
    /// messages that are pending delivery. The command may specify a particular message to
    /// cancel, or all messages matching a particular source, destination and service_type.
    ///
    /// If the message_id is set to the ID of a previously submitted message, then provided the
    /// source address supplied by the ESME matches that of the stored message, that message
    /// will be cancelled.
    ///
    /// If the message_id is NULL, all outstanding undelivered messages with matching source and
    /// destination addresses (and service_type if specified) are cancelled.  
    /// Where the original submit_sm, data_sm or submit_multi ‘source address’ is defaulted to
    /// NULL, then the source address in the cancel_sm command should also be NULL.
    CancelSm(cancel_sm::CancelSm),
    // ReplaceSm(ReplaceSm),
    // SubmitMulti(SubmitMulti),
    // SubmitMultiResp(SubmitOrDataSmResp),
    // BroadcastSm(BroadcastSm),
    // BroadcastSmResp(BroadcastSmResp),
    // QueryBroadcastSm(QueryBroadcastSm),
    // QueryBroadcastSmResp(QueryBroadcastSmResp),
    // CancelBroadcastSm(CancelBroadcastSm),
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
    Other {
        command_id: CommandId,
        body: NoFixedSizeOctetString,
    },
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
            Pdu::Outbind(_) => CommandId::Outbind,
            Pdu::AlertNotification(_) => CommandId::AlertNotification,
            Pdu::SubmitSm(_) => CommandId::SubmitSm,
            Pdu::SubmitSmResp(_) => CommandId::SubmitSmResp,
            Pdu::QuerySm(_) => CommandId::QuerySm,
            Pdu::QuerySmResp(_) => CommandId::QuerySmResp,
            Pdu::DeliverSm(_) => CommandId::DeliverSm,
            Pdu::DeliverSmResp(_) => CommandId::DeliverSmResp,
            Pdu::DataSm(_) => CommandId::DataSm,
            Pdu::DataSmResp(_) => CommandId::DataSmResp,
            Pdu::CancelSm(_) => CommandId::CancelSm,
            // Pdu::ReplaceSm(_) => CommandId::ReplaceSm,
            // Pdu::SubmitMulti(_) => CommandId::SubmitMulti,
            // Pdu::SubmitMultiResp(_) => CommandId::SubmitMultiResp,
            // Pdu::BroadcastSm(_) => CommandId::BroadcastSm,
            // Pdu::BroadcastSmResp(_) => CommandId::BroadcastSmResp,
            // Pdu::QueryBroadcastSm(_) => CommandId::QueryBroadcastSm,
            // Pdu::QueryBroadcastSmResp(_) => CommandId::QueryBroadcastSmResp,
            // Pdu::CancelBroadcastSm(_) => CommandId::CancelBroadcastSm,
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
            Pdu::Outbind(body) => body.length(),
            Pdu::AlertNotification(body) => body.length(),
            Pdu::SubmitSm(body) => body.length(),
            Pdu::SubmitSmResp(body) => body.length(),
            Pdu::QuerySm(body) => body.length(),
            Pdu::QuerySmResp(body) => body.length(),
            Pdu::DeliverSm(body) => body.length(),
            Pdu::DeliverSmResp(body) => body.length(),
            Pdu::DataSm(body) => body.length(),
            Pdu::DataSmResp(body) => body.length(),
            Pdu::CancelSm(body) => body.length(),
            Pdu::Unbind => 0,
            Pdu::UnbindResp => 0,
            Pdu::EnquireLink => 0,
            Pdu::EnquireLinkResp => 0,
            Pdu::GenericNack => 0,
            Pdu::CancelSmResp => 0,
            Pdu::ReplaceSmResp => 0,
            Pdu::CancelBroadcastSmResp => 0,
            Pdu::Other { body, .. } => body.length(),
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
            Pdu::Outbind(body) => body.encode_to(writer),
            Pdu::AlertNotification(body) => body.encode_to(writer),
            Pdu::SubmitSm(body) => body.encode_to(writer),
            Pdu::SubmitSmResp(body) => body.encode_to(writer),
            Pdu::QuerySm(body) => body.encode_to(writer),
            Pdu::QuerySmResp(body) => body.encode_to(writer),
            Pdu::DeliverSm(body) => body.encode_to(writer),
            Pdu::DeliverSmResp(body) => body.encode_to(writer),
            Pdu::DataSm(body) => body.encode_to(writer),
            Pdu::DataSmResp(body) => body.encode_to(writer),
            Pdu::CancelSm(body) => body.encode_to(writer),
            Pdu::Unbind => Ok(()),
            Pdu::UnbindResp => Ok(()),
            Pdu::EnquireLink => Ok(()),
            Pdu::EnquireLinkResp => Ok(()),
            Pdu::GenericNack => Ok(()),
            Pdu::CancelSmResp => Ok(()),
            Pdu::ReplaceSmResp => Ok(()),
            Pdu::CancelBroadcastSmResp => Ok(()),
            Pdu::Other { body, .. } => body.encode_to(writer),
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
            CommandId::Outbind => Pdu::Outbind(tri!(outbind::Outbind::decode_from(reader))),
            CommandId::AlertNotification => Pdu::AlertNotification(tri!(
                alert_notification::AlertNotification::decode_from(reader, length)
            )),
            CommandId::SubmitSm => {
                Pdu::SubmitSm(tri!(submit_sm::SubmitSm::decode_from(reader, length)))
            }
            CommandId::SubmitSmResp => Pdu::SubmitSmResp(tri!(
                submit_sm_resp::SubmitSmResp::decode_from(reader, length)
            )),
            CommandId::QuerySm => Pdu::QuerySm(tri!(query_sm::QuerySm::decode_from(reader))),
            CommandId::QuerySmResp => {
                Pdu::QuerySmResp(tri!(query_sm_resp::QuerySmResp::decode_from(reader)))
            }
            CommandId::DeliverSm => {
                Pdu::DeliverSm(tri!(deliver_sm::DeliverSm::decode_from(reader, length)))
            }
            CommandId::DeliverSmResp => {
                Pdu::DeliverSmResp(tri!(sm_resp::SmResp::decode_from(reader, length)))
            }
            CommandId::DataSm => Pdu::DataSm(tri!(data_sm::DataSm::decode_from(reader, length))),
            CommandId::DataSmResp => {
                Pdu::DataSmResp(tri!(sm_resp::SmResp::decode_from(reader, length)))
            }
            CommandId::CancelSm => Pdu::CancelSm(tri!(cancel_sm::CancelSm::decode_from(reader))),
            CommandId::Unbind => Pdu::Unbind,
            CommandId::UnbindResp => Pdu::UnbindResp,
            CommandId::EnquireLink => Pdu::EnquireLink,
            CommandId::EnquireLinkResp => Pdu::EnquireLinkResp,
            CommandId::GenericNack => Pdu::GenericNack,
            CommandId::CancelSmResp => Pdu::CancelSmResp,
            CommandId::ReplaceSmResp => Pdu::ReplaceSmResp,
            CommandId::CancelBroadcastSmResp => Pdu::CancelBroadcastSmResp,
            CommandId::Other(_) => Pdu::Other {
                command_id: key,
                body: tri!(NoFixedSizeOctetString::decode_from(reader, length)),
            },
            _ => unimplemented!(),
        };

        Ok(body)
    }
}
