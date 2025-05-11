use crate::{
    commands::{
        tlvs::tlv::{message_submission_request::MessageSubmissionRequestTlv, Tlv},
        types::{
            data_coding::DataCoding, esm_class::EsmClass, npi::Npi,
            registered_delivery::RegisteredDelivery, service_type::ServiceType, ton::Ton,
        },
    },
    types::COctetString,
};

use super::Pdu;

crate::create! {
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
        pub service_type: ServiceType,
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
        @[length = unchecked]
        tlvs: Vec<Tlv>,
    }
}

impl DataSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
        esm_class: EsmClass,
        registered_delivery: RegisteredDelivery,
        data_coding: DataCoding,
        tlvs: Vec<impl Into<MessageSubmissionRequestTlv>>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        Self {
            service_type,
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

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<MessageSubmissionRequestTlv>>) {
        self.tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionRequestTlv>) {
        let tlv: MessageSubmissionRequestTlv = tlv.into();
        let tlv: Tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> DataSmBuilder {
        DataSmBuilder::new()
    }
}

impl From<DataSm> for Pdu {
    fn from(value: DataSm) -> Self {
        Self::DataSm(value)
    }
}

#[derive(Debug, Default)]
pub struct DataSmBuilder {
    inner: DataSm,
}

impl DataSmBuilder {
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

    pub fn tlvs(mut self, tlvs: Vec<impl Into<MessageSubmissionRequestTlv>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<MessageSubmissionRequestTlv>) -> Self {
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
        crate::tests::default_encode_decode_with_length::<DataSm>();
    }
}
