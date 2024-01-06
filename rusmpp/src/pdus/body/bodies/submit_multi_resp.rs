use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::tlv::{MessageSubmissionRequestTLV, TLV},
        types::unsuccess_sme::UnsuccessSme,
    },
    types::c_octet_string::COctetString,
};
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Default,
    Getters,
    CopyGetters,
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
pub struct SubmitMultiResp {
    #[getset(get = "pub", set = "pub")]
    message_id: COctetString<1, 65>,
    #[getset(get_copy = "pub")]
    no_unsuccess: u8,
    #[getset(get = "pub")]
    #[rusmpp_io_read(count=no_unsuccess)]
    #[builder(setter(custom))]
    unsuccess_sme: Vec<UnsuccessSme>,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
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

    pub fn set_unsuccess_sme(&mut self, unsuccess_sme: Vec<UnsuccessSme>) {
        self.no_unsuccess = unsuccess_sme.len() as u8;
        self.unsuccess_sme = unsuccess_sme;
    }

    pub fn push_unsuccess_sme(&mut self, unsuccess_sme: UnsuccessSme) {
        self.unsuccess_sme.push(unsuccess_sme);
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageSubmissionRequestTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionRequestTLV) {
        self.tlvs.push(tlv.into());
    }
}

impl SubmitMultiRespBuilder {
    pub fn unsuccess_sme(&mut self, unsuccess_sme: Vec<UnsuccessSme>) -> &mut Self {
        self.no_unsuccess = Some(unsuccess_sme.len() as u8);
        self.unsuccess_sme = Some(unsuccess_sme);
        self
    }

    pub fn push_unsuccess_sme(&mut self, unsuccess_sme: UnsuccessSme) -> &mut Self {
        let self_unsuccess_sme = self.unsuccess_sme.get_or_insert_with(Vec::new);
        self_unsuccess_sme.push(unsuccess_sme);
        self.no_unsuccess = Some(self_unsuccess_sme.len() as u8);

        self
    }

    pub fn tlvs(&mut self, tlvs: Vec<MessageSubmissionRequestTLV>) -> &mut Self {
        self.tlvs = Some(tlvs.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionRequestTLV) -> &mut Self {
        self.tlvs.get_or_insert_with(Vec::new).push(tlv.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<SubmitMultiResp>().await;
    }

    #[test]
    fn unsuccess_sme() {
        let submit_multi_resp = SubmitMultiResp::default();

        assert_eq!(submit_multi_resp.no_unsuccess(), 0);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &Vec::<UnsuccessSme>::new()
        );

        let mut submit_multi_resp = SubmitMultiResp::default();
        submit_multi_resp.push_unsuccess_sme(UnsuccessSme::default());

        assert_eq!(submit_multi_resp.no_unsuccess(), 1);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &vec![UnsuccessSme::default()]
        );

        submit_multi_resp.push_unsuccess_sme(UnsuccessSme::default());

        assert_eq!(submit_multi_resp.no_unsuccess(), 2);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &vec![UnsuccessSme::default(), UnsuccessSme::default()]
        );

        let mut submit_multi_resp = SubmitMultiResp::default();
        submit_multi_resp.set_unsuccess_sme(vec![UnsuccessSme::default(), UnsuccessSme::default()]);

        assert_eq!(submit_multi_resp.no_unsuccess(), 2);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &vec![UnsuccessSme::default(), UnsuccessSme::default()]
        );

        submit_multi_resp.set_unsuccess_sme(vec![]);

        assert_eq!(submit_multi_resp.no_unsuccess(), 0);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &Vec::<UnsuccessSme>::new()
        );
    }

    #[test]
    fn builder_unsuccess_sme() {
        let submit_multi_resp = SubmitMultiRespBuilder::default()
            .unsuccess_sme(vec![UnsuccessSme::default(), UnsuccessSme::default()])
            .build()
            .unwrap();

        assert_eq!(submit_multi_resp.no_unsuccess(), 2);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &vec![UnsuccessSme::default(), UnsuccessSme::default()]
        );

        let submit_multi_resp = SubmitMultiRespBuilder::default()
            .push_unsuccess_sme(UnsuccessSme::default())
            .push_unsuccess_sme(UnsuccessSme::default())
            .build()
            .unwrap();

        assert_eq!(submit_multi_resp.no_unsuccess(), 2);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &vec![UnsuccessSme::default(), UnsuccessSme::default()]
        );

        let submit_multi_resp = SubmitMultiRespBuilder::default()
            .push_unsuccess_sme(UnsuccessSme::default())
            .unsuccess_sme(vec![UnsuccessSme::default(), UnsuccessSme::default()])
            .build()
            .unwrap();

        assert_eq!(submit_multi_resp.no_unsuccess(), 2);
        assert_eq!(
            submit_multi_resp.unsuccess_sme(),
            &vec![UnsuccessSme::default(), UnsuccessSme::default()]
        );

        let submit_multi_resp = SubmitMultiRespBuilder::default()
            .push_unsuccess_sme(UnsuccessSme::default())
            .unsuccess_sme(vec![])
            .build()
            .unwrap();

        assert_eq!(submit_multi_resp.no_unsuccess(), 0);
        assert_eq!(submit_multi_resp.unsuccess_sme(), &vec![]);
    }
}
