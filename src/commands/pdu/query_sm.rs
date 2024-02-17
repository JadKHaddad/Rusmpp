use crate::{
    commands::types::{npi::Npi, ton::Ton},
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

/// This command is issued by the ESME to query the status of a previously submitted short
/// message.
/// The matching mechanism is based on the MC assigned message_id and source address.
/// Where the original submit_sm, data_sm or submit_multi ‘source address’ was defaulted to
/// NULL, then the source address in the query_sm command should also be set to NULL.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QuerySm {
    /// Message ID of the message whose state
    /// is to be queried. This must be the MC
    /// assigned Message ID allocated to the
    /// original short message when submitted
    /// to the MC by the submit_sm, data_sm or
    /// submit_multi command, and returned in
    /// the response PDU by the MC.
    pub message_id: COctetString<1, 65>,
    /// Type of Number of message originator.
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original request PDU (e.g. submit_sm).
    ///
    /// If not known, set to NULL.  
    pub source_addr_ton: Ton,
    /// Numbering Plan Identity of message
    /// originator.  
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original message submission request
    /// PDU.  
    ///
    /// If not known, set to NULL.
    pub source_addr_npi: Npi,
    /// Address of message originator.  
    /// This is used for verification purposes,
    /// and must match that supplied in the
    /// original request PDU (e.g. submit_sm).
    ///
    /// If not known, set to NULL.  
    pub source_addr: COctetString<1, 21>,
}

impl Length for QuerySm {
    fn length(&self) -> usize {
        self.message_id.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
    }
}

impl Encode for QuerySm {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.message_id.encode_to(writer));
        tri!(self.source_addr_ton.encode_to(writer));
        tri!(self.source_addr_npi.encode_to(writer));
        tri!(self.source_addr.encode_to(writer));

        Ok(())
    }
}

impl Decode for QuerySm {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let message_id = tri!(COctetString::decode_from(reader));
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::decode_from(reader));

        Ok(Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        })
    }
}
