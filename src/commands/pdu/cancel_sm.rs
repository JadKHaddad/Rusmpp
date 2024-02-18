use crate::{
    commands::types::{npi::Npi, service_type::ServiceType, ton::Ton},
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

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
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CancelSm {
    /// Set to indicate SMS Application service,
    /// if cancellation of a group of application
    /// service messages is desired.
    /// Otherwise set to NULL.
    pub serivce_type: ServiceType,
    /// Message ID of the message to be
    /// cancelled. This must be the MC
    /// assigned Message ID of the original
    /// message.
    ///
    /// Set to NULL if cancelling a group of
    /// messages.
    pub message_id: COctetString<1, 65>,
    /// Type of Number of message originator.
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request PDU.
    ///
    /// If not known, set to NULL.
    pub source_addr_ton: Ton,
    /// Numbering Plan Identity of message
    /// originator.  
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request PDU.  
    ///
    /// If not known, set to NULL.
    pub source_addr_npi: Npi,
    /// Source address of message(s) to be
    /// cancelled. This is used for verification
    /// purposes, and must match that supplied
    /// in the original message submission
    /// request PDU(s).
    ///
    /// If not known, set to NULL.
    pub source_addr: COctetString<1, 21>,
    /// Type of number of destination SME
    /// address of the message(s) to be cancelled.
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU (e.g. submit_sm).  
    ///
    /// May be set to NULL when the
    /// message_id is provided.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator of destination
    /// SME address of the message(s) to be
    /// cancelled.
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU.  
    ///
    /// May be set to NULL when the
    /// message_id is provided.
    pub dest_addr_npi: Npi,
    /// Destination address of message(s) to be
    /// cancelled.
    ///
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU.  
    ///
    /// May be set to NULL when the
    /// message_id is provided.
    pub destination_addr: COctetString<1, 21>,
}

impl CancelSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }

    pub fn builder() -> CancelSmBuilder {
        CancelSmBuilder::new()
    }
}

impl Length for CancelSm {
    fn length(&self) -> usize {
        self.serivce_type.length()
            + self.message_id.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.dest_addr_ton.length()
            + self.dest_addr_npi.length()
            + self.destination_addr.length()
    }
}

impl Encode for CancelSm {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.serivce_type.encode_to(writer));
        tri!(self.message_id.encode_to(writer));
        tri!(self.source_addr_ton.encode_to(writer));
        tri!(self.source_addr_npi.encode_to(writer));
        tri!(self.source_addr.encode_to(writer));
        tri!(self.dest_addr_ton.encode_to(writer));
        tri!(self.dest_addr_npi.encode_to(writer));
        tri!(self.destination_addr.encode_to(writer));

        Ok(())
    }
}

impl Decode for CancelSm {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let serivce_type = tri!(ServiceType::decode_from(reader));
        let message_id = tri!(COctetString::decode_from(reader));
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::decode_from(reader));
        let dest_addr_ton = tri!(Ton::decode_from(reader));
        let dest_addr_npi = tri!(Npi::decode_from(reader));
        let destination_addr = tri!(COctetString::decode_from(reader));

        Ok(Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        })
    }
}

#[derive(Default)]
pub struct CancelSmBuilder {
    inner: CancelSm,
}

impl CancelSmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn service_type(mut self, service_type: ServiceType) -> Self {
        self.inner.serivce_type = service_type;
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

    pub fn dest_addr_ton(mut self, dest_addr_ton: Ton) -> Self {
        self.inner.dest_addr_ton = dest_addr_ton;
        self
    }

    pub fn dest_addr_npi(mut self, dest_addr_npi: Npi) -> Self {
        self.inner.dest_addr_npi = dest_addr_npi;
        self
    }

    pub fn destination_addr(mut self, destination_addr: COctetString<1, 21>) -> Self {
        self.inner.destination_addr = destination_addr;
        self
    }

    pub fn build(self) -> CancelSm {
        self.inner
    }
}
