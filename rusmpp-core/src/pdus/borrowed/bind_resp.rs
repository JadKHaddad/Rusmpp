use rusmpp_macros::Rusmpp;

use crate::{
    tlvs::borrowed::{Tlv, TlvValue},
    types::borrowed::COctetString,
    values::interface_version::InterfaceVersion,
};

macro_rules! bind_resp {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
        #[rusmpp(decode = borrowed, test = skip)]
        #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
        #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
        pub struct $name<'a> {
            /// MC identifier.
            ///
            /// Identifies the MC to the ESME.
            pub system_id: COctetString<'a, 1, 16>,
            /// `SMPP` version supported by MC. [`ScInterfaceVersion`].
            #[rusmpp(length = "checked")]
            sc_interface_version: Option<Tlv<'a>>,
        }

        impl<'a> $name<'a> {
            pub fn new(
                system_id: COctetString<'a, 1, 16>,
                sc_interface_version: Option<InterfaceVersion>,
            ) -> Self {
                Self {
                    system_id,
                    sc_interface_version: sc_interface_version
                        .map(TlvValue::ScInterfaceVersion)
                        .map(From::from),
                }
            }

            pub const fn sc_interface_version_tlv(&'_ self) -> Option<&'_ Tlv<'_>> {
                self.sc_interface_version.as_ref()
            }

            pub fn sc_interface_version(&self) -> Option<InterfaceVersion> {
                self.sc_interface_version_tlv()
                    .and_then(|tlv| match tlv.value() {
                        Some(TlvValue::ScInterfaceVersion(value)) => Some(value),
                        _ => None,
                    })
                    .copied()
            }

            pub fn set_sc_interface_version(
                &mut self,
                sc_interface_version: Option<InterfaceVersion>,
            ) {
                self.sc_interface_version = sc_interface_version
                    .map(TlvValue::ScInterfaceVersion)
                    .map(From::from);
            }

            ::pastey::paste! {
                pub fn builder() -> [<$name Builder>]<'a> {
                    [<$name Builder>]::new()
                }
            }
        }

        ::pastey::paste! {
            #[derive(Debug, Default)]
            pub struct [<$name Builder>]<'a> {
               inner: $name<'a>,
            }

            impl<'a> [<$name Builder>]<'a> {
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn system_id(mut self, system_id: COctetString<'a, 1, 16>) -> Self {
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

                pub fn build(self) -> $name<'a> {
                    self.inner
                }
            }
        }
    };
}

bind_resp!(BindTransmitterResp);
bind_resp!(BindReceiverResp);
bind_resp!(BindTransceiverResp);
