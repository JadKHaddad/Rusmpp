use super::Pdu;
use crate::{
    commands::{
        tlvs::{
            tlv::{broadcast_request::BroadcastRequestTlv, Tlv},
            tlv_value::TlvValue,
        },
        types::{
            broadcast_area_identifier::BroadcastAreaIdentifier,
            broadcast_content_type::BroadcastContentType,
            broadcast_frequency_interval::BroadcastFrequencyInterval, data_coding::DataCoding,
            npi::Npi, priority_flag::PriorityFlag, replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::ServiceType, ton::Ton,
        },
    },
    types::{COctetString, EmptyOrFullCOctetString},
};

crate::create! {
    /// This operation is issued by the ESME to submit a message to the Message Centre for
    /// broadcast to a specified geographical area or set of geographical areas.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct BroadcastSm {
        /// The service_type parameter can be used to
        /// indicate the SMS Application service
        /// associated with the message. Specifying the
        /// service_type allows the ESME to avail of enhanced
        /// messaging services such as “replace by
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
        /// If using broadcast_sm to replace a message,
        /// previously submitted for broadcast, then set
        /// message_id to the MC assigned message ID
        /// allocated to the original message and returned in
        /// the broadcast_sm_resp  (to the original broadcast_sm request).
        ///
        /// Note: For "broadcast replace", either the message_id or the
        /// user_message_reference field should be used. Both
        /// fields must not be used simultaneously.
        /// Set to NULL:
        ///
        /// * if not using MC message ID in broadcast_sm to
        /// replace a message, previously submitted for broadcast.
        /// * if setting user_message_reference TLV.
        pub message_id: COctetString<1, 65>,
        /// Designates the propriety level of the message.
        pub priority_flag: PriorityFlag,
        /// The short message is to be scheduled by the MC for
        /// delivery.
        ///
        /// Set to NULL for immediate message broadcast.
        pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
        /// The validity period of this message.
        ///
        /// Set to NULL to specify that a ‘broadcast_rep_num’
        /// parameter and a ‘broadcast_frequency_interval’
        /// parameter have been specified from which a
        /// default value should be derived.
        pub validity_period: EmptyOrFullCOctetString<17>,
        /// Flag indicating if the submitted message should
        /// replace an existing message which has:
        /// * (1) MC message ID matching the ID supplied in
        /// the message_id field.
        /// * (2) or ESME assigned message reference number
        /// supplied in the user_message_reference field.
        pub replace_if_present_flag: ReplaceIfPresentFlag,
        /// Defines the encoding scheme of the short
        /// message user data.
        pub data_coding: DataCoding,
        /// Indicates the short message to send from a list of pre-
        /// defined (‘canned’) short messages stored on the MC.
        ///
        /// If not using a MC canned message, set to NULL.
        pub sm_default_msg_id: u8,
        /// [`TLVValue::BroadcastAreaIdentifier`].
        ///
        /// Identifies the target Broadcast Area(s) for the
        /// requested message broadcast.
        ///
        /// This parameter can be included a number of times
        /// for multiple target Broadcast Areas(s).
        broadcast_area_identifier: Tlv,
        /// [`TLVValue::BroadcastContentType`].
        ///
        /// Specifies the content type of the message.
        broadcast_content_type: Tlv,
        /// [`TLVValue::BroadcastRepNum`].
        ///
        /// This field indicates the number of repeated
        /// broadcasts of a message requested by the submitter.
        broadcast_rep_num: Tlv,
        /// [`TLVValue::BroadcastFrequencyInterval`].
        ///
        /// This field indicates the frequency interval at which
        /// the broadcasts of a message should be repeated.
        broadcast_frequency_interval: Tlv,
        /// Broadcast request TLVs ([`BroadcastRequestTLV`]).
        @[length = unchecked]
        tlvs: Vec<Tlv>,
    }
}

