use crate::{
    commands::{
        tlvs::tlv::{message_submission_request::MessageSubmissionRequestTLV, TLV},
        types::{
            data_coding::DataCoding, esm_class::EsmClass, npi::Npi,
            registered_delivery::RegisteredDelivery, service_type::ServiceType, ton::Ton,
        },
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

/// The data_sm operation is similar to the submit_sm in that it provides a means to submit a
/// mobile-terminated message. However, data_sm is intended for packet-based applications
/// such as WAP in that it features a reduced PDU body containing fields relevant to WAP or
/// packet-based applications.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DataSm {
    /// The service_type parameter can be used to indicate the
    /// SMS Application service associated with the message.
    /// Specifying the service_type allows the ESME to avail of
    /// enhanced messaging services such as “replace by
    /// service_type” or control the teleservice used on the air
    /// interface.
    ///
    /// Set to NULL for default MC
    /// settings.
    pub serivce_type: ServiceType,
    /// Type of Number for source
    /// address.
    ///
    /// If not known, set to NULL
    /// (Unknown).
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for
    /// source address.
    ///
    /// If not known, set to NULL
    /// (Unknown).
    pub source_addr_npi: Npi,
    /// Address of SME which
    /// originated this message.
    ///
    /// If not known, set to NULL
    /// (Unknown).
    pub source_addr: COctetString<1, 21>,
    /// Type of Number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination address of this short message For mobile
    /// terminated messages, this is the directory number of the
    /// recipient MS.
    pub destination_addr: COctetString<1, 21>,
    /// Indicates Message Mode and Message Type.
    pub esm_class: EsmClass,
    /// Indicator to signify if a MC
    /// delivery receipt or an SME
    /// acknowledgement is required.
    pub registered_delivery: RegisteredDelivery,
    /// Defines the encoding scheme
    /// of the short message user data.
    pub data_coding: DataCoding,
    /// Message submission request TLVs ([`MessageSubmissionRequestTLV`])
    tlvs: Vec<TLV>,
}

impl DataSm {
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
        registered_delivery: RegisteredDelivery,
        data_coding: DataCoding,
        tlvs: Vec<MessageSubmissionRequestTLV>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            registered_delivery,
            data_coding,
            tlvs,
        }
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageSubmissionRequestTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionRequestTLV) {
        self.tlvs.push(tlv.into());
    }
}

impl Length for DataSm {
    fn length(&self) -> usize {
        self.serivce_type.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.dest_addr_ton.length()
            + self.dest_addr_npi.length()
            + self.destination_addr.length()
            + self.esm_class.length()
            + self.registered_delivery.length()
            + self.data_coding.length()
            + self.tlvs.length()
    }
}

impl Encode for DataSm {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.serivce_type.encode_to(writer));
        tri!(self.source_addr_ton.encode_to(writer));
        tri!(self.source_addr_npi.encode_to(writer));
        tri!(self.source_addr.encode_to(writer));
        tri!(self.dest_addr_ton.encode_to(writer));
        tri!(self.dest_addr_npi.encode_to(writer));
        tri!(self.destination_addr.encode_to(writer));
        tri!(self.esm_class.encode_to(writer));
        tri!(self.registered_delivery.encode_to(writer));
        tri!(self.data_coding.encode_to(writer));
        tri!(self.tlvs.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for DataSm {
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
        let registered_delivery = tri!(RegisteredDelivery::decode_from(reader));
        let data_coding = tri!(DataCoding::decode_from(reader));

        let tlvs_length = length.saturating_sub(
            serivce_type.length()
                + source_addr_ton.length()
                + source_addr_npi.length()
                + source_addr.length()
                + dest_addr_ton.length()
                + dest_addr_npi.length()
                + destination_addr.length()
                + esm_class.length()
                + registered_delivery.length()
                + data_coding.length(),
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
            registered_delivery,
            data_coding,
            tlvs,
        })
    }
}
