use rusmpp_macros::RusmppIo;

use rusmpp_io::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    },
    types::{c_octet_string::COctetString, option},
};

use crate::pdus::{
    tlvs::{tlv::TLV, tlv_value::TLVValue},
    types::interface_version::InterfaceVersion,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct BindResp {
    pub system_id: COctetString<1, 16>,
    pub sc_interface_version: Option<TLV>,
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
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for BindResp {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let system_id = COctetString::async_io_read(buf).await?;

        let sc_interface_version_expected_len = length.saturating_sub(system_id.length());

        let sc_interface_version =
            option::async_io_read(buf, sc_interface_version_expected_len).await?;

        Ok(Self {
            system_id,
            sc_interface_version,
        })
    }
}
