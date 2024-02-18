use crate::{
    commands::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bind {
    /// Identifies the ESME system
    /// requesting to bind with the MC.
    pub system_id: COctetString<1, 16>,
    /// The password may be used by the
    /// MC to authenticate the ESME
    /// requesting to bind.
    pub password: COctetString<1, 9>,
    /// Identifies the type of ESME system
    /// requesting to bind with the MC.
    pub system_type: COctetString<1, 13>,
    /// Identifies the version of the SMPP
    /// protocol supported by the ESME.
    pub interface_version: InterfaceVersion,
    /// Type of Number (TON) for ESME
    /// address(es) served via this SMPP session.
    ///
    /// Set to NULL (Unknown) if not known.
    pub addr_ton: Ton,
    /// Numbering Plan Indicator (NPI) for
    /// ESME address(es) served via this SMPP session.
    ///
    /// Set to NULL (Unknown) if not known.
    pub addr_npi: Npi,
    /// A single ESME address or a range of
    /// ESME addresses served via this SMPP session.
    ///   
    /// Set to NULL if not known.
    pub address_range: COctetString<1, 41>,
}

impl Length for Bind {
    fn length(&self) -> usize {
        self.system_id.length()
            + self.password.length()
            + self.system_type.length()
            + self.interface_version.length()
            + self.addr_ton.length()
            + self.addr_npi.length()
            + self.address_range.length()
    }
}

impl Encode for Bind {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.system_id.encode_to(writer));
        tri!(self.password.encode_to(writer));
        tri!(self.system_type.encode_to(writer));
        tri!(self.interface_version.encode_to(writer));
        tri!(self.addr_ton.encode_to(writer));
        tri!(self.addr_npi.encode_to(writer));
        tri!(self.address_range.encode_to(writer));

        Ok(())
    }
}

impl Decode for Bind {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let system_id = tri!(COctetString::decode_from(reader));
        let password = tri!(COctetString::decode_from(reader));
        let system_type = tri!(COctetString::decode_from(reader));
        let interface_version = tri!(InterfaceVersion::decode_from(reader));
        let addr_ton = tri!(Ton::decode_from(reader));
        let addr_npi = tri!(Npi::decode_from(reader));
        let address_range = tri!(COctetString::decode_from(reader));

        Ok(Self {
            system_id,
            password,
            system_type,
            interface_version,
            addr_ton,
            addr_npi,
            address_range,
        })
    }
}
