use super::Pdu;
use crate::{
    commands::types::{npi::Npi, ton::Ton},
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode},
        length::Length,
    },
    impl_length_encode, tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

impl_length_encode! {
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
}

impl QuerySm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        }
    }

    pub fn builder() -> QuerySmBuilder {
        QuerySmBuilder::new()
    }

    pub fn into_query_sm(self) -> Pdu {
        Pdu::QuerySm(self)
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

#[derive(Default)]
pub struct QuerySmBuilder {
    inner: QuerySm,
}

impl QuerySmBuilder {
    pub fn new() -> Self {
        Default::default()
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

    pub fn build(self) -> QuerySm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode::<QuerySm>();
    }
}
