use super::Pdu;
use crate::{
    commands::{
        tlvs::{
            tlv::{message_submission_request::MessageSubmissionRequestTLV, TLV},
            tlv_tag::TLVTag,
        },
        types::{
            data_coding::DataCoding, dest_address::DestAddress, esm_class::EsmClass, npi::Npi,
            priority_flag::PriorityFlag, registered_delivery::RegisteredDelivery,
            replace_if_present_flag::ReplaceIfPresentFlag, service_type::ServiceType, ton::Ton,
        },
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        length::Length,
    },
    impl_length_encode, tri,
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        octet_string::OctetString, u8::EndeU8,
    },
};

impl_length_encode! {
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct SubmitMulti {
        /// The service_type parameter can be used to indicate the
        /// SMS Application service associated with the message.
        /// Specifying the service_type allows the ESME to avail of
        /// enhanced messaging services such as “replace by
        /// service_type” or control the teleservice used on the air
        /// interface.
        ///
        /// Set to NULL for default MC settings.
        pub service_type: ServiceType,
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
        /// Number of destination addresses – indicates the
        /// number of destinations that are to follow.
        ///
        /// A maximum of 255 destination addresses are allowed.
        ///
        /// Note: Set to 1 when submitting to one SME Address or when
        /// submitting to one Distribution List.
        number_of_dests: u8,
        /// Composite field.
        dest_address: Vec<DestAddress>,
        /// Indicates Message Mode and Message Type.
        pub esm_class: EsmClass,
        /// Protocol Identifier.
        ///
        /// Network specific field.
        pub protocol_id: u8,
        /// Designates the priority level of the message.
        pub priority_flag: PriorityFlag,
        /// The short message is to be scheduled by the MC for delivery.
        ///
        /// Set to NULL for immediate message delivery.
        pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
        /// The validity period of this message.
        ///
        /// Set to NULL to request the SMSC default validity period.
        ///
        /// Note: this is superseded by the qos_time_to_live TLV if specified.
        pub validity_period: EmptyOrFullCOctetString<17>,
        /// Indicator to signify if a MC delivery receipt or an SME
        /// acknowledgement is required.
        pub registered_delivery: RegisteredDelivery,
        // Flag indicating if submitted message should replace an
        // existing message.
        pub replace_if_present_flag: ReplaceIfPresentFlag,
        /// Defines the encoding scheme of the short message user data.
        pub data_coding: DataCoding,
        /// Indicates the short message to send from a list of pre- defined
        /// (‘canned’) short messages stored on the MC.
        ///
        /// If not using a MC canned message, set to NULL.
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
        ///
        /// Applications which need to send messages longer than
        /// 255 octets should use the message_payload TLV. In
        /// this case the sm_length field should be set to zero.
        short_message: OctetString<0, 255>,
        /// Message submission request TLVs ([`MessageSubmissionRequestTLV`]).
        tlvs: Vec<TLV>,
    }
}

impl SubmitMulti {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_address: Vec<DestAddress>,
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
        tlvs: Vec<impl Into<MessageSubmissionRequestTLV>>,
    ) -> Self {
        let sm_length = short_message.length() as u8;
        let number_of_dests = dest_address.len() as u8;

        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<TLV>>();

        let mut submit_multi = Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            number_of_dests,
            dest_address,
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

        submit_multi.clear_short_message_if_message_payload_exists();

        submit_multi
    }

    pub fn number_of_dests(&self) -> u8 {
        self.number_of_dests
    }

    pub fn dest_address(&self) -> &[DestAddress] {
        &self.dest_address
    }

    pub fn set_dest_address(&mut self, dest_address: Vec<DestAddress>) {
        self.number_of_dests = dest_address.len() as u8;
        self.dest_address = dest_address;
    }

    pub fn push_dest_address(&mut self, dest_address: DestAddress) {
        self.number_of_dests += 1;
        self.dest_address.push(dest_address);
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

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<MessageSubmissionRequestTLV>>) {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<TLV>>();

        self.tlvs = tlvs;
        self.clear_short_message_if_message_payload_exists();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionRequestTLV>) {
        let tlv: MessageSubmissionRequestTLV = tlv.into();
        let tlv: TLV = tlv.into();

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

    pub fn builder() -> SubmitMultiBuilder {
        SubmitMultiBuilder::new()
    }
}

impl From<SubmitMulti> for Pdu {
    fn from(value: SubmitMulti) -> Self {
        Self::SubmitMulti(value)
    }
}

impl DecodeWithLength for SubmitMulti {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let service_type = tri!(ServiceType::decode_from(reader));
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::decode_from(reader));
        let number_of_dests = tri!(u8::decode_from(reader));
        let dest_address = tri!(DestAddress::vectorized_decode_from(
            reader,
            number_of_dests as usize
        ));
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

        let tlvs_length = length
            .saturating_sub(service_type.length())
            .saturating_sub(source_addr_ton.length())
            .saturating_sub(source_addr_npi.length())
            .saturating_sub(source_addr.length())
            .saturating_sub(number_of_dests.length())
            .saturating_sub(dest_address.length())
            .saturating_sub(esm_class.length())
            .saturating_sub(protocol_id.length())
            .saturating_sub(priority_flag.length())
            .saturating_sub(schedule_delivery_time.length())
            .saturating_sub(validity_period.length())
            .saturating_sub(registered_delivery.length())
            .saturating_sub(replace_if_present_flag.length())
            .saturating_sub(data_coding.length())
            .saturating_sub(sm_default_msg_id.length())
            .saturating_sub(sm_length.length())
            .saturating_sub(short_message.length());

        let tlvs = tri!(Vec::<TLV>::decode_from(reader, tlvs_length));

        Ok(Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            number_of_dests,
            dest_address,
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

#[derive(Debug, Default)]
pub struct SubmitMultiBuilder {
    inner: SubmitMulti,
}

impl SubmitMultiBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn service_type(mut self, service_type: ServiceType) -> Self {
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

    pub fn source_addr(mut self, source_addr: COctetString<1, 21>) -> Self {
        self.inner.source_addr = source_addr;
        self
    }

    pub fn number_of_dests(mut self, number_of_dests: u8) -> Self {
        self.inner.number_of_dests = number_of_dests;
        self
    }

    pub fn dest_address(mut self, dest_address: Vec<DestAddress>) -> Self {
        self.inner.set_dest_address(dest_address);
        self
    }

    pub fn push_dest_address(mut self, dest_address: DestAddress) -> Self {
        self.inner.push_dest_address(dest_address);
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

    pub fn tlvs(mut self, tlvs: Vec<impl Into<MessageSubmissionRequestTLV>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<MessageSubmissionRequestTLV>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> SubmitMulti {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<SubmitMulti>();
    }
}
