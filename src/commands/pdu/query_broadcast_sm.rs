use super::Pdu;
use crate::{
    commands::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{npi::Npi, ton::Ton},
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        length::Length,
    },
    impl_length_encode, tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

impl_length_encode! {
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
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct QueryBroadcastSm {
        /// Message ID of the message to be queried. This must be
        /// the MC assigned Message ID allocated to the original
        /// short message when submitted to the MC by the
        /// broadcast_sm command, and returned in the response
        /// PDU by the MC.
        ///
        /// Set to NULL if setting user_message_reference.
        pub message_id: COctetString<1, 65>,
        /// Type of Number for source address.
        ///
        /// If not known, set to NULL (Unknown).
        pub source_addr_ton: Ton,
        /// Numbering Plan Indicator for source address.
        ///
        /// If not known, set to NULL (Unknown).
        pub source_addr_npi: Npi,
        /// Address of SME which originated this message.
        ///
        /// If not known, set to NULL (Unknown).
        pub source_addr: COctetString<1, 21>,
        /// [`TLVValue::UserMessageReference`].
        ///
        /// ESME assigned message reference number.
        user_message_reference: Option<TLV>,
    }
}

impl QueryBroadcastSm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        user_message_reference: Option<u16>,
    ) -> Self {
        let user_message_reference =
            user_message_reference.map(|value| TLV::new(TLVValue::UserMessageReference(value)));

        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            user_message_reference,
        }
    }

    pub fn user_message_reference(&self) -> Option<&TLV> {
        self.user_message_reference.as_ref()
    }

    pub fn set_user_message_reference(&mut self, user_message_reference: Option<u16>) {
        self.user_message_reference =
            user_message_reference.map(|value| TLV::new(TLVValue::UserMessageReference(value)));
    }

    pub fn builder() -> QueryBroadcastSmBuilder {
        QueryBroadcastSmBuilder::new()
    }

    pub fn into_query_broadcast_sm(self) -> Pdu {
        Pdu::QueryBroadcastSm(self)
    }
}

impl DecodeWithLength for QueryBroadcastSm {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let message_id = tri!(COctetString::<1, 65>::decode_from(reader));
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::<1, 21>::decode_from(reader));

        let user_message_reference_length = length
            .saturating_sub(message_id.length())
            .saturating_sub(source_addr_ton.length())
            .saturating_sub(source_addr_npi.length())
            .saturating_sub(source_addr.length());

        let user_message_reference = tri!(TLV::length_checked_decode_from(
            reader,
            user_message_reference_length
        ));

        Ok(Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            user_message_reference,
        })
    }
}

#[derive(Debug, Default)]
pub struct QueryBroadcastSmBuilder {
    inner: QueryBroadcastSm,
}

impl QueryBroadcastSmBuilder {
    pub fn new() -> Self {
        Self::default()
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

    pub fn user_message_reference(mut self, user_message_reference: Option<u16>) -> Self {
        self.inner
            .set_user_message_reference(user_message_reference);
        self
    }

    pub fn build(self) -> QueryBroadcastSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<QueryBroadcastSm>();
    }
}
