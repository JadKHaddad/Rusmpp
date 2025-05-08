//! `SMPP` PDUs.

use super::types::command_id::CommandId;
use crate::{
    errors::DecodeError, tri, types::any_octet_string::AnyOctetString, DecodeWithKeyOptional,
    Encode, Length,
};

pub mod alert_notification;
pub use alert_notification::AlertNotification;

pub mod bind;
pub use bind::{BindReceiver, BindTransceiver, BindTransmitter};

pub mod bind_resp;
pub use bind_resp::{BindReceiverResp, BindTransceiverResp, BindTransmitterResp};

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
pub use sm_resp::{DataSmResp, DeliverSmResp};

pub mod submit_sm;
pub use submit_sm::SubmitSm;

pub mod submit_sm_resp;
pub use submit_sm_resp::SubmitSmResp;

pub mod submit_multi;
pub use submit_multi::SubmitMulti;

pub mod submit_multi_resp;
pub use submit_multi_resp::SubmitMultiResp;

pub mod broadcast_sm;
pub use broadcast_sm::BroadcastSm;

pub mod broadcast_sm_resp;
pub use broadcast_sm_resp::BroadcastSmResp;

pub mod query_broadcast_sm;
pub use query_broadcast_sm::QueryBroadcastSm;

pub mod query_broadcast_sm_resp;
pub use query_broadcast_sm_resp::QueryBroadcastSmResp;

pub mod cancel_broadcast_sm;
pub use cancel_broadcast_sm::CancelBroadcastSm;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pdu {
    /// Authentication PDU used by a transmitter ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information and an access password for the ESME.
    BindTransmitter(BindTransmitter),
    /// Message Centre response to a bind_transmitter PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transmitter.
    BindTransmitterResp(BindTransmitterResp),
    /// Authentication PDU used by a receiver ESME to bind to the
    /// Message Centre. The PDU contains identification information,
    /// an access password for the ESME and may also contain
    /// routing information specifying the range of addresses
    /// serviced by the ESME.
    BindReceiver(BindReceiver),
    /// Message Centre response to a bind_receiver PDU. This PDU
    /// indicates the success or failure of the ESME’s attempt to bind
    /// as a receiver.
    BindReceiverResp(BindReceiverResp),
    /// Authentication PDU used by a transceiver ESME to bind to
    /// the Message Centre. The PDU contains identification
    /// information, an access password for the ESME and may also
    /// contain routing information specifying the range of addresses
    /// serviced by the ESME.
    BindTransceiver(BindTransceiver),
    /// Message Centre response to a bind_transceiver PDU. This
    /// PDU indicates the success or failure of the ESME’s attempt
    /// to bind as a transceiver.
    BindTransceiverResp(BindTransceiverResp),
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
    DeliverSmResp(DeliverSmResp),
    /// The data_sm operation is similar to the submit_sm in that it provides a means to submit a
    /// mobile-terminated message. However, data_sm is intended for packet-based applications
    /// such as WAP in that it features a reduced PDU body containing fields relevant to WAP or
    /// packet-based applications.
    DataSm(DataSm),
    DataSmResp(DataSmResp),
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
    /// The submit_multi operation is an enhanced variation of submit_sm designed to support up to
    /// 255 different destinations instead of the default single destination. It provides an efficient
    /// means of sending the same message to several different subscribers at the same time.
    SubmitMulti(SubmitMulti),
    SubmitMultiResp(SubmitMultiResp),
    /// This operation is issued by the ESME to submit a message to the Message Centre for
    /// broadcast to a specified geographical area or set of geographical areas.
    BroadcastSm(BroadcastSm),
    BroadcastSmResp(BroadcastSmResp),
    /// This command is issued by the ESME to query the status of a previously submitted
    /// broadcast message. The message can be queried either on the basis of the Message Center
    /// assigned reference message_id returned in the broadcast_sm_resp or by the ESME
    /// assigned message reference number user_message_reference as indicated in the
    /// broadcast_sm operation associated with that message.
    ///
    /// Note:  Where the broadcast is queried on the basis of the ESME assigned message
    /// reference user_message_reference this should be qualified within the service by the
    /// system_id and/or the system_type associated with the query_broadcast_sm operation
    /// (specified in the bind operation). If more than one message with the same
    /// user_message_reference value is present in the Message Center, the details of the most
    /// recently submitted message with the specified user_message_reference value will be
    /// returned in the query_broadcast_sm_resp.
    QueryBroadcastSm(QueryBroadcastSm),
    QueryBroadcastSmResp(QueryBroadcastSmResp),
    /// This command is issued by the ESME to cancel a broadcast message which has been
    /// previously submitted to the Message Centre for broadcast via broadcast_sm and which is still
    /// pending delivery.  
    ///
    /// If the message_id is set to the ID of a previously submitted message, then provided the
    /// source address supplied by the ESME matches that of the stored message, that message
    /// will be cancelled.
    ///
    /// If the message_id is NULL, all outstanding undelivered messages with matching source and
    /// destination addresses (and service_type if specified) are cancelled.
    ///
    /// If the user_message_reference is set to the ESME-assigned reference of a previously
    /// submitted message, then provided the source address supplied by the ESME matches that of
    /// the stored message, that message will be cancelled.
    ///
    /// Where the original broadcast_sm ‘source address’ was defaulted to NULL, then the source
    /// address in the cancel_broadcast_sm command should also be NULL.
    CancelBroadcastSm(CancelBroadcastSm),
    /// This PDU can be sent by the ESME or MC as a means of
    /// initiating the termination of a `SMPP` session.
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
        body: AnyOctetString,
    },
}

