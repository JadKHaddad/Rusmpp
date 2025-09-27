use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    pdus::borrowed::Pdu,
    tlvs::{
        TlvTag,
        borrowed::{MessageSubmissionRequestTlvValue, Tlv},
    },
    types::borrowed::{COctetString, EmptyOrFullCOctetString, OctetString},
    values::{
        data_coding::DataCoding, esm_class::EsmClass, npi::Npi, priority_flag::PriorityFlag,
        registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
        service_type::borrowed::ServiceType, ton::Ton,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct SubmitSm<'a, const N: usize> {
    /// The service_type parameter can be used to
    /// indicate the SMS Application service
    /// associated with the message. Specifying the
    /// service_type allows the ESME to avail of enhanced
    /// messaging services such as “replace by service_type”
    /// or to control the teleservice used on the
    /// air interface.
    ///
    /// Set to NULL for default MC settings.
    pub service_type: ServiceType<'a>,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// Address of SME which originated this message.
    pub source_addr: COctetString<'a, 1, 21>,
    /// Type of Number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination address of this
    /// short message For mobile
    /// terminated messages, this
    /// is the directory number of
    /// the recipient MS
    pub destination_addr: COctetString<'a, 1, 21>,
    /// Indicates Message Mode
    /// and Message Type.
    pub esm_class: EsmClass,
    /// Protocol Identifier.
    /// Network specific field.
    pub protocol_id: u8,
    /// Designates the priority level of the message.
    pub priority_flag: PriorityFlag,
    /// The short message is to be
    /// scheduled by the MC for delivery.
    /// Set to NULL for immediate message delivery.
    pub schedule_delivery_time: EmptyOrFullCOctetString<'a, 17>,
    /// The validity period of this message.
    /// Set to NULL to request the MC default validity period.
    ///
    /// Note: this is superseded by the qos_time_to_live TLV if
    /// specified.
    pub validity_period: EmptyOrFullCOctetString<'a, 17>,
    /// Indicator to signify if a MC delivery receipt, manual
    /// ACK, delivery ACK or an intermediate notification is required.
    pub registered_delivery: RegisteredDelivery,
    /// Flag indicating if the submitted message should replace an existing message.
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    // Defines the encoding scheme of the short message user data.
    pub data_coding: DataCoding,
    /// Indicates the short message to send from a list of pre- defined (‘canned’)
    /// short messages stored on the MC. If not using a MC canned message, set to NULL.
    pub sm_default_msg_id: u8,
    /// Length in octets of the short_message user data.
    sm_length: u8,
    /// Up to 255 octets of short message user data.
    ///
    /// The exact physical limit for short_message size may
    /// vary according to the underlying network
    ///
    /// Note: this field is superceded by the message_payload TLV if
    /// specified.
    #[rusmpp(length = sm_length)]
    short_message: OctetString<'a, 0, 255>,
    /// Message submission request TLVs ([`MessageSubmissionRequestTlvValue`]).
    #[rusmpp(length = "unchecked")]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> SubmitSm<'a, N> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType<'a>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<'a, 1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<'a, 1, 21>,
        esm_class: EsmClass,
        protocol_id: u8,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<'a, 17>,
        validity_period: EmptyOrFullCOctetString<'a, 17>,
        registered_delivery: RegisteredDelivery,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        short_message: OctetString<'a, 0, 255>,
        tlvs: heapless::vec::Vec<impl Into<MessageSubmissionRequestTlvValue<'a>>, N>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        let sm_length = short_message.length() as u8;

        let mut submit_sm = Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            protocol_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            sm_length,
            short_message,
            tlvs,
        };

        submit_sm.clear_short_message_if_message_payload_exists();

        submit_sm
    }

    pub fn sm_length(&self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&'_ self) -> &'_ OctetString<'_, 0, 255> {
        &self.short_message
    }

    /// Sets the short message and short message length.
    /// Updates the short message and short message length accordingly.
    /// Has no effect if the message payload is set.
    /// Returns true if the short message and short message length were set.
    pub fn set_short_message(&mut self, short_message: OctetString<'a, 0, 255>) -> bool {
        self.short_message = short_message;
        self.sm_length = self.short_message.length() as u8;

        !self.clear_short_message_if_message_payload_exists()
    }

    pub fn tlvs(&'_ self) -> &'_ [Tlv<'_>] {
        &self.tlvs
    }

    pub fn set_tlvs(
        &mut self,
        tlvs: heapless::vec::Vec<impl Into<MessageSubmissionRequestTlvValue<'a>>, N>,
    ) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        self.clear_short_message_if_message_payload_exists();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(
        &mut self,
        tlv: impl Into<MessageSubmissionRequestTlvValue<'a>>,
    ) -> Result<(), Tlv<'a>> {
        self.tlvs.push(Tlv::from(tlv.into()))?;

        self.clear_short_message_if_message_payload_exists();

        Ok(())
    }

    /// Clears the short message and short message length if the message payload is set.
    /// Returns true if the short message and short message length were cleared.
    fn clear_short_message_if_message_payload_exists(&mut self) -> bool {
        let message_payload_exists = self
            .tlvs
            .iter()
            .any(|value| matches!(value.tag(), TlvTag::MessagePayload));

        if message_payload_exists {
            self.short_message = OctetString::empty();
            self.sm_length = 0;

            return true;
        };

        false
    }

    pub fn builder() -> SubmitSmBuilder<'a, N> {
        SubmitSmBuilder::new()
    }
}

