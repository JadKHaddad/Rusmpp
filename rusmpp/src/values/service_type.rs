use crate::types::COctetString;

/// Helper for creating a [`ServiceType`] with predefined values.
///
/// # Example
/// ```rust
/// use rusmpp::values::{GenericServiceType, ServiceType};
///
/// let service_type = ServiceType::new(GenericServiceType::CellularMessaging.into());
/// assert_eq!(service_type.value().bytes(), b"CMT\0");
/// assert_eq!(service_type.value().to_str(), Ok("CMT"));
///
/// let generic_service_type = GenericServiceType::VoiceMailAlerting;
/// let service_type: ServiceType = generic_service_type.into();
/// assert_eq!(service_type.value().bytes(), b"VMA\0");
/// assert_eq!(service_type.value().to_str(), Ok("VMA"));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GenericServiceType {
    /// Empty value
    #[default]
    Default,
    /// CMT
    CellularMessaging,
    /// CPT
    CellularPaging,
    /// VMN
    VoiceMailNotification,
    /// VMA
    VoiceMailAlerting,
    /// WAP
    WirelessApplicationProtocol,
    /// USSD
    UnstructuredSupplementaryServicesData,
    /// CBS
    CellBroadcastService,
    /// GUTS
    GenericUDPTransportService,
}

impl From<GenericServiceType> for COctetString<1, 6> {
    fn from(value: GenericServiceType) -> Self {
        match value {
            GenericServiceType::Default => COctetString::null(),
            GenericServiceType::CellularMessaging => COctetString::new_unchecked(b"CMT\0"),
            GenericServiceType::CellularPaging => COctetString::new_unchecked(b"CPT\0"),
            GenericServiceType::VoiceMailNotification => COctetString::new_unchecked(b"VMN\0"),
            GenericServiceType::VoiceMailAlerting => COctetString::new_unchecked(b"VMA\0"),
            GenericServiceType::WirelessApplicationProtocol => {
                COctetString::new_unchecked(b"WAP\0")
            }
            GenericServiceType::UnstructuredSupplementaryServicesData => {
                COctetString::new_unchecked(b"USSD\0")
            }
            GenericServiceType::CellBroadcastService => COctetString::new_unchecked(b"CBS\0"),
            GenericServiceType::GenericUDPTransportService => {
                COctetString::new_unchecked(b"GUTS\0")
            }
        }
    }
}

impl From<GenericServiceType> for ServiceType {
    fn from(value: GenericServiceType) -> Self {
        ServiceType::new(value.into())
    }
}

crate::create! {
    /// The service_type parameter can be used to indicate the SMS Application service associated
    /// with the message. Specifying the service_type allows the ESME to:
    ///
    /// * Avail of enhanced messaging services such as replace_if_present by service type
    ///   (generic to all network types).
    /// * Control the teleservice used on the air interface (e.g. ANSI-136/TDMA, IS-95/CDMA).
    ///
    /// MCs may implicitly associate a ‘replace if present’ function from the indicated service_type in
    /// a message submission operation, i.e., the MC will always replace an existing message
    /// pending delivery, that has the same originating and destination address as the submitted
    /// message. For example, a MC can ensure that a Voice Mail System using a service_type of
    /// “VMA” has at most one outstanding notification per destination MS by automatically invoking
    /// the “replace if present” function.
    ///
    /// Note: In the case of Cell Broadcast Service replace functionality by service type is not
    /// supported.
    ///
    /// See [`GenericServiceType`].
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct ServiceType {
        value: COctetString<1, 6>,
    }
}

impl ServiceType {
    pub fn new(value: COctetString<1, 6>) -> Self {
        Self { value }
    }

    /// Create a new [`ServiceType`] with a value of 0.
    pub fn null() -> Self {
        Self {
            value: COctetString::null(),
        }
    }

    pub fn value(&self) -> &COctetString<1, 6> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<ServiceType>();
    }
}
