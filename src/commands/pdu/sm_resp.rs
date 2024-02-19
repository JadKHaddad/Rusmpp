use super::Pdu;
use crate::{
    commands::tlvs::tlv::{message_delivery_response::MessageDeliveryResponseTLV, TLV},
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::c_octet_string::COctetString,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SmResp {
    /// This field contains the MC message ID of the submitted message.
    /// It may be used at a later stage to query the status of a message,
    /// cancel or replace the message.
    message_id: COctetString<1, 65>,
    /// Message delivery response TLVs ([`MessageDeliveryResponseTLV`])
    tlvs: Vec<TLV>,
}

impl SmResp {
    pub fn new(message_id: COctetString<1, 65>, tlvs: Vec<MessageDeliveryResponseTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|value| value.into()).collect();

        Self { message_id, tlvs }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<MessageDeliveryResponseTLV>) {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        self.tlvs = tlvs;
    }

    pub fn push_tlv(&mut self, tlv: MessageDeliveryResponseTLV) {
        let tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> SmRespBuilder {
        SmRespBuilder::new()
    }

    pub fn into_deliver_sm_resp(self) -> Pdu {
        Pdu::DeliverSmResp(self)
    }

    pub fn into_data_sm_resp(self) -> Pdu {
        Pdu::DataSmResp(self)
    }
}

impl Length for SmResp {
    fn length(&self) -> usize {
        self.message_id.length() + self.tlvs.length()
    }
}

impl Encode for SmResp {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.message_id.encode_to(writer));
        tri!(self.tlvs.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for SmResp {
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

#[derive(Default)]
pub struct SmRespBuilder {
    inner: SmResp,
}

impl SmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<MessageDeliveryResponseTLV>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: MessageDeliveryResponseTLV) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> SmResp {
        self.inner
    }
}
