use super::Pdu;
use crate::{
    commands::{
        tlvs::tlv::{message_submission_response::MessageSubmissionResponseTlv, Tlv},
        types::unsuccess_sme::UnsuccessSme,
    },
    types::COctetString,
};

crate::create! {
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
        /// Message submission response TLVs ([`MessageSubmissionResponseTLV`])
        @[length = unchecked]
        tlvs: Vec<Tlv>,
    }
}

impl SubmitMultiResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        unsuccess_sme: Vec<UnsuccessSme>,
        tlvs: Vec<impl Into<MessageSubmissionResponseTlv>>,
    ) -> Self {
        let no_unsuccess = unsuccess_sme.len() as u8;

        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

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
        self.no_unsuccess = unsuccess_sme.len() as u8;
        self.unsuccess_sme = unsuccess_sme;
    }

    pub fn push_unsuccess_sme(&mut self, unsuccess_sme: UnsuccessSme) {
        self.no_unsuccess += 1;
        self.unsuccess_sme.push(unsuccess_sme);
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<MessageSubmissionResponseTlv>>) {
        self.tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionResponseTlv>) {
        let tlv: MessageSubmissionResponseTlv = tlv.into();
        let tlv: Tlv = tlv.into();

        self.tlvs.push(tlv);
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
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<SubmitMultiResp>();
    }
}