impl Pdu {
    pub const fn command_id(&self) -> CommandId {
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
            Pdu::SubmitMulti(_) => CommandId::SubmitMulti,
            Pdu::SubmitMultiResp(_) => CommandId::SubmitMultiResp,
            Pdu::BroadcastSm(_) => CommandId::BroadcastSm,
            Pdu::BroadcastSmResp(_) => CommandId::BroadcastSmResp,
            Pdu::QueryBroadcastSm(_) => CommandId::QueryBroadcastSm,
            Pdu::QueryBroadcastSmResp(_) => CommandId::QueryBroadcastSmResp,
            Pdu::CancelBroadcastSm(_) => CommandId::CancelBroadcastSm,
            Pdu::Other { command_id, .. } => *command_id,
            // These are empty pdus.
            // The reason they exist is to force the creation of a command with the correct command_id using a pdu.
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
            Pdu::SubmitMulti(body) => body.length(),
            Pdu::SubmitMultiResp(body) => body.length(),
            Pdu::BroadcastSm(body) => body.length(),
            Pdu::BroadcastSmResp(body) => body.length(),
            Pdu::QueryBroadcastSm(body) => body.length(),
            Pdu::QueryBroadcastSmResp(body) => body.length(),
            Pdu::CancelBroadcastSm(body) => body.length(),
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
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            Pdu::BindTransmitter(body) => body.encode(dst),
            Pdu::BindTransmitterResp(body) => body.encode(dst),
            Pdu::BindReceiver(body) => body.encode(dst),
            Pdu::BindReceiverResp(body) => body.encode(dst),
            Pdu::BindTransceiver(body) => body.encode(dst),
            Pdu::BindTransceiverResp(body) => body.encode(dst),
            Pdu::Outbind(body) => body.encode(dst),
            Pdu::AlertNotification(body) => body.encode(dst),
            Pdu::SubmitSm(body) => body.encode(dst),
            Pdu::SubmitSmResp(body) => body.encode(dst),
            Pdu::QuerySm(body) => body.encode(dst),
            Pdu::QuerySmResp(body) => body.encode(dst),
            Pdu::DeliverSm(body) => body.encode(dst),
            Pdu::DeliverSmResp(body) => body.encode(dst),
            Pdu::DataSm(body) => body.encode(dst),
            Pdu::DataSmResp(body) => body.encode(dst),
            Pdu::CancelSm(body) => body.encode(dst),
            Pdu::ReplaceSm(body) => body.encode(dst),
            Pdu::SubmitMulti(body) => body.encode(dst),
            Pdu::SubmitMultiResp(body) => body.encode(dst),
            Pdu::BroadcastSm(body) => body.encode(dst),
            Pdu::BroadcastSmResp(body) => body.encode(dst),
            Pdu::QueryBroadcastSm(body) => body.encode(dst),
            Pdu::QueryBroadcastSmResp(body) => body.encode(dst),
            Pdu::CancelBroadcastSm(body) => body.encode(dst),
            Pdu::Unbind
            | Pdu::UnbindResp
            | Pdu::EnquireLink
            | Pdu::EnquireLinkResp
            | Pdu::GenericNack
            | Pdu::CancelSmResp
            | Pdu::ReplaceSmResp
            | Pdu::CancelBroadcastSmResp => 0,
            Pdu::Other { body, .. } => body.encode(dst),
        }
    }
}

impl DecodeWithKeyOptional for Pdu {
    type Key = CommandId;

