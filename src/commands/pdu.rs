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
pub use alert_notification::AlertNotification;

pub mod bind;
pub use bind::Bind;

pub mod bind_resp;
pub use bind_resp::BindResp;

pub mod cancel_sm;
pub use cancel_sm::CancelSm;

pub mod data_sm;
pub use data_sm::DataSm;

pub mod deliver_sm;
pub use deliver_sm::DeliverSm;

pub mod outbind;
pub use outbind::Outbind;

pub mod query_sm;
pub use query_sm::QuerySm;

pub mod query_sm_resp;
pub use query_sm_resp::QuerySmResp;

pub mod replace_sm;
pub use replace_sm::ReplaceSm;

pub mod sm_resp;
pub use sm_resp::SmResp;

pub mod submit_sm;
pub use submit_sm::SubmitSm;

pub mod submit_sm_resp;
pub use submit_sm_resp::SubmitSmResp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pdu {
    /// Authentication PDU used by a transmitter ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information and an access password for the ESME.
    BindTransmitter(Bind),
    /// Message Centre response to a bind_transmitter PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transmitter.
    BindTransmitterResp(BindResp),
    /// Authentication PDU used by a receiver ESME to bind to the
    /// Message Centre. The PDU contains identification information,
    /// an access password for the ESME and may also contain
    /// routing information specifying the range of addresses
    /// serviced by the ESME.
    BindReceiver(Bind),
    /// Message Centre response to a bind_receiver PDU. This PDU
    /// indicates the success or failure of the ESME’s attempt to bind
    /// as a receiver.
    BindReceiverResp(BindResp),
    /// Authentication PDU used by a transceiver ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information, an access password for the ESME and may also
    /// contain routing information specifying the range of addresses
    /// serviced by the ESME.
    BindTransceiver(Bind),
    /// Message Centre response to a bind_transceiver PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transceiver.
    BindTransceiverResp(BindResp),
    /// Authentication PDU used by a Message Centre to Outbind to
    /// an ESME to inform it that messages are present in the MC.
    /// The PDU contains identification, and access password for the
    /// ESME. If the ESME authenticates the request, it will respond
    /// with a bind_receiver or bind_transceiver to begin the process
    /// of binding into the MC.
    Outbind(Outbind),
    /// The alert_notification PDU is sent by the MC to the ESME across a Receiver or Transceiver
    /// session. It is sent when the MC has detected that a particular mobile subscriber has become
    /// available and a delivery pending flag had been previously set for that subscriber by means of
    /// the set_dpf TLV.
    ///
    /// A typical use of this operation is to trigger a data content ‘Push’ to the subscriber from a WAP
    /// Proxy Server.
    ///
    /// Note: There is no associated alert_notification_resp PDU.
    AlertNotification(AlertNotification),
    /// This operation is used by an ESME to submit a short message to the MC for onward
    /// transmission to a specified short message entity (SME).
    SubmitSm(SubmitSm),
    SubmitSmResp(SubmitSmResp),
    /// This command is issued by the ESME to query the status of a previously submitted short
    /// message.
    /// The matching mechanism is based on the MC assigned message_id and source address.
    /// Where the original submit_sm, data_sm or submit_multi ‘source address’ was defaulted to
    /// NULL, then the source address in the query_sm command should also be set to NULL.
    QuerySm(QuerySm),
    QuerySmResp(QuerySmResp),
    /// The deliver_sm is issued by the MC to send a message to an ESME. Using this command,
    /// the MC may route a short message to the ESME for delivery.
    DeliverSm(DeliverSm),
    DeliverSmResp(SmResp),
    /// The data_sm operation is similar to the submit_sm in that it provides a means to submit a
    /// mobile-terminated message. However, data_sm is intended for packet-based applications
    /// such as WAP in that it features a reduced PDU body containing fields relevant to WAP or
    /// packet-based applications.
    DataSm(DataSm),
    DataSmResp(SmResp),
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
    CancelSm(CancelSm),
    /// This command is issued by the ESME to replace a previously submitted short message that
    /// is pending delivery. The matching mechanism is based on the message_id and source
    /// address of the original message.
    ///
    /// Where the original submit_sm ‘source address’ was defaulted to NULL, then the source
    /// address in the replace_sm command should also be NULL.  
    ReplaceSm(ReplaceSm),
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
            Pdu::ReplaceSm(_) => CommandId::ReplaceSm,
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
            Pdu::ReplaceSm(body) => body.length(),
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
            Pdu::ReplaceSm(body) => body.encode_to(writer),
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
            CommandId::BindTransmitter => Pdu::BindTransmitter(tri!(Bind::decode_from(reader))),
            CommandId::BindTransmitterResp => {
                Pdu::BindTransmitterResp(tri!(BindResp::decode_from(reader, length)))
            }
            CommandId::BindReceiver => Pdu::BindReceiver(tri!(Bind::decode_from(reader))),
            CommandId::BindReceiverResp => {
                Pdu::BindReceiverResp(tri!(BindResp::decode_from(reader, length)))
            }
            CommandId::BindTransceiver => Pdu::BindTransceiver(tri!(Bind::decode_from(reader))),
            CommandId::BindTransceiverResp => {
                Pdu::BindTransceiverResp(tri!(BindResp::decode_from(reader, length)))
            }
            CommandId::Outbind => Pdu::Outbind(tri!(Outbind::decode_from(reader))),
            CommandId::AlertNotification => {
                Pdu::AlertNotification(tri!(AlertNotification::decode_from(reader, length)))
            }
            CommandId::SubmitSm => Pdu::SubmitSm(tri!(SubmitSm::decode_from(reader, length))),
            CommandId::SubmitSmResp => {
                Pdu::SubmitSmResp(tri!(SubmitSmResp::decode_from(reader, length)))
            }
            CommandId::QuerySm => Pdu::QuerySm(tri!(QuerySm::decode_from(reader))),
            CommandId::QuerySmResp => Pdu::QuerySmResp(tri!(QuerySmResp::decode_from(reader))),
            CommandId::DeliverSm => Pdu::DeliverSm(tri!(DeliverSm::decode_from(reader, length))),
            CommandId::DeliverSmResp => {
                Pdu::DeliverSmResp(tri!(SmResp::decode_from(reader, length)))
            }
            CommandId::DataSm => Pdu::DataSm(tri!(DataSm::decode_from(reader, length))),
            CommandId::DataSmResp => Pdu::DataSmResp(tri!(SmResp::decode_from(reader, length))),
            CommandId::CancelSm => Pdu::CancelSm(tri!(CancelSm::decode_from(reader))),
            CommandId::ReplaceSm => Pdu::ReplaceSm(tri!(ReplaceSm::decode_from(reader, length))),
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
