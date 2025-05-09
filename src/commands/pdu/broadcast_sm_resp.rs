use super::Pdu;
use crate::{
    commands::tlvs::tlv::{broadcast_response::BroadcastResponseTLV, TLV},
    types::c_octet_string::COctetString,
};

crate::create! {
    #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct BroadcastSmResp {
        /// This field contains the MC message ID of the submitted
        /// message. It may be used at a later stage to perform
        /// subsequent operations on the message.
        pub message_id: COctetString<1, 65>,
        /// Broadcast response TLVs ([`BroadcastResponseTLV`]).
        @[length = unchecked]
        tlvs: Vec<TLV>,
    }
}

impl BroadcastSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        tlvs: Vec<impl Into<BroadcastResponseTLV>>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<TLV>>();

        Self { message_id, tlvs }
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<BroadcastResponseTLV>>) {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<TLV>>();

        self.tlvs = tlvs;
    }

    pub fn push_tlv(&mut self, tlv: impl Into<BroadcastResponseTLV>) {
        let tlv: BroadcastResponseTLV = tlv.into();
        let tlv: TLV = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> BroadcastSmRespBuilder {
        BroadcastSmRespBuilder::new()
    }
}

impl From<BroadcastSmResp> for Pdu {
    fn from(value: BroadcastSmResp) -> Self {
        Self::BroadcastSmResp(value)
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

    pub fn tlvs(mut self, tlvs: Vec<impl Into<BroadcastResponseTLV>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<BroadcastResponseTLV>) -> Self {
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
