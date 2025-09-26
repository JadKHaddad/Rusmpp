pub mod borrowed;
#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

// TODO: add example for the owned ServiceType as well
/// Helper for creating a [`ServiceType`] with predefined values.
///
/// # Example
/// ```rust
/// use rusmpp::values::{GenericServiceType, borrowed::ServiceType};
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
