use super::Pdu;
use crate::{
    commands::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::interface_version::InterfaceVersion,
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::c_octet_string::COctetString,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BindResp {
    /// MC identifier.
    ///
    /// Identifies the MC to the ESME.
    pub system_id: COctetString<1, 16>,
    /// SMPP version supported by MC.
    sc_interface_version: Option<TLV>,
}

impl BindResp {
    pub fn new(
        system_id: COctetString<1, 16>,
        sc_interface_version: Option<InterfaceVersion>,
    ) -> Self {
        Self {
            system_id,
            sc_interface_version: sc_interface_version
                .map(|v| TLV::new(TLVValue::ScInterfaceVersion(v))),
        }
    }

    pub fn sc_interface_version(&self) -> Option<&TLV> {
        self.sc_interface_version.as_ref()
    }

    pub fn set_sc_interface_version(&mut self, sc_interface_version: Option<InterfaceVersion>) {
        self.sc_interface_version =
            sc_interface_version.map(|v| TLV::new(TLVValue::ScInterfaceVersion(v)));
    }

    pub fn builder() -> BindRespBuilder {
        BindRespBuilder::new()
    }

    pub fn into_bind_transmitter_resp(self) -> Pdu {
        Pdu::BindTransmitterResp(self)
    }

    pub fn into_bind_receiver_resp(self) -> Pdu {
        Pdu::BindReceiverResp(self)
    }

    pub fn into_bind_transceiver_resp(self) -> Pdu {
        Pdu::BindTransceiverResp(self)
    }
}

impl Length for BindResp {
    fn length(&self) -> usize {
        self.system_id.length() + self.sc_interface_version.length()
    }
}

impl Encode for BindResp {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.system_id.encode_to(writer));
        tri!(self.sc_interface_version.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for BindResp {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let system_id = tri!(COctetString::decode_from(reader));

        let sc_interface_version_length = length.saturating_sub(system_id.length());

        let sc_interface_version = tri!(TLV::length_checked_decode_from(
            reader,
            sc_interface_version_length
        ));

        Ok(Self {
            system_id,
            sc_interface_version,
        })
    }
}

#[derive(Default)]
pub struct BindRespBuilder {
    inner: BindResp,
}

impl BindRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
        self.inner.system_id = system_id;
        self
    }

    pub fn sc_interface_version(mut self, sc_interface_version: Option<InterfaceVersion>) -> Self {
        self.inner.set_sc_interface_version(sc_interface_version);
        self
    }

    pub fn build(self) -> BindResp {
        self.inner
    }
}
