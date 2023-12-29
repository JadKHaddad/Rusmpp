use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{length::IoLength, read::AsyncIoRead},
    pdus::{
        tlvs::tlv::{MessageSubmissionRequestTLV, TLV},
        types::unsuccess_sme::UnsuccessSme,
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
pub struct SubmitMultiResp {
    message_id: COctetString<1, 65>,
    no_unsuccess: u8,
    #[rusmpp_io_read(count=no_unsuccess)]
    unsuccess_sme: Vec<UnsuccessSme>,
    #[rusmpp_io_read(length=(length - all_before))]
    tlvs: Vec<TLV>,
}

impl SubmitMultiResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        unsuccess_sme: Vec<UnsuccessSme>,
        tlvs: Vec<MessageSubmissionRequestTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        let no_unsuccess = unsuccess_sme.len() as u8;
        Self {
            message_id,
            no_unsuccess,
            unsuccess_sme,
            tlvs,
        }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn no_unsuccess(&self) -> u8 {
        self.no_unsuccess
    }

    pub fn unsuccess_sme(&self) -> &[UnsuccessSme] {
        &self.unsuccess_sme
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (COctetString<1, 65>, u8, Vec<UnsuccessSme>, Vec<TLV>) {
        (
            self.message_id,
            self.no_unsuccess,
            self.unsuccess_sme,
            self.tlvs,
        )
    }
}
