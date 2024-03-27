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
    impl_length_encode, tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

impl_length_encode! {
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

    pub fn builder() -> DataSmBuilder {
        DataSmBuilder::new()
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

#[derive(Default)]
pub struct DataSmBuilder {
    inner: DataSm,
}

impl DataSmBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn service_type(mut self, service_type: ServiceType) -> Self {
        self.inner.serivce_type = service_type;
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

    pub fn registered_delivery(mut self, registered_delivery: RegisteredDelivery) -> Self {
        self.inner.registered_delivery = registered_delivery;
        self
    }

    pub fn data_coding(mut self, data_coding: DataCoding) -> Self {
        self.inner.data_coding = data_coding;
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<MessageSubmissionRequestTLV>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: MessageSubmissionRequestTLV) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> DataSm {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<DataSm>();
    }
}
