use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::interface_version::InterfaceVersion,
    },
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use getset::{Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Default,
    Getters,
    Setters,
    Builder,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoReadLength,
)]
#[builder(default)]
pub struct BindResp {
    #[getset(get = "pub", set = "pub")]
    system_id: COctetString<1, 16>,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(private, setter(name = "_sc_interface_version"))]
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

    pub fn set_sc_interface_version(&mut self, sc_interface_version: Option<InterfaceVersion>) {
        self.sc_interface_version =
            sc_interface_version.map(|v| TLV::new(TLVValue::ScInterfaceVersion(v)));
    }
}

impl BindRespBuilder {
    pub fn sc_interface_version(
        &mut self,
        sc_interface_version: Option<InterfaceVersion>,
    ) -> &mut Self {
        self.sc_interface_version = sc_interface_version
            .map(|v| TLV::new(TLVValue::ScInterfaceVersion(v)))
            .into();
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<BindResp>().await;
    }

    #[test]
    fn builder() {
        let bind_resp = BindRespBuilder::default()
            .system_id(COctetString::from_str("system_id").unwrap())
            .build()
            .unwrap();

        assert_eq!(bind_resp.system_id().to_str().unwrap(), "system_id");
        assert!(bind_resp.sc_interface_version().is_none());

        let bind_resp = BindRespBuilder::default()
            .sc_interface_version(Some(InterfaceVersion::Smpp5_0))
            .build()
            .unwrap();

        assert_eq!(bind_resp.system_id().to_str().unwrap(), "");
        assert_eq!(
            bind_resp
                .sc_interface_version()
                .as_ref()
                .unwrap()
                .value()
                .unwrap(),
            &TLVValue::ScInterfaceVersion(InterfaceVersion::Smpp5_0)
        );
    }
}
