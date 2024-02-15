use crate::{
    commands::{
        tlvs::{
            tlv::TLV, tlv_value::TLVValue, tlv_values::ms_availability_status::MsAvailabilityStatus,
        },
        types::{npi::Npi, ton::Ton},
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::c_octet_string::COctetString,
};

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
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 65>,
    pub esme_addr_ton: Ton,
    pub esme_addr_npi: Npi,
    pub esme_addr: COctetString<1, 65>,
    ms_availability_status: Option<TLV>,
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
}

impl Length for AlertNotification {
    fn length(&self) -> usize {
        self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.esme_addr_ton.length()
            + self.esme_addr_npi.length()
            + self.esme_addr.length()
            + self.ms_availability_status.length()
    }
}

impl Encode for AlertNotification {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.source_addr_ton.encode_to(writer));
        tri!(self.source_addr_npi.encode_to(writer));
        tri!(self.source_addr.encode_to(writer));
        tri!(self.esme_addr_ton.encode_to(writer));
        tri!(self.esme_addr_npi.encode_to(writer));
        tri!(self.esme_addr.encode_to(writer));
        tri!(self.ms_availability_status.encode_to(writer));

        Ok(())
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
