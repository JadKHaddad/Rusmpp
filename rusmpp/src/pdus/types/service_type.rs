use std::str::FromStr;

use crate::io::length::IoLength;
use crate::io::read::AsyncIoRead;
use crate::io::read::AsyncIoReadable;
use crate::io::read::IoReadError;
use crate::io::write::AsyncIoWritable;
use crate::io::write::AsyncIoWrite;
use crate::types::c_octet_string::COctetString;
use crate::types::c_octet_string::Error as COctetStringError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GenericServiceType<'a> {
    #[default]
    Default,
    CellularMessaging,
    CellularPaging,
    VoiceMailNotification,
    VoiceMailAlerting,
    WirelessApplicationProtocol,
    UnstructuredSupplementaryServicesData,
    CellBroadcastService,
    GenericUDPTransportService,
    Other(&'a str),
}

impl<'a> GenericServiceType<'a> {
    pub fn value(&self) -> Result<COctetString<1, 6>, COctetStringError> {
        match self {
            GenericServiceType::Default => COctetString::from_str(""),
            GenericServiceType::CellularMessaging => COctetString::from_str("CMT"),
            GenericServiceType::CellularPaging => COctetString::from_str("CPT"),
            GenericServiceType::VoiceMailNotification => COctetString::from_str("VMN"),
            GenericServiceType::VoiceMailAlerting => COctetString::from_str("VMA"),
            GenericServiceType::WirelessApplicationProtocol => COctetString::from_str("WAP"),
            GenericServiceType::UnstructuredSupplementaryServicesData => {
                COctetString::from_str("USSD")
            }
            GenericServiceType::CellBroadcastService => COctetString::from_str("CBS"),
            GenericServiceType::GenericUDPTransportService => COctetString::from_str("GUTS"),
            GenericServiceType::Other(value) => COctetString::from_str(value),
        }
    }
}

/// The service_type parameter can be used to indicate the SMS Application service associated
/// with the message. Specifying the service_type allows the ESME to:
///
///    • Avail of enhanced messaging services such as replace_if_present by service type
///      (generic to all network types).
///    • Control the teleservice used on the air interface (e.g. ANSI-136/TDMA, IS-95/CDMA).
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ServiceType {
    value: COctetString<1, 6>,
}

impl ServiceType {
    pub fn new(generic_service_type: GenericServiceType<'_>) -> Result<Self, COctetStringError> {
        Ok(Self {
            value: generic_service_type.value()?,
        })
    }

    pub fn value(&self) -> &COctetString<1, 6> {
        &self.value
    }
}

impl IoLength for ServiceType {
    fn length(&self) -> usize {
        self.value.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for ServiceType {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.value.async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for ServiceType {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            value: COctetString::async_io_read(buf).await?,
        })
    }
}
