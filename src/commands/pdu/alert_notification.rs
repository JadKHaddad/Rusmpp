use super::Pdu;
use crate::{
    commands::types::{ms_availability_status::MsAvailabilityStatus, npi::Npi, ton::Ton},
    tlvs::SingleTlv,
    types::COctetString,
};

crate::create! {
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
        @[length = checked]
        ms_availability_status: Option<SingleTlv<MsAvailabilityStatus>>,
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
            ms_availability_status: ms_availability_status.map(From::from),
        }
    }

    //TODO: fix commented out code
    // pub fn ms_availability_status(&self) -> Option<MsAvailabilityStatus> {
    //     self.ms_availability_status
    //         .as_ref()
    //         .map(|tlv| tlv.value())
    //         .copied()
    // }

    pub fn set_ms_availability_status(
        &mut self,
        ms_availability_status: Option<MsAvailabilityStatus>,
    ) {
        self.ms_availability_status = ms_availability_status.map(From::from);
    }

    pub fn builder() -> AlertNotificationBuilder {
        AlertNotificationBuilder::new()
    }
}

impl From<AlertNotification> for Pdu {
    fn from(value: AlertNotification) -> Self {
        Self::AlertNotification(value)
    }
}

#[derive(Debug, Default)]
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
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<AlertNotification>();
    }
}
