use super::Pdu;
use crate::{
    commands::types::{interface_version::InterfaceVersion, npi::Npi, ton::Ton},
    ende::decode::{Decode, DecodeError},
    impl_length_encode, tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

impl_length_encode! {
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
}

impl Bind {
    pub fn new(
        system_id: COctetString<1, 16>,
        password: COctetString<1, 9>,
        system_type: COctetString<1, 13>,
        interface_version: InterfaceVersion,
        addr_ton: Ton,
        addr_npi: Npi,
        address_range: COctetString<1, 41>,
    ) -> Self {
        Self {
            system_id,
            password,
            system_type,
            interface_version,
            addr_ton,
            addr_npi,
            address_range,
        }
    }

    pub fn builder() -> BindBuilder {
        BindBuilder::new()
    }

    pub fn into_bind_transmitter(self) -> Pdu {
        Pdu::BindTransmitter(self)
    }

    pub fn into_bind_receiver(self) -> Pdu {
        Pdu::BindReceiver(self)
    }

    pub fn into_bind_transceiver(self) -> Pdu {
        Pdu::BindTransceiver(self)
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

#[derive(Default)]
pub struct BindBuilder {
    inner: Bind,
}

impl BindBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
        self.inner.system_id = system_id;
        self
    }

    pub fn password(mut self, password: COctetString<1, 9>) -> Self {
        self.inner.password = password;
        self
    }

    pub fn system_type(mut self, system_type: COctetString<1, 13>) -> Self {
        self.inner.system_type = system_type;
        self
    }

    pub fn interface_version(mut self, interface_version: InterfaceVersion) -> Self {
        self.inner.interface_version = interface_version;
        self
    }

    pub fn addr_ton(mut self, addr_ton: Ton) -> Self {
        self.inner.addr_ton = addr_ton;
        self
    }

    pub fn addr_npi(mut self, addr_npi: Npi) -> Self {
        self.inner.addr_npi = addr_npi;
        self
    }

    pub fn address_range(mut self, address_range: COctetString<1, 41>) -> Self {
        self.inner.address_range = address_range;
        self
    }

    pub fn build(self) -> Bind {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode::<Bind>();
    }
}
