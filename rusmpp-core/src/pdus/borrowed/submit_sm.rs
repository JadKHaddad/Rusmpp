use rusmpp_macros::Rusmpp;

use crate::{
    tlvs::borrowed::Tlv,
    types::borrowed::{COctetString, EmptyOrFullCOctetString, OctetString},
    values::{
        data_coding::DataCoding, esm_class::EsmClass, npi::Npi, priority_flag::PriorityFlag,
        registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
        service_type::borrowed::ServiceType, ton::Ton,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed,test = skip)]
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
