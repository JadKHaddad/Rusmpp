use rusmpp_macros::Rusmpp;

use crate::{
    pdus::borrowed::Pdu,
    tlvs::borrowed::{CancelBroadcastTlvValue, Tlv},
    types::borrowed::COctetString,
    values::{npi::Npi, service_type::borrowed::ServiceType, ton::Ton},
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct CancelBroadcastSm<'a, const N: usize> {
    /// Set to indicate CBS Application service, if
    /// cancellation of a group of application service
    /// messages is desired.
    ///
    /// Otherwise set to NULL.
    pub service_type: ServiceType<'a>,
    /// Message ID of the message to be cancelled. This must
    /// be the MC assigned Message ID of the original message.
    ///
    /// Set to NULL if setting user_message_reference.
    pub message_id: COctetString<'a, 1, 65>,
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
    pub source_addr: COctetString<'a, 1, 21>,
    /// Cancel broadcast  TLVs ([`CancelBroadcastTlvValue`]).
    #[rusmpp(length = "unchecked")]
    #[cfg_attr(feature = "arbitrary", arbitrary(default))]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> CancelBroadcastSm<'a, N> {
    pub fn new(
        service_type: ServiceType<'a>,
        message_id: COctetString<'a, 1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<'a, 1, 21>,
        tlvs: heapless::vec::Vec<impl Into<CancelBroadcastTlvValue>, N>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self {
            service_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        }
    }

    pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: heapless::vec::Vec<impl Into<CancelBroadcastTlvValue>, N>) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<CancelBroadcastTlvValue>) -> Result<(), Tlv<'a>> {
        self.tlvs.push(Tlv::from(tlv.into()))?;
        Ok(())
    }

    pub fn builder() -> CancelBroadcastSmBuilder<'a, N> {
        CancelBroadcastSmBuilder::new()
    }
}

impl<'a, const N: usize> From<CancelBroadcastSm<'a, N>> for Pdu<'a, N> {
    fn from(value: CancelBroadcastSm<'a, N>) -> Self {
        Self::CancelBroadcastSm(value)
    }
}

#[derive(Debug, Default)]
pub struct CancelBroadcastSmBuilder<'a, const N: usize> {
    inner: CancelBroadcastSm<'a, N>,
}

impl<'a, const N: usize> CancelBroadcastSmBuilder<'a, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn service_type(mut self, service_type: ServiceType<'a>) -> Self {
        self.inner.service_type = service_type;
        self
    }

    pub fn message_id(mut self, message_id: COctetString<'a, 1, 65>) -> Self {
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

    pub fn source_addr(mut self, source_addr: COctetString<'a, 1, 21>) -> Self {
        self.inner.source_addr = source_addr;
        self
    }

    pub fn tlvs(mut self, tlvs: heapless::vec::Vec<impl Into<CancelBroadcastTlvValue>, N>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<CancelBroadcastTlvValue>) -> Result<Self, Tlv<'a>> {
        self.inner.push_tlv(tlv)?;
        Ok(self)
    }

    pub fn build(self) -> CancelBroadcastSm<'a, N> {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tests::TestInstance,
        values::{
            broadcast_content_type::{BroadcastContentType, EncodingContentType, TypeOfNetwork},
            user_message_reference::UserMessageReference,
        },
    };

    use super::*;

    impl<const N: usize> TestInstance for CancelBroadcastSm<'_, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::default())
                    .message_id(COctetString::new(b"1234567890\0").unwrap())
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Ermes)
                    .source_addr(COctetString::new(b"1234567890\0").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::new(b"1234567890\0").unwrap())
                    .source_addr(COctetString::new(b"1234567890\0").unwrap())
                    .tlvs(
                        [
                            CancelBroadcastTlvValue::BroadcastContentType(
                                BroadcastContentType::new(
                                    TypeOfNetwork::Gsm,
                                    EncodingContentType::BusinessFinancialNewsInternational,
                                )
                            ),
                            CancelBroadcastTlvValue::UserMessageReference(
                                UserMessageReference::new(16,)
                            ),
                        ]
                        .into()
                    )
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<
            CancelBroadcastSm<'static, 16>,
        >();
    }
}
