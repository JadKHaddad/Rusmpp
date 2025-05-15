use super::Pdu;
use crate::{
    commands::types::interface_version::InterfaceVersion, tlvs::SingleTlv, types::COctetString,
};

macro_rules! declare_bind_resp {
    ($name:ident, $builder_name:ident) => {
        crate::create! {
            #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct $name {
                /// MC identifier.
                ///
                /// Identifies the MC to the ESME.
                pub system_id: COctetString<1, 16>,
                /// [`TlvValue::ScInterfaceVersion`].
                ///
                /// `SMPP` version supported by MC.
                @[length = checked]
                sc_interface_version: Option<SingleTlv<InterfaceVersion>>,
            }
        }

        impl $name {
            pub fn new(
                system_id: COctetString<1, 16>,
                sc_interface_version: Option<InterfaceVersion>,
            ) -> Self {
                Self {
                    system_id,
                    sc_interface_version: sc_interface_version.map(From::from),
                }
            }

            pub fn sc_interface_version_tlv(&self) -> Option<&SingleTlv<InterfaceVersion>> {
                self.sc_interface_version.as_ref()
            }

            pub fn sc_interface_version(&self) -> Option<InterfaceVersion> {
                self.sc_interface_version_tlv()
                    .and_then(SingleTlv::value)
                    .copied()
            }

            pub fn set_sc_interface_version(
                &mut self,
                sc_interface_version: Option<InterfaceVersion>,
            ) {
                self.sc_interface_version = sc_interface_version.map(From::from);
            }

            pub fn builder() -> $builder_name {
                $builder_name::new()
            }
        }

        #[derive(Debug, Default)]
        pub struct $builder_name {
            inner: $name,
        }

        impl $builder_name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn system_id(mut self, system_id: COctetString<1, 16>) -> Self {
                self.inner.system_id = system_id;
                self
            }

            pub fn sc_interface_version(
                mut self,
                sc_interface_version: Option<InterfaceVersion>,
            ) -> Self {
                self.inner.set_sc_interface_version(sc_interface_version);
                self
            }

            pub fn build(self) -> $name {
                self.inner
            }
        }
    };
}

declare_bind_resp!(BindTransmitterResp, BindTransmitterRespBuilder);
declare_bind_resp!(BindReceiverResp, BindReceiverRespBuilder);
declare_bind_resp!(BindTransceiverResp, BindTransceiverRespBuilder);

impl From<BindTransmitterResp> for Pdu {
    fn from(value: BindTransmitterResp) -> Self {
        Self::BindTransmitterResp(value)
    }
}

impl From<BindReceiverResp> for Pdu {
    fn from(value: BindReceiverResp) -> Self {
        Self::BindReceiverResp(value)
    }
}

impl From<BindTransceiverResp> for Pdu {
    fn from(value: BindTransceiverResp) -> Self {
        Self::BindTransceiverResp(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<BindTransmitterResp>();
        crate::tests::encode_decode_with_length_test_instances::<BindReceiverResp>();
        crate::tests::encode_decode_with_length_test_instances::<BindTransceiverResp>();
    }
}
