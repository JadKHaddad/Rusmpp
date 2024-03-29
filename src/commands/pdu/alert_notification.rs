use super::Pdu;
use crate::{
    commands::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{ms_availability_status::MsAvailabilityStatus, npi::Npi, ton::Ton},
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        length::Length,
    },
    impl_length_encode, tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

impl_length_encode! {
    /// The alert_notification PDU is sent by the MC to the ESME across a Receiver or Transceiver
    /// session. It is sent when the MC has detected that a particular mobile subscriber has become
    /// available and a delivery pending flag had been previously set for that subscriber by means of
    /// the set_dpf TLV.
    ///
    /// A typical use of this operation is to trigger a data content ‘Push’ to the subscriber from a WAP
    /// Proxy Server.
    ///
    /// Note: There is no associated alert_notification_resp PDU.
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct AlertNotification {
        /// Type of Number for alert SME.
        pub source_addr_ton: Ton,
        /// Numbering Plan Indicator for alert SME.
        pub source_addr_npi: Npi,
        /// Address of alert SME.
        pub source_addr: COctetString<1, 65>,
        /// Type of Number for ESME address
        /// which requested the alert.
        pub esme_addr_ton: Ton,
        /// Numbering Plan Indicator for ESME
        /// address which requested the alert.
        pub esme_addr_npi: Npi,
        /// Address for ESME which requested the alert.
        pub esme_addr: COctetString<1, 65>,
        /// The status of the mobile station.
        ms_availability_status: Option<TLV>,
    }
}

impl AlertNotification {
    pub fn new(
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 65>,
        esme_addr_ton: Ton,
        esme_addr_npi: Npi,
        esme_addr: COctetString<1, 65>,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) -> Self {
        Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status: ms_availability_status
                .map(|v| TLV::new(TLVValue::MsAvailabilityStatus(v))),
        }
    }

    pub fn ms_availability_status(&self) -> Option<&TLV> {
        self.ms_availability_status.as_ref()
    }

    pub fn set_ms_availability_status(
        &mut self,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) {
        self.ms_availability_status =
            ms_availability_status.map(|v| TLV::new(TLVValue::MsAvailabilityStatus(v)));
    }

    pub fn builder() -> AlertNotificationBuilder {
        AlertNotificationBuilder::new()
    }

    pub fn into_alert_notification(self) -> Pdu {
        Pdu::AlertNotification(self)
    }
}

impl DecodeWithLength for AlertNotification {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let source_addr_ton = tri!(Ton::decode_from(reader));
        let source_addr_npi = tri!(Npi::decode_from(reader));
        let source_addr = tri!(COctetString::decode_from(reader));
        let esme_addr_ton = tri!(Ton::decode_from(reader));
        let esme_addr_npi = tri!(Npi::decode_from(reader));
        let esme_addr = tri!(COctetString::decode_from(reader));

        let ms_availability_status_length = length.saturating_sub(
            source_addr_ton.length()
                + source_addr_npi.length()
                + source_addr.length()
                + esme_addr_ton.length()
                + esme_addr_npi.length()
                + esme_addr.length(),
        );

        let ms_availability_status = tri!(TLV::length_checked_decode_from(
            reader,
            ms_availability_status_length
        ));

        Ok(Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status,
        })
    }
}

#[derive(Default)]
pub struct AlertNotificationBuilder {
    inner: AlertNotification,
}

impl AlertNotificationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn source_addr_ton(mut self, source_addr_ton: Ton) -> Self {
        self.inner.source_addr_ton = source_addr_ton;
        self
    }

    pub fn source_addr_npi(mut self, source_addr_npi: Npi) -> Self {
        self.inner.source_addr_npi = source_addr_npi;
        self
    }

    pub fn source_addr(mut self, source_addr: COctetString<1, 65>) -> Self {
        self.inner.source_addr = source_addr;
        self
    }

    pub fn esme_addr_ton(mut self, esme_addr_ton: Ton) -> Self {
        self.inner.esme_addr_ton = esme_addr_ton;
        self
    }

    pub fn esme_addr_npi(mut self, esme_addr_npi: Npi) -> Self {
        self.inner.esme_addr_npi = esme_addr_npi;
        self
    }

    pub fn esme_addr(mut self, esme_addr: COctetString<1, 65>) -> Self {
        self.inner.esme_addr = esme_addr;
        self
    }

    pub fn ms_availability_status(
        mut self,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) -> Self {
        self.inner
            .set_ms_availability_status(ms_availability_status);
        self
    }

    pub fn build(self) -> AlertNotification {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<AlertNotification>();
    }
}
