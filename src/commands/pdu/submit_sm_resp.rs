use super::Pdu;
use crate::{
    tlvs::{MessageSubmissionResponseTlv, Tlv},
    types::COctetString,
};

crate::create! {
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct SubmitSmResp {
        /// This field contains the MC message ID of the submitted message.
        /// It may be used at a later stage to query the status of a message,
        /// cancel or replace the message.
        message_id: COctetString<1, 65>,
        /// Message submission response TLVs ([`MessageSubmissionResponseTLV`])
        @[length = unchecked]
        tlvs: Vec<Tlv>,
    }
}

impl SubmitSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        tlvs: Vec<impl Into<MessageSubmissionResponseTlv>>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

        Self { message_id, tlvs }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<MessageSubmissionResponseTlv>>) {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        self.tlvs = tlvs;
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<MessageSubmissionResponseTlv>) {
        let tlv: MessageSubmissionResponseTlv = tlv.into();
        let tlv: Tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> SubmitSmRespBuilder {
        SubmitSmRespBuilder::new()
    }
}

impl From<SubmitSmResp> for Pdu {
    fn from(value: SubmitSmResp) -> Self {
        Self::SubmitSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct SubmitSmRespBuilder {
    inner: SubmitSmResp,
}

impl SubmitSmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
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

    pub fn build(self) -> SubmitSmResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<SubmitSmResp>();
    }
}
