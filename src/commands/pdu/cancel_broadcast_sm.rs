use super::Pdu;
use crate::{
    commands::{
        tlvs::tlv::{cancel_broadcast::CancelBroadcastTLV, TLV},
        types::{npi::Npi, service_type::ServiceType, ton::Ton},
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

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
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CancelBroadcastSm {
    /// Set to indicate CBS Application service, if
    /// cancellation of a group of application service
    /// messages is desired.
    ///
    /// Otherwise set to NULL.
    pub serivce_type: ServiceType,
    /// Message ID of the message to be cancelled. This must
    /// be the MC assigned Message ID of the original message.  
    ///
    /// Set to NULL if setting user_message_reference.
    pub message_id: COctetString<1, 65>,
    /// Type of Number of message originator. This is used for
    /// verification purposes, and must match that supplied in
    /// the original message submission request PDU.
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_ton: Ton,
    /// Numbering Plan Identity of message originator.  
    ///
    /// This is used for verification purposes, and must match
    /// that supplied in the original message submission
    /// request PDU.  
    ///
    /// If not known, set to NULL (Unknown).
    pub source_addr_npi: Npi,
    /// Source address of message to be cancelled. This is used
    // for verification purposes, and must match that supplied in
    // the original message submission request PDU.
    //
    // If not known, set to NULL (Unknown).
    pub source_addr: COctetString<1, 21>,
    /// Cancel broadcast  TLVs ([`CancelBroadcastTLV`]).
    tlvs: Vec<TLV>,
}

impl CancelBroadcastSm {
    pub fn new(
        serivce_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        tlvs: Vec<CancelBroadcastTLV>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        }
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<CancelBroadcastTLV>) {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        self.tlvs = tlvs;
    }

    pub fn push_tlv(&mut self, tlv: CancelBroadcastTLV) {
        let tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> CancelBroadcastSmBuilder {
        CancelBroadcastSmBuilder::new()
    }

    pub fn into_cancel_broadcast_sm(self) -> Pdu {
        Pdu::CancelBroadcastSm(self)
    }
}

impl Length for CancelBroadcastSm {
    fn length(&self) -> usize {
        self.serivce_type.length()
            + self.message_id.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.tlvs.length()
    }
}

impl Encode for CancelBroadcastSm {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.serivce_type.encode_to(writer));
        tri!(self.message_id.encode_to(writer));
        tri!(self.source_addr_ton.encode_to(writer));
        tri!(self.source_addr_npi.encode_to(writer));
        tri!(self.source_addr.encode_to(writer));
        tri!(self.tlvs.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for CancelBroadcastSm {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let serivce_type = tri!(ServiceType::decode_from(reader));
        let message_id = tri!(COctetString::decode_from(reader));
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::decode_from(reader));

        let tlvs_length = length.saturating_sub(
            serivce_type.length()
                + message_id.length()
                + source_addr_ton.length()
                + source_addr_npi.length()
                + source_addr.length(),
        );

        let tlvs = tri!(Vec::<TLV>::decode_from(reader, tlvs_length));

        Ok(Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        })
    }
}

#[derive(Default)]
pub struct CancelBroadcastSmBuilder {
    inner: CancelBroadcastSm,
}

impl CancelBroadcastSmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serivce_type(mut self, serivce_type: ServiceType) -> Self {
        self.inner.serivce_type = serivce_type;
        self
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn source_addr_ton(mut self, source_addr_ton: Ton) -> Self {
        self.inner.source_addr_ton = source_addr_ton;
        self
    }

    pub fn source_addr_npi(mut self, source_addr_npi: Npi) -> Self {
        self.inner.source_addr_npi = source_addr_npi;
        self
    }

    pub fn source_addr(mut self, source_addr: COctetString<1, 21>) -> Self {
        self.inner.source_addr = source_addr;
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<CancelBroadcastTLV>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: CancelBroadcastTLV) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> CancelBroadcastSm {
        self.inner
    }
}
