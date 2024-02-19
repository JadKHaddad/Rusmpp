use super::Pdu;
use crate::{
    commands::{
        tlvs::{
            tlv::{message_delivery_request::MessageDeliveryRequestTLV, TLV},
            tlv_tag::TLVTag,
        },
        types::{
            data_coding::DataCoding, esm_class::EsmClass, npi::Npi, priority_flag::PriorityFlag,
            registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::ServiceType, ton::Ton,
        },
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        octet_string::OctetString, u8::EndeU8,
    },
};

/// This operation is used by an ESME to submit a short message to the MC for onward
/// transmission to a specified short message entity (SME).
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeliverSm {
    /// The service_type parameter can be used to
    /// indicate the SMS Application service
    /// associated with the message. Specifying the
    /// service_type allows the ESME to avail of enhanced
    /// messaging services such as “replace by service_type”
    /// or to control the teleservice used on the
    /// air interface.  
    ///
    /// Set to NULL for default MC settings.
    pub serivce_type: ServiceType,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// Address of SME which originated this message.
    pub source_addr: COctetString<1, 21>,
    /// Type of Number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination address of this
    /// short message For mobile
    /// terminated messages, this
    /// is the directory number of
    /// the recipient MS
    pub destination_addr: COctetString<1, 21>,
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
    pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
    /// The validity period of this message.  
    /// Set to NULL to request the MC default validity period.
    ///
    /// Note: this is superseded by the qos_time_to_live TLV if
    /// specified.
    pub validity_period: EmptyOrFullCOctetString<17>,
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
    short_message: OctetString<0, 255>,
    /// Message delivery request TLVs ([`MessageDeliveryRequestTLV`])
    tlvs: Vec<TLV>,
}

impl DeliverSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
        esm_class: EsmClass,
        protocol_id: u8,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        tlvs: Vec<MessageDeliveryRequestTLV>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        let sm_length = short_message.length() as u8;

        let mut submit_sm = Self {
            serivce_type,
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

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageDeliveryRequestTLV>) {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        self.tlvs = tlvs;
        self.clear_short_message_if_message_payload_exists();
    }

    pub fn push_tlv(&mut self, tlv: MessageDeliveryRequestTLV) {
        let tlv = tlv.into();

        self.tlvs.push(tlv);
        self.clear_short_message_if_message_payload_exists();
    }

    /// Clears the short message and short message length if the message payload is set.
    /// Returns true if the short message and short message length were cleared.
    fn clear_short_message_if_message_payload_exists(&mut self) -> bool {
        let message_payload_exists = self
            .tlvs
            .iter()
            .any(|value| matches!(value.tag(), TLVTag::MessagePayload));

        if message_payload_exists {
            self.short_message = OctetString::empty();
            self.sm_length = 0;

            return true;
        };

        false
    }

    pub fn builder() -> DeliverSmBuilder {
        DeliverSmBuilder::new()
    }

    pub fn into_deliver_sm(self) -> Pdu {
        Pdu::DeliverSm(self)
    }
}

impl Length for DeliverSm {
    fn length(&self) -> usize {
        self.serivce_type.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.dest_addr_ton.length()
            + self.dest_addr_npi.length()
            + self.destination_addr.length()
            + self.esm_class.length()
            + self.protocol_id.length()
            + self.priority_flag.length()
            + self.schedule_delivery_time.length()
            + self.validity_period.length()
            + self.registered_delivery.length()
            + self.replace_if_present_flag.length()
            + self.data_coding.length()
            + self.sm_default_msg_id.length()
            + self.sm_length.length()
            + self.tlvs.length()
    }
}

impl Encode for DeliverSm {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.serivce_type.encode_to(writer));
        tri!(self.source_addr_ton.encode_to(writer));
        tri!(self.source_addr_npi.encode_to(writer));
        tri!(self.source_addr.encode_to(writer));
        tri!(self.dest_addr_ton.encode_to(writer));
        tri!(self.dest_addr_npi.encode_to(writer));
        tri!(self.destination_addr.encode_to(writer));
        tri!(self.esm_class.encode_to(writer));
        tri!(self.protocol_id.encode_to(writer));
        tri!(self.priority_flag.encode_to(writer));
        tri!(self.schedule_delivery_time.encode_to(writer));
        tri!(self.validity_period.encode_to(writer));
        tri!(self.registered_delivery.encode_to(writer));
        tri!(self.replace_if_present_flag.encode_to(writer));
        tri!(self.data_coding.encode_to(writer));
        tri!(self.sm_default_msg_id.encode_to(writer));
        tri!(self.sm_length.encode_to(writer));
        tri!(self.short_message.encode_to(writer));
        tri!(self.tlvs.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for DeliverSm {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let serivce_type = tri!(ServiceType::decode_from(reader));
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::decode_from(reader));
        let dest_addr_ton = tri!(Ton::decode_from(reader));
        let dest_addr_npi = tri!(Npi::decode_from(reader));
        let destination_addr = tri!(COctetString::decode_from(reader));
        let esm_class = tri!(EsmClass::decode_from(reader));
        let protocol_id = tri!(u8::decode_from(reader));
        let priority_flag = tri!(PriorityFlag::decode_from(reader));
        let schedule_delivery_time = tri!(EmptyOrFullCOctetString::decode_from(reader));
        let validity_period = tri!(EmptyOrFullCOctetString::decode_from(reader));
        let registered_delivery = tri!(RegisteredDelivery::decode_from(reader));
        let replace_if_present_flag = tri!(ReplaceIfPresentFlag::decode_from(reader));
        let data_coding = tri!(DataCoding::decode_from(reader));
        let sm_default_msg_id = tri!(u8::decode_from(reader));
        let sm_length = tri!(u8::decode_from(reader));
        let short_message = tri!(OctetString::decode_from(reader, sm_length as usize));

        let tlvs_length = length.saturating_sub(
            serivce_type.length()
                + source_addr_ton.length()
                + source_addr_npi.length()
                + source_addr.length()
                + dest_addr_ton.length()
                + dest_addr_npi.length()
                + destination_addr.length()
                + esm_class.length()
                + protocol_id.length()
                + priority_flag.length()
                + schedule_delivery_time.length()
                + validity_period.length()
                + registered_delivery.length()
                + replace_if_present_flag.length()
                + data_coding.length()
                + sm_default_msg_id.length()
                + sm_length.length()
                + short_message.length(),
        );

        let tlvs = tri!(Vec::<TLV>::decode_from(reader, tlvs_length));

        Ok(Self {
            serivce_type,
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
        })
    }
}

#[derive(Default)]
pub struct DeliverSmBuilder {
    inner: DeliverSm,
}

impl DeliverSmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serivce_type(mut self, serivce_type: ServiceType) -> Self {
        self.inner.serivce_type = serivce_type;
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

    pub fn short_message(mut self, short_message: OctetString<0, 255>) -> Self {
        self.inner.set_short_message(short_message);
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<MessageDeliveryRequestTLV>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: MessageDeliveryRequestTLV) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> DeliverSm {
        self.inner
    }
}