// TODO: add the downcast for these tlvs
impl BroadcastSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        message_id: COctetString<1, 65>,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        broadcast_area_identifier: BroadcastAreaIdentifier,
        broadcast_content_type: BroadcastContentType,
        broadcast_rep_num: u16,
        broadcast_frequency_interval: BroadcastFrequencyInterval,
        tlvs: Vec<impl Into<BroadcastRequestTlv>>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        let broadcast_area_identifier =
            Tlv::new(TlvValue::BroadcastAreaIdentifier(broadcast_area_identifier));

        let broadcast_content_type =
            Tlv::new(TlvValue::BroadcastContentType(broadcast_content_type));

        let broadcast_rep_num = Tlv::new(TlvValue::BroadcastRepNum(broadcast_rep_num));

        let broadcast_frequency_interval = Tlv::new(TlvValue::BroadcastFrequencyInterval(
            broadcast_frequency_interval,
        ));

        Self {
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            message_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            broadcast_area_identifier,
            broadcast_content_type,
            broadcast_rep_num,
            broadcast_frequency_interval,
            tlvs,
        }
    }

    pub fn broadcast_area_identifier(&self) -> &Tlv {
        &self.broadcast_area_identifier
    }

    pub fn set_broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) {
        self.broadcast_area_identifier =
            Tlv::new(TlvValue::BroadcastAreaIdentifier(broadcast_area_identifier));
    }

    pub fn broadcast_content_type(&self) -> &Tlv {
        &self.broadcast_content_type
    }

    pub fn set_broadcast_content_type(&mut self, broadcast_content_type: BroadcastContentType) {
        self.broadcast_content_type =
            Tlv::new(TlvValue::BroadcastContentType(broadcast_content_type));
    }

    pub fn broadcast_rep_num(&self) -> &Tlv {
        &self.broadcast_rep_num
    }

    pub fn set_broadcast_rep_num(&mut self, broadcast_rep_num: u16) {
        self.broadcast_rep_num = Tlv::new(TlvValue::BroadcastRepNum(broadcast_rep_num));
    }

    pub fn broadcast_frequency_interval(&self) -> &Tlv {
        &self.broadcast_frequency_interval
    }

    pub fn set_broadcast_frequency_interval(
        &mut self,
        broadcast_frequency_interval: BroadcastFrequencyInterval,
    ) {
        self.broadcast_frequency_interval = Tlv::new(TlvValue::BroadcastFrequencyInterval(
            broadcast_frequency_interval,
        ));
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<BroadcastRequestTlv>>) {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        self.tlvs = tlvs;
    }

    pub fn push_tlv(&mut self, tlv: impl Into<BroadcastRequestTlv>) {
        let tlv: BroadcastRequestTlv = tlv.into();
        let tlv: Tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> BroadcastSmBuilder {
        BroadcastSmBuilder::new()
    }
}

impl From<BroadcastSm> for Pdu {
    fn from(value: BroadcastSm) -> Self {
        Self::BroadcastSm(value)
    }
}

impl Default for BroadcastSm {
    fn default() -> Self {
        Self {
            service_type: Default::default(),
            source_addr_ton: Default::default(),
            source_addr_npi: Default::default(),
            source_addr: Default::default(),
            message_id: Default::default(),
            priority_flag: Default::default(),
            schedule_delivery_time: Default::default(),
            validity_period: Default::default(),
            replace_if_present_flag: Default::default(),
            data_coding: Default::default(),
            sm_default_msg_id: Default::default(),
            broadcast_area_identifier: Tlv::new(TlvValue::BroadcastAreaIdentifier(
                Default::default(),
            )),
            broadcast_content_type: Tlv::new(TlvValue::BroadcastContentType(Default::default())),
            broadcast_rep_num: Tlv::new(TlvValue::BroadcastRepNum(Default::default())),
            broadcast_frequency_interval: Tlv::new(TlvValue::BroadcastFrequencyInterval(
                Default::default(),
            )),
            tlvs: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct BroadcastSmBuilder {
    inner: BroadcastSm,
}

impl BroadcastSmBuilder {
    pub fn new() -> Self {
        Self::default()
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

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
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

    pub fn broadcast_area_identifier(
        mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) -> Self {
        self.inner
            .set_broadcast_area_identifier(broadcast_area_identifier);
        self
    }

    pub fn broadcast_content_type(mut self, broadcast_content_type: BroadcastContentType) -> Self {
        self.inner
            .set_broadcast_content_type(broadcast_content_type);
        self
    }

    pub fn broadcast_rep_num(mut self, broadcast_rep_num: u16) -> Self {
        self.inner.set_broadcast_rep_num(broadcast_rep_num);
        self
    }

    pub fn broadcast_frequency_interval(
        mut self,
        broadcast_frequency_interval: BroadcastFrequencyInterval,
    ) -> Self {
        self.inner
            .set_broadcast_frequency_interval(broadcast_frequency_interval);
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<impl Into<BroadcastRequestTlv>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<BroadcastRequestTlv>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> BroadcastSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<BroadcastSm>();
    }
}
