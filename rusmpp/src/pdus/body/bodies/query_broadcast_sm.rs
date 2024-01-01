use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{npi::Npi, ton::Ton},
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
pub struct QueryBroadcastSm {
    message_id: COctetString<1, 65>,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    #[rusmpp_io_read(length=(length - all_before))]
    user_message_reference: Option<TLV>,
}

impl QueryBroadcastSm {
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        user_message_reference: Option<u16>,
    ) -> Self {
        let user_message_reference =
            user_message_reference.map(|v| TLV::new(TLVValue::UserMessageReference(v)));

        Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            user_message_reference,
        }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn source_addr_ton(&self) -> Ton {
        self.source_addr_ton
    }

    pub fn source_addr_npi(&self) -> Npi {
        self.source_addr_npi
    }

    pub fn source_addr(&self) -> &COctetString<1, 21> {
        &self.source_addr
    }

    pub fn user_message_reference(&self) -> Option<&TLV> {
        self.user_message_reference.as_ref()
    }

    pub fn into_parts(
        self,
    ) -> (
        COctetString<1, 65>,
        Ton,
        Npi,
        COctetString<1, 21>,
        Option<TLV>,
    ) {
        (
            self.message_id,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.user_message_reference,
        )
    }
}