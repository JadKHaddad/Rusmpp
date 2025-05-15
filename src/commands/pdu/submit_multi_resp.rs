use super::Pdu;
use crate::{
    commands::types::unsuccess_sme::UnsuccessSme, tlvs::MessageSubmissionResponseTlv,
    types::COctetString,
};

crate::create! {
    @[skip_test]
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct SubmitMultiResp {
        /// This field contains the MC message ID of the submitted
        /// message. It may be used at a later stage to query the status
        /// of a message, cancel or replace the message.
        pub message_id: COctetString<1, 65>,
        /// The number of messages to destination SME addresses that
        /// were unsuccessfully submitted to the MC. This is followed by
        /// the specified number of unsuccessful SMEs, each
        /// specified in a unsuccess_sme field.
        no_unsuccess: u8,
        /// Unsuccessful SME.
        ///
        /// (Composite Field).
        @[count = no_unsuccess]
        unsuccess_sme: Vec<UnsuccessSme>,
        /// Message submission response TLVs ([`MessageSubmissionResponseTlv`])
        @[length = unchecked]
        tlvs: Vec<MessageSubmissionResponseTlv>,
    }
}

impl SubmitMultiResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        unsuccess_sme: Vec<UnsuccessSme>,
        tlvs: Vec<impl Into<MessageSubmissionResponseTlv>>,
    ) -> Self {
        let no_unsuccess = unsuccess_sme.len() as u8;

        let tlvs = tlvs.into_iter().map(Into::into).collect();

        Self {
            message_id,
            no_unsuccess,
            unsuccess_sme,
            tlvs,
        }
    }

    pub fn no_unsuccess(&self) -> u8 {
        self.no_unsuccess
    }

    pub fn unsuccess_sme(&self) -> &[UnsuccessSme] {
        &self.unsuccess_sme
    }

    pub fn set_unsuccess_sme(&mut self, unsuccess_sme: Vec<UnsuccessSme>) {
        self.unsuccess_sme = unsuccess_sme;
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn push_unsuccess_sme(&mut self, unsuccess_sme: UnsuccessSme) {
        self.unsuccess_sme.push(unsuccess_sme);
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn clear_unsuccess_sme(&mut self) {
        self.unsuccess_sme.clear();
        self.no_unsuccess = self.unsuccess_sme.len() as u8;
    }

    pub fn tlvs(&self) -> &[MessageSubmissionResponseTlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<MessageSubmissionResponseTlv>>) {
        self.tlvs = tlvs.into_iter().map(Into::into).collect();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionResponseTlv>) {
        self.tlvs.push(tlv.into());
    }

    pub fn builder() -> SubmitMultiRespBuilder {
        SubmitMultiRespBuilder::new()
    }
}

impl From<SubmitMultiResp> for Pdu {
    fn from(value: SubmitMultiResp) -> Self {
        Self::SubmitMultiResp(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitMultiRespBuilder {
    inner: SubmitMultiResp,
}

impl SubmitMultiRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn unsuccess_sme(mut self, unsuccess_sme: Vec<UnsuccessSme>) -> Self {
        self.inner.set_unsuccess_sme(unsuccess_sme);
        self
    }

    pub fn push_unsuccess_sme(mut self, unsuccess_sme: UnsuccessSme) -> Self {
        self.inner.push_unsuccess_sme(unsuccess_sme);
        self
    }

    pub fn clear_unsuccess_sme(mut self) -> Self {
        self.inner.clear_unsuccess_sme();
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<impl Into<MessageSubmissionResponseTlv>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<MessageSubmissionResponseTlv>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> SubmitMultiResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        commands::types::{
            network_error_code::ErrorCodeNetworkType, DpfResult, NetworkErrorCode, Npi, Ton,
        },
        tests::TestInstance,
        tlvs::MessageSubmissionResponseTlvValue,
        CommandStatus,
    };

    use super::*;

    impl TestInstance for SubmitMultiResp {
        fn instances() -> Vec<Self> {
            vec![
                Self::default(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .push_unsuccess_sme(UnsuccessSme::default())
                    .push_unsuccess_sme(UnsuccessSme::new(
                        Ton::International,
                        Npi::Data,
                        COctetString::from_str("1234567890").unwrap(),
                        CommandStatus::EsmeRunknownerr,
                    ))
                    .build(),
                Self::builder()
                    .message_id(COctetString::from_str("1234567890").unwrap())
                    .push_tlv(MessageSubmissionResponseTlvValue::DpfResult(
                        DpfResult::NotSet,
                    ))
                    .push_tlv(MessageSubmissionResponseTlvValue::NetworkErrorCode(
                        NetworkErrorCode::new(ErrorCodeNetworkType::Is95AccessDeniedReason, 0x01),
                    ))
                    .build(),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<SubmitMultiResp>();
    }

    #[test]
    fn count() {
        let submit_multi = SubmitMultiResp::default();

        assert_eq!(submit_multi.no_unsuccess(), 0);
        assert!(submit_multi.unsuccess_sme().is_empty());

        let submit_multi = SubmitMultiResp::builder()
            .unsuccess_sme(vec![
                UnsuccessSme::default(),
                UnsuccessSme::new(
                    Ton::International,
                    Npi::Data,
                    COctetString::from_str("1234567890").unwrap(),
                    CommandStatus::EsmeRunknownerr,
                ),
            ])
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 2);
        assert_eq!(submit_multi.unsuccess_sme().len(), 2);

        let submit_multi = SubmitMultiResp::builder()
            .push_unsuccess_sme(UnsuccessSme::default())
            .push_unsuccess_sme(UnsuccessSme::new(
                Ton::International,
                Npi::Data,
                COctetString::from_str("1234567890").unwrap(),
                CommandStatus::EsmeRunknownerr,
            ))
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 2);
        assert_eq!(submit_multi.unsuccess_sme().len(), 2);

        let submit_multi = SubmitMultiResp::builder()
            .push_unsuccess_sme(UnsuccessSme::default())
            .push_unsuccess_sme(UnsuccessSme::new(
                Ton::International,
                Npi::Data,
                COctetString::from_str("1234567890").unwrap(),
                CommandStatus::EsmeRunknownerr,
            ))
            .clear_unsuccess_sme()
            .build();

        assert_eq!(submit_multi.no_unsuccess(), 0);
        assert!(submit_multi.unsuccess_sme().is_empty());
    }
}
