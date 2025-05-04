use super::Pdu;
use crate::{
    commands::{
        tlvs::tlv::{message_submission_response::MessageSubmissionResponseTLV, TLV},
        types::unsuccess_sme::UnsuccessSme,
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        length::Length,
    },
    impl_length_encode, tri,
    types::c_octet_string::COctetString,
};

impl_length_encode! {
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
        unsuccess_sme: Vec<UnsuccessSme>,
        /// Message submission response TLVs ([`MessageSubmissionResponseTLV`])
        tlvs: Vec<TLV>,
    }
}

impl SubmitMultiResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        unsuccess_sme: Vec<UnsuccessSme>,
        tlvs: Vec<MessageSubmissionResponseTLV>,
    ) -> Self {
        let no_unsuccess = unsuccess_sme.len() as u8;

        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

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

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageSubmissionResponseTLV>) {
        self.tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();
    }

    pub fn push_tlv(&mut self, tlv: MessageSubmissionResponseTLV) {
        self.tlvs.push(tlv.into());
    }

    pub fn builder() -> SubmitMultiRespBuilder {
        SubmitMultiRespBuilder::new()
    }

    pub fn into_submit_multi_resp(self) -> Pdu {
        Pdu::SubmitMultiResp(self)
    }
}

impl DecodeWithLength for SubmitMultiResp {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let message_id = tri!(COctetString::<1, 65>::decode_from(reader));
        let no_unsuccess = tri!(u8::decode_from(reader));
        let unsuccess_sme = tri!(UnsuccessSme::vectorized_decode_from(
            reader,
            no_unsuccess as usize
        ));

        let tlvs_length = length
            .saturating_sub(message_id.length())
            .saturating_sub(no_unsuccess.length())
            .saturating_sub(unsuccess_sme.length());

        let tlvs = tri!(Vec::<TLV>::decode_from(reader, tlvs_length));

        Ok(Self {
            message_id,
            no_unsuccess,
            unsuccess_sme,
            tlvs,
        })
    }
}

#[derive(Default)]
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

    pub fn tlvs(mut self, tlvs: Vec<MessageSubmissionResponseTLV>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: MessageSubmissionResponseTLV) -> Self {
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
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<SubmitMultiResp>();
    }
}
