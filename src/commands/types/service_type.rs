use crate::{
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::c_octet_string::{COctetString, Error as COctetStringError},
};
use std::str::FromStr;

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

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ServiceType {
    value: COctetString<1, 6>,
}

impl ServiceType {
    pub fn new(generic_service_type: GenericServiceType<'_>) -> Result<Self, COctetStringError> {
        Ok(Self {
            value: tri!(generic_service_type.value()),
        })
    }

    pub fn value(&self) -> &COctetString<1, 6> {
        &self.value
    }
}

impl Length for ServiceType {
    fn length(&self) -> usize {
        self.value.length()
    }
}

impl Encode for ServiceType {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.value.encode_to(writer));

        Ok(())
    }
}

impl Decode for ServiceType {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = tri!(COctetString::decode_from(reader));

        Ok(Self { value })
    }
}
