use super::Pdu;
use crate::{
    commands::tlvs::tlv::{broadcast_response::BroadcastResponseTLV, TLV},
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        length::Length,
    },
    impl_length_encode, tri,
    types::c_octet_string::COctetString,
};

impl_length_encode! {
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct BroadcastSmResp {
        /// This field contains the MC message ID of the submitted
        /// message. It may be used at a later stage to perform
        /// subsequent operations on the message.
        pub message_id: COctetString<1, 65>,
        /// Broadcast response TLVs ([`BroadcastResponseTLV`]).
        tlvs: Vec<TLV>,
    }
}

impl BroadcastSmResp {
    pub fn new(message_id: COctetString<1, 65>, tlvs: Vec<BroadcastResponseTLV>) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        Self { message_id, tlvs }
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<BroadcastResponseTLV>) {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        self.tlvs = tlvs;
    }

    pub fn push_tlv(&mut self, tlv: BroadcastResponseTLV) {
        let tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> BroadcastSmRespBuilder {
        BroadcastSmRespBuilder::new()
    }

    pub fn into_broadcast_sm_resp(self) -> Pdu {
        Pdu::BroadcastSmResp(self)
    }
}

impl DecodeWithLength for BroadcastSmResp {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let message_id = tri!(COctetString::<1, 65>::decode_from(reader));

        let tlvs_length = length.saturating_sub(message_id.length());

        let tlvs = tri!(Vec::<TLV>::decode_from(reader, tlvs_length));

        Ok(Self { message_id, tlvs })
    }
}

#[derive(Debug, Default)]
pub struct BroadcastSmRespBuilder {
    inner: BroadcastSmResp,
}

impl BroadcastSmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<BroadcastResponseTLV>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: BroadcastResponseTLV) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> BroadcastSmResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<BroadcastSmResp>();
    }
}
