use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

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

#[derive(
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
pub struct BindResp {
    pub system_id: COctetString<1, 16>,
    #[rusmpp_io_read(length=(length - all_before))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusmpp_io::io::{read::AsyncIoReadWithLength, write::AsyncIoWrite};
    use std::{io::Cursor, str::FromStr};

    #[tokio::test]
    async fn write_read_compare() {
        let bind_resp = BindResp::new(
            COctetString::from_str("system_id").unwrap(),
            Some(InterfaceVersion::Smpp5_0),
        );

        let mut curser = Cursor::new(Vec::new());

        bind_resp
            .async_io_write(&mut curser)
            .await
            .expect("Failed to write bytes");

        curser.set_position(0);

        let bind_resp_read = BindResp::async_io_read(&mut curser, bind_resp.length())
            .await
            .expect("Failed to read bytes");

        assert_eq!(bind_resp, bind_resp_read);
    }
}
