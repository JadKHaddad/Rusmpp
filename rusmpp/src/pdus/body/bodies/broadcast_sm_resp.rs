use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::tlvs::tlv::{BroadcastResponseTLV, TLV},
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
pub struct BroadcastSmResp {
    message_id: COctetString<1, 65>,
    #[rusmpp_io_read(length=(length - all_before))]
    tlvs: Vec<TLV>,
}

impl BroadcastSmResp {
    pub fn new(message_id: COctetString<1, 65>, tlvs: Vec<BroadcastResponseTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect();

        Self { message_id, tlvs }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (COctetString<1, 65>, Vec<TLV>) {
        (self.message_id, self.tlvs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdus::{
        tlvs::{
            tlv_value::BroadcastResponseTLVValue,
            tlv_values::broadcast_area_identifier::{BroadcastAreaFormat, BroadcastAreaIdentifier},
        },
        types::command_status::CommandStatus,
    };
    use rusmpp_io::{
        io::{read::AsyncIoReadWithLength, write::AsyncIoWrite},
        types::octet_string::OctetString,
    };
    use std::{io::Cursor, str::FromStr};

    #[tokio::test]
    async fn write_read_compare() {
        let broadcast_sm_resp = BroadcastSmResp::new(
            COctetString::from_str("message_id").unwrap(),
            vec![
                BroadcastResponseTLV::new(BroadcastResponseTLVValue::BroadcastErrorStatus(
                    CommandStatus::EsmeRalybnd,
                )),
                BroadcastResponseTLV::new(BroadcastResponseTLVValue::BroadcastAreaIdentifier(
                    BroadcastAreaIdentifier {
                        format: BroadcastAreaFormat::AliasName,
                        area: OctetString::from_str("an area!").unwrap(),
                    },
                )),
            ],
        );

        let mut curser = Cursor::new(Vec::new());

        broadcast_sm_resp
            .async_io_write(&mut curser)
            .await
            .expect("Failed to write bytes");

        curser.set_position(0);

        let broadcast_sm_resp_read =
            BroadcastSmResp::async_io_read(&mut curser, broadcast_sm_resp.length())
                .await
                .expect("Failed to read bytes");

        assert_eq!(broadcast_sm_resp, broadcast_sm_resp_read);
    }
}
