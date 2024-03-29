use super::Pdu;
use crate::{
    commands::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{npi::Npi, registered_delivery::RegisteredDelivery, ton::Ton},
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        length::Length,
    },
    impl_length_encode, tri,
    types::{
        any_octet_string::AnyOctetString, c_octet_string::COctetString,
        empty_or_full_c_octet_string::EmptyOrFullCOctetString, octet_string::OctetString,
        u8::EndeU8,
    },
};

impl_length_encode! {
    /// This command is issued by the ESME to replace a previously submitted short message that
    /// is pending delivery. The matching mechanism is based on the message_id and source
    /// address of the original message.
    ///
    /// Where the original submit_sm ‘source address’ was defaulted to NULL, then the source
    /// address in the replace_sm command should also be NULL.
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct ReplaceSm {
        /// Message ID of the message to be replaced.
        /// This must be the MC assigned Message ID
        /// allocated to the original short message when
        /// submitted to the MC by the submit_sm, data_sm or
        /// submit_multi command, and returned in the
        /// response PDU by the MC.
        pub message_id: COctetString<1, 65>,
        /// Type of Number of message originator. This is used for
        /// verification purposes, and must match that supplied in
        /// the original request PDU (e.g. submit_sm).
        ///
        /// If not known, set to NULL.
        pub source_addr_ton: Ton,
        /// Numbering Plan Indicator for source address of
        /// original message.
        ///
        /// If not known, set to NULL (Unknown).
        pub source_addr_npi: Npi,
        /// Address of SME, which originated this message.
        /// If not known, set to NULL (Unknown).
        pub source_addr: COctetString<1, 21>,
        /// New scheduled delivery time for the short message.
        // Set to NULL to preserve the original scheduled
        // delivery time.
        pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
        /// New expiry time for the short message.
        ///
        /// Set to NULL to preserve
        /// the original validity period
        /// setting.
        pub validity_period: EmptyOrFullCOctetString<17>,
        /// Indicator to signify if a MC delivery receipt,
        /// user/manual or delivery ACK or intermediate
        /// notification is required.
        pub registered_delivery: RegisteredDelivery,
        /// Indicates the short message to send from a list
        /// of predefined (‘canned’) short messages stored on
        /// the MC.
        ///
        /// If not using a MC canned message, set to NULL.
        pub sm_default_msg_id: u8,
        /// Length in octets of the short_message user data.
        sm_length: u8,
        /// Up to 255 octets of short message user data.
        /// The exact physical limit for short_message size may
        /// vary according to the underlying network
        ///
        /// Note: this field is superceded by the message_payload TLV if specified.
        ///
        /// Applications which need to send messages longer than
        /// 255 octets should use the message_payload TLV. In
        /// this case the sm_length field should be set to zero.
        short_message: OctetString<0, 255>,
        /// Message replacement request TLVs.
        message_payload: Option<TLV>,
    }
}

impl ReplaceSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        message_payload: Option<AnyOctetString>,
    ) -> Self {
        let message_payload =
            message_payload.map(|value| TLV::new(TLVValue::MessagePayload(value)));
        let sm_length = short_message.length() as u8;

        let mut replace_sm = Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            sm_length,
            short_message,
            message_payload,
        };

        replace_sm.clear_short_message_if_message_payload_exists();

        replace_sm
    }

    pub fn sm_length(&self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
    }

    /// Sets the short message and short message length.
    /// Updates the short message and short message length accordingly.
    /// Has no effect if the message payload is set.
    /// Returns true if the short message and short message length were set.
    pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) -> bool {
        self.sm_length = short_message.length() as u8;
        self.short_message = short_message;

        !self.clear_short_message_if_message_payload_exists()
    }

    pub fn message_payload(&self) -> Option<&TLV> {
        self.message_payload.as_ref()
    }

    /// Sets the message payload.
    /// Updates the short message and short message length accordingly.
    pub fn set_message_payload(&mut self, message_payload: Option<AnyOctetString>) {
        self.message_payload = message_payload.map(|v| TLV::new(TLVValue::MessagePayload(v)));

        self.clear_short_message_if_message_payload_exists();
    }

    /// Clears the short message and short message length if the message payload is set.
    /// Returns true if the short message and short message length were cleared.
    fn clear_short_message_if_message_payload_exists(&mut self) -> bool {
        if self.message_payload.is_some() {
            self.short_message = OctetString::empty();
            self.sm_length = 0;

            return true;
        };

        false
    }

    pub fn builder() -> ReplaceSmBuilder {
        ReplaceSmBuilder::new()
    }

    pub fn into_replace_sm(self) -> Pdu {
        Pdu::ReplaceSm(self)
    }
}

impl DecodeWithLength for ReplaceSm {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let message_id = tri!(COctetString::decode_from(reader));
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::decode_from(reader));
        let schedule_delivery_time = tri!(EmptyOrFullCOctetString::decode_from(reader));
        let validity_period = tri!(EmptyOrFullCOctetString::decode_from(reader));
        let registered_delivery = tri!(RegisteredDelivery::decode_from(reader));
        let sm_default_msg_id = tri!(u8::decode_from(reader));
        let sm_length = tri!(u8::decode_from(reader));
        let short_message = tri!(OctetString::decode_from(reader, sm_length as usize));

        let message_payload_length = length.saturating_sub(
            message_id.length()
                + source_addr_ton.length()
                + source_addr_npi.length()
                + source_addr.length()
                + schedule_delivery_time.length()
                + validity_period.length()
                + registered_delivery.length()
                + sm_default_msg_id.length()
                + sm_length.length()
                + short_message.length(),
        );

        let message_payload = tri!(TLV::length_checked_decode_from(
            reader,
            message_payload_length
        ));

        Ok(Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            sm_length,
            short_message,
            message_payload,
        })
    }
}

#[derive(Default)]
pub struct ReplaceSmBuilder {
    inner: ReplaceSm,
}

impl ReplaceSmBuilder {
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

    pub fn schedule_delivery_time(
        mut self,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
    ) -> Self {
        self.inner.schedule_delivery_time = schedule_delivery_time;
        self
    }

    pub fn validity_period(mut self, validity_period: EmptyOrFullCOctetString<17>) -> Self {
        self.inner.validity_period = validity_period;
        self
    }

    pub fn registered_delivery(mut self, registered_delivery: RegisteredDelivery) -> Self {
        self.inner.registered_delivery = registered_delivery;
        self
    }

    pub fn sm_default_msg_id(mut self, sm_default_msg_id: u8) -> Self {
        self.inner.sm_default_msg_id = sm_default_msg_id;
        self
    }

    pub fn short_message(mut self, short_message: OctetString<0, 255>) -> Self {
        self.inner.set_short_message(short_message);
        self
    }

    pub fn message_payload(mut self, message_payload: Option<AnyOctetString>) -> Self {
        self.inner.set_message_payload(message_payload);
        self
    }

    pub fn build(self) -> ReplaceSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<ReplaceSm>();
    }
}
