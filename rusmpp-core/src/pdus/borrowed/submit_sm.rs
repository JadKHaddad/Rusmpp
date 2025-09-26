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

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// #[rusmpp(decode = skip, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
// TODO: fix the generics in rusmpp-macros
pub struct SubmitSm<'a, const N: usize = 6> {
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
    // #[rusmpp(length = sm_length)]
    short_message: OctetString<'a, 0, 255>,
    // /// Message submission request TLVs ([`MessageSubmissionRequestTlvValue`]).
    // #[rusmpp(length = "unchecked")]
    tlvs: heapless::vec::Vec<Tlv<'a>, N>,
}

impl<'a, const N: usize> crate::encode::Length for SubmitSm<'a, N> {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.service_type);
        length += crate::encode::Length::length(&self.source_addr_ton);
        length += crate::encode::Length::length(&self.source_addr_npi);
        length += crate::encode::Length::length(&self.source_addr);
        length += crate::encode::Length::length(&self.dest_addr_ton);
        length += crate::encode::Length::length(&self.dest_addr_npi);
        length += crate::encode::Length::length(&self.destination_addr);
        length += crate::encode::Length::length(&self.esm_class);
        length += crate::encode::Length::length(&self.protocol_id);
        length += crate::encode::Length::length(&self.priority_flag);
        length += crate::encode::Length::length(&self.schedule_delivery_time);
        length += crate::encode::Length::length(&self.validity_period);
        length += crate::encode::Length::length(&self.registered_delivery);
        length += crate::encode::Length::length(&self.replace_if_present_flag);
        length += crate::encode::Length::length(&self.data_coding);
        length += crate::encode::Length::length(&self.sm_default_msg_id);
        length += crate::encode::Length::length(&self.sm_length);
        length += crate::encode::Length::length(&self.short_message);
        length += crate::encode::Length::length(&self.tlvs);
        length
    }
}
impl<'a, const N: usize> crate::encode::Encode for SubmitSm<'a, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.service_type, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.source_addr_ton, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.source_addr_npi, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.source_addr, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.dest_addr_ton, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.dest_addr_npi, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.destination_addr, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.esm_class, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.protocol_id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.priority_flag, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.schedule_delivery_time, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.validity_period, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.registered_delivery, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.replace_if_present_flag, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.data_coding, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.sm_default_msg_id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.sm_length, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.short_message, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.tlvs, dst, size);
        size
    }
}

impl<'a, const N: usize> crate::decode::borrowed::DecodeWithLength<'a> for SubmitSm<'a, N> {
    fn decode(src: &'a [u8], length: usize) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (service_type, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::service_type,
        )?;
        let (source_addr_ton, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::source_addr_ton,
        )?;
        let (source_addr_npi, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::source_addr_npi,
        )?;
        let (source_addr, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::source_addr,
        )?;
        let (dest_addr_ton, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::dest_addr_ton,
        )?;
        let (dest_addr_npi, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::dest_addr_npi,
        )?;
        let (destination_addr, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::destination_addr,
        )?;
        let (esm_class, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::esm_class,
        )?;
        let (protocol_id, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::protocol_id,
        )?;
        let (priority_flag, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::priority_flag,
        )?;
        let (schedule_delivery_time, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::schedule_delivery_time,
        )?;
        let (validity_period, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::validity_period,
        )?;
        let (registered_delivery, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::registered_delivery,
        )?;
        let (replace_if_present_flag, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::replace_if_present_flag,
        )?;
        let (data_coding, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::data_coding,
        )?;
        let (sm_default_msg_id, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::sm_default_msg_id,
        )?;
        let (sm_length, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::sm_length,
        )?;
        let (short_message, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeWithLengthExt::decode_move(
                src,
                sm_length as usize,
                size,
            ),
            crate::fields::SmppField::short_message,
        )?;
        let (tlvs, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::borrowed::DecodeWithLengthExt::decode_move(
                src,
                length.saturating_sub(size),
                size,
            ),
            crate::fields::SmppField::tlvs,
        )?;
        Ok((
            Self {
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
            },
            size,
        ))
    }
}
