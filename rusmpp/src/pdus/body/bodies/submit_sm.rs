use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

use crate::{
    io::{length::IoLength, read::AsyncIoRead},
    pdus::tlvs::tlv::{MessageSubmissionRequestTLV, TLV},
};

use super::s_sm::SSm;

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
pub struct SubmitSm {
    ssm: SSm,
    #[rusmpp_io_read(length=(length - all_before))]
    tlvs: Vec<TLV>,
}

impl SubmitSm {
    pub fn new(ssm: SSm, tlvs: Vec<MessageSubmissionRequestTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        let ssm = SSm::check_for_message_payload_and_update(ssm, &tlvs);

        Self { ssm, tlvs }
    }

    pub fn ssm(&self) -> &SSm {
        &self.ssm
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (SSm, Vec<TLV>) {
        (self.ssm, self.tlvs)
    }
}
