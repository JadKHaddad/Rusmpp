use super::Pdu;
use crate::{
    commands::{
        tlvs::tlv::{cancel_broadcast::CancelBroadcastTlv, Tlv},
        types::{npi::Npi, service_type::ServiceType, ton::Ton},
    },
    types::COctetString,
};

crate::create! {
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
        pub service_type: ServiceType,
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
        @[length = unchecked]
        tlvs: Vec<Tlv>,
    }
}

impl CancelBroadcastSm {
    pub fn new(
        service_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        tlvs: Vec<impl Into<CancelBroadcastTlv>>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        Self {
            service_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        }
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<CancelBroadcastTlv>>) {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        self.tlvs = tlvs;
    }

    pub fn push_tlv(&mut self, tlv: impl Into<CancelBroadcastTlv>) {
        let tlv: CancelBroadcastTlv = tlv.into();
        let tlv: Tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> CancelBroadcastSmBuilder {
        CancelBroadcastSmBuilder::new()
    }
}

impl From<CancelBroadcastSm> for Pdu {
    fn from(value: CancelBroadcastSm) -> Self {
        Self::CancelBroadcastSm(value)
    }
}

#[derive(Debug, Default)]
pub struct CancelBroadcastSmBuilder {
    inner: CancelBroadcastSm,
}

impl CancelBroadcastSmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn service_type(mut self, service_type: ServiceType) -> Self {
        self.inner.service_type = service_type;
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

    pub fn tlvs(mut self, tlvs: Vec<impl Into<CancelBroadcastTlv>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<CancelBroadcastTlv>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> CancelBroadcastSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<CancelBroadcastSm>();
    }
}