impl<'a, const N: usize> From<SubmitSm<'a, N>> for Pdu<'a, N> {
    fn from(value: SubmitSm<'a, N>) -> Self {
        Self::SubmitSm(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitSmBuilder<'a, const N: usize> {
    inner: SubmitSm<'a, N>,
}

impl<'a, const N: usize> SubmitSmBuilder<'a, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn service_type(mut self, service_type: ServiceType<'a>) -> Self {
        self.inner.service_type = service_type;
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

    pub fn dest_addr_ton(mut self, dest_addr_ton: Ton) -> Self {
        self.inner.dest_addr_ton = dest_addr_ton;
        self
    }

    pub fn dest_addr_npi(mut self, dest_addr_npi: Npi) -> Self {
        self.inner.dest_addr_npi = dest_addr_npi;
        self
    }

    pub fn destination_addr(mut self, destination_addr: COctetString<'a, 1, 21>) -> Self {
        self.inner.destination_addr = destination_addr;
        self
    }

    pub fn esm_class(mut self, esm_class: EsmClass) -> Self {
        self.inner.esm_class = esm_class;
        self
    }

    pub fn protocol_id(mut self, protocol_id: u8) -> Self {
        self.inner.protocol_id = protocol_id;
        self
    }

    pub fn priority_flag(mut self, priority_flag: PriorityFlag) -> Self {
        self.inner.priority_flag = priority_flag;
        self
    }

    pub fn schedule_delivery_time(
        mut self,
        schedule_delivery_time: EmptyOrFullCOctetString<'a, 17>,
    ) -> Self {
        self.inner.schedule_delivery_time = schedule_delivery_time;
        self
    }

    pub fn validity_period(mut self, validity_period: EmptyOrFullCOctetString<'a, 17>) -> Self {
        self.inner.validity_period = validity_period;
        self
    }

    pub fn registered_delivery(mut self, registered_delivery: RegisteredDelivery) -> Self {
        self.inner.registered_delivery = registered_delivery;
        self
    }

    pub fn replace_if_present_flag(
        mut self,
        replace_if_present_flag: ReplaceIfPresentFlag,
    ) -> Self {
        self.inner.replace_if_present_flag = replace_if_present_flag;
        self
    }

    pub fn data_coding(mut self, data_coding: DataCoding) -> Self {
        self.inner.data_coding = data_coding;
        self
    }

    pub fn sm_default_msg_id(mut self, sm_default_msg_id: u8) -> Self {
        self.inner.sm_default_msg_id = sm_default_msg_id;
        self
    }

    pub fn short_message(mut self, short_message: OctetString<'a, 0, 255>) -> Self {
        self.inner.set_short_message(short_message);
        self
    }

    pub fn tlvs(
        mut self,
        tlvs: heapless::vec::Vec<impl Into<MessageSubmissionRequestTlvValue<'a>>, N>,
    ) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(
        mut self,
        tlv: impl Into<MessageSubmissionRequestTlvValue<'a>>,
    ) -> Result<Self, Tlv<'a>> {
        self.inner.push_tlv(tlv)?;
        Ok(self)
    }

    pub fn build(self) -> SubmitSm<'a, N> {
        self.inner
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        tests::TestInstance,
        types::borrowed::AnyOctetString,
        values::{
            esm_class::{Ansi41Specific, GsmFeatures, MessageType, MessagingMode},
            message_payload::borrowed::MessagePayload,
            priority_flag::{Ansi136, PriorityFlagType},
            service_type::GenericServiceType,
        },
    };

    use super::*;

    impl<const N: usize> TestInstance for SubmitSm<'static, N> {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::builder()
                    .service_type(ServiceType::new(
                        GenericServiceType::CellularMessaging.into(),
                    ))
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::new(b"Source Address\0").unwrap())
                    .dest_addr_ton(Ton::International)
                    .dest_addr_npi(Npi::Isdn)
                    .destination_addr(COctetString::new(b"Destination Address\0").unwrap())
                    .esm_class(EsmClass::new(
                        MessagingMode::StoreAndForward,
                        MessageType::ShortMessageContainsMCDeliveryReceipt,
                        Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement,
                        GsmFeatures::SetUdhiAndReplyPath,
                    ))
                    .protocol_id(0)
                    .priority_flag(PriorityFlag::from(PriorityFlagType::from(Ansi136::Bulk)))
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::new(b"2023-09-01T12:00\0").unwrap(),
                    )
                    .validity_period(EmptyOrFullCOctetString::new(b"2023-10-01T12:00\0").unwrap())
                    .registered_delivery(RegisteredDelivery::request_all())
                    .replace_if_present_flag(ReplaceIfPresentFlag::Replace)
                    .data_coding(DataCoding::Ksc5601)
                    .sm_default_msg_id(69)
                    .short_message(OctetString::new(b"Short Message").unwrap())
                    .build(),
                Self::builder()
                    .service_type(ServiceType::new(
                        GenericServiceType::CellularMessaging.into(),
                    ))
                    .source_addr_ton(Ton::International)
                    .source_addr_npi(Npi::Isdn)
                    .source_addr(COctetString::new(b"Source Address\0").unwrap())
                    .dest_addr_ton(Ton::International)
                    .dest_addr_npi(Npi::Isdn)
                    .destination_addr(COctetString::new(b"Destination Address\0").unwrap())
                    .esm_class(EsmClass::new(
                        MessagingMode::Default,
                        MessageType::ShortMessageContainsIntermediateDeliveryNotification,
                        Ansi41Specific::ShortMessageContainsUserAcknowledgment,
                        GsmFeatures::SetUdhiAndReplyPath,
                    ))
                    .protocol_id(0)
                    .priority_flag(PriorityFlag::from(PriorityFlagType::from(
                        Ansi136::VeryUrgent,
                    )))
                    .schedule_delivery_time(
                        EmptyOrFullCOctetString::new(b"2023-09-01T12:01\0").unwrap(),
                    )
                    .validity_period(EmptyOrFullCOctetString::new(b"2023-10-01T12:20\0").unwrap())
                    .registered_delivery(RegisteredDelivery::request_all())
                    .replace_if_present_flag(ReplaceIfPresentFlag::DoNotReplace)
                    .data_coding(DataCoding::Jis)
                    .sm_default_msg_id(96)
                    .short_message(OctetString::new(b"Short Message").unwrap())
                    .tlvs(
                        [MessageSubmissionRequestTlvValue::MessagePayload(
                            MessagePayload::new(AnyOctetString::new(b"Message Payload")),
                        )]
                        .into()
                    )
                    .build(),
                Self::builder()
                    .short_message(OctetString::new(b"Short Message").unwrap())
                    .tlvs(
                        [
                            MessageSubmissionRequestTlvValue::MessagePayload(MessagePayload::new(
                                AnyOctetString::new(b"Message Payload"),
                            )),
                            // TODO: restore after implementing more TLVs
                            // MessageSubmissionRequestTlvValue::UserResponseCode(3),
                            // MessageSubmissionRequestTlvValue::DestBearerType(BearerType::FlexReFlex),
                            // MessageSubmissionRequestTlvValue::SourceSubaddress(Subaddress::new(
                            //     SubaddressTag::NsapOdd,
                            //     OctetString::from_str("Subaddress :D").unwrap(),
                            // )),
                        ]
                        .into()
                    )
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_with_length_test_instances::<SubmitSm<'static, 16>>();
    }

    #[test]
    fn short_message_length() {
        let short_message = OctetString::new(b"Short Message").unwrap();

        let submit_sm = SubmitSm::<'static, 16>::builder()
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message);
        assert_eq!(submit_sm.sm_length(), short_message.length() as u8);
    }

    #[test]
    fn short_message_override() {
        let short_message_1 = OctetString::new(b"Short Message 101").unwrap();
        let short_message_2 = OctetString::new(b"Short Message 2").unwrap();

        let submit_sm = SubmitSm::<'static, 16>::builder()
            .short_message(short_message_1)
            .short_message(short_message_2.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message_2);
        assert_eq!(submit_sm.sm_length(), short_message_2.length() as u8);
    }

    #[test]
    fn message_payload_suppresses_short_message() {
        let short_message = OctetString::new(b"Short Message").unwrap();
        let message_payload = MessagePayload::new(AnyOctetString::new(b"Message Payload"));

        // Using push_tlv
        let submit_sm = SubmitSm::<'static, 16>::builder()
            .short_message(short_message.clone())
            .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                message_payload.clone(),
            ))
            .expect("Failed to push TLV")
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Using tlvs
        let submit_sm = SubmitSm::<'static, 16>::builder()
            .short_message(short_message.clone())
            .tlvs(
                [MessageSubmissionRequestTlvValue::MessagePayload(
                    message_payload.clone(),
                )]
                .into(),
            )
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Even setting the short message after the message payload should not set the short message
        // Using push_tlv
        let submit_sm = SubmitSm::<'static, 16>::builder()
            .short_message(short_message.clone())
            .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                message_payload.clone(),
            ))
            .expect("Failed to push TLV")
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Using tlvs
        let submit_sm = SubmitSm::<'static, 16>::builder()
            .short_message(short_message.clone())
            .tlvs(
                [MessageSubmissionRequestTlvValue::MessagePayload(
                    message_payload.clone(),
                )]
                .into(),
            )
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &OctetString::empty());
        assert_eq!(submit_sm.sm_length(), 0);

        // Removing the message payload and then setting the short message should set the short message
        let submit_sm = SubmitSm::<'static, 16>::builder()
            .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                message_payload.clone(),
            ))
            .expect("Failed to push TLV")
            .clear_tlvs()
            .short_message(short_message.clone())
            .build();

        assert_eq!(submit_sm.short_message(), &short_message);
        assert_eq!(submit_sm.sm_length(), short_message.length() as u8);
    }
}