    fn decode(
        key: Self::Key,
        src: &mut [u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        if length == 0 {
            let body = match key {
                CommandId::Unbind => Pdu::Unbind,
                CommandId::UnbindResp => Pdu::UnbindResp,
                CommandId::EnquireLink => Pdu::EnquireLink,
                CommandId::EnquireLinkResp => Pdu::EnquireLinkResp,
                CommandId::GenericNack => Pdu::GenericNack,
                CommandId::CancelSmResp => Pdu::CancelSmResp,
                CommandId::ReplaceSmResp => Pdu::ReplaceSmResp,
                CommandId::CancelBroadcastSmResp => Pdu::CancelBroadcastSmResp,
                _ => return Ok(None),
            };

            return Ok(Some(body));
        }

        let (body, size) = match key {
            CommandId::BindTransmitter => Pdu::BindTransmitter(tri!(BindTransmitter::decode(src))),
            CommandId::BindTransmitterResp => {
                Pdu::BindTransmitterResp(tri!(BindTransmitterResp::decode(src, length)))
            }
            CommandId::BindReceiver => Pdu::BindReceiver(tri!(BindReceiver::decode(src))),
            CommandId::BindReceiverResp => {
                Pdu::BindReceiverResp(tri!(BindReceiverResp::decode(src, length)))
            }
            CommandId::BindTransceiver => Pdu::BindTransceiver(tri!(BindTransceiver::decode(src))),
            CommandId::BindTransceiverResp => {
                Pdu::BindTransceiverResp(tri!(BindTransceiverResp::decode(src, length)))
            }
            CommandId::Outbind => Pdu::Outbind(tri!(Outbind::decode(src))),
            CommandId::AlertNotification => {
                Pdu::AlertNotification(tri!(AlertNotification::decode(src, length)))
            }
            CommandId::SubmitSm => Pdu::SubmitSm(tri!(SubmitSm::decode(src, length))),
            CommandId::SubmitSmResp => Pdu::SubmitSmResp(tri!(SubmitSmResp::decode(src, length))),
            CommandId::QuerySm => Pdu::QuerySm(tri!(QuerySm::decode(src))),
            CommandId::QuerySmResp => Pdu::QuerySmResp(tri!(QuerySmResp::decode(src))),
            CommandId::DeliverSm => Pdu::DeliverSm(tri!(DeliverSm::decode(src, length))),
            CommandId::DeliverSmResp => {
                Pdu::DeliverSmResp(tri!(DeliverSmResp::decode(src, length)))
            }
            CommandId::DataSm => Pdu::DataSm(tri!(DataSm::decode(src, length))),
            CommandId::DataSmResp => Pdu::DataSmResp(tri!(DataSmResp::decode(src, length))),
            CommandId::CancelSm => Pdu::CancelSm(tri!(CancelSm::decode(src))),
            CommandId::ReplaceSm => Pdu::ReplaceSm(tri!(ReplaceSm::decode(src, length))),
            CommandId::SubmitMulti => Pdu::SubmitMulti(tri!(SubmitMulti::decode(src, length))),
            CommandId::SubmitMultiResp => {
                Pdu::SubmitMultiResp(tri!(SubmitMultiResp::decode(src, length)))
            }
            CommandId::BroadcastSm => Pdu::BroadcastSm(tri!(BroadcastSm::decode(src, length))),
            CommandId::BroadcastSmResp => {
                Pdu::BroadcastSmResp(tri!(BroadcastSmResp::decode(src, length)))
            }
            CommandId::QueryBroadcastSm => {
                Pdu::QueryBroadcastSm(tri!(QueryBroadcastSm::decode(src, length)))
            }
            CommandId::QueryBroadcastSmResp => {
                Pdu::QueryBroadcastSmResp(tri!(QueryBroadcastSmResp::decode(src, length)))
            }
            CommandId::CancelBroadcastSm => {
                Pdu::CancelBroadcastSm(tri!(CancelBroadcastSm::decode(src, length)))
            }
            CommandId::Other(_) => Pdu::Other {
                command_id: key,
                body: tri!(AnyOctetString::decode(src, length)),
            },
            // Length is not 0 and still have to decode the body. This is an invalid PDU.
            CommandId::Unbind
            | CommandId::UnbindResp
            | CommandId::EnquireLink
            | CommandId::EnquireLinkResp
            | CommandId::GenericNack
            | CommandId::CancelSmResp
            | CommandId::ReplaceSmResp
            | CommandId::CancelBroadcastSmResp => return Ok(None),
        };

        // Ok(Some(body))

        todo!()
    }
}
