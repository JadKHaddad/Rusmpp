use super::Pdu;
use crate::{
    commands::tlvs::tlv::{message_delivery_response::MessageDeliveryResponseTLV, TLV},
    types::COctetString,
};

macro_rules! declare_sm_resp {
    ($name:ident, $builder_name:ident) => {
        crate::create! {
            #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct $name {
                /// This field contains the MC message ID of the submitted message.
                /// It may be used at a later stage to query the status of a message,
                /// cancel or replace the message.
                message_id: COctetString<1, 65>,
                /// Message delivery response TLVs ([`MessageDeliveryResponseTLV`])
                @[length = unchecked]
                tlvs: Vec<TLV>,
            }
        }

        impl $name {
            pub fn new(
                message_id: COctetString<1, 65>,
                tlvs: Vec<impl Into<MessageDeliveryResponseTLV>>,
            ) -> Self {
                let tlvs = tlvs.into_iter().map(Into::into).map(From::from).collect();

                Self { message_id, tlvs }
            }

            pub fn message_id(&self) -> &COctetString<1, 65> {
                &self.message_id
            }

            pub fn tlvs(&self) -> &[TLV] {
                &self.tlvs
            }

            pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<MessageDeliveryResponseTLV>>) {
                let tlvs = tlvs
                    .into_iter()
                    .map(Into::into)
                    .map(From::from)
                    .collect::<Vec<TLV>>();

                self.tlvs = tlvs;
            }

            pub fn push_tlv(&mut self, tlv: impl Into<MessageDeliveryResponseTLV>) {
                let tlv: MessageDeliveryResponseTLV = tlv.into();
                let tlv: TLV = tlv.into();

                self.tlvs.push(tlv);
            }

            pub fn builder() -> $builder_name {
                $builder_name::new()
            }
        }

        #[derive(Debug, Default)]
        pub struct $builder_name {
            inner: $name,
        }

        impl $builder_name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
                self.inner.message_id = message_id;
                self
            }

            pub fn tlvs(mut self, tlvs: Vec<impl Into<MessageDeliveryResponseTLV>>) -> Self {
                self.inner.set_tlvs(tlvs);
                self
            }

            pub fn push_tlv(mut self, tlv: impl Into<MessageDeliveryResponseTLV>) -> Self {
                self.inner.push_tlv(tlv);
                self
            }

            pub fn build(self) -> $name {
                self.inner
            }
        }
    };
}

declare_sm_resp!(DeliverSmResp, DeliverSmRespBuilder);
declare_sm_resp!(DataSmResp, DataSmRespBuilder);

impl From<DeliverSmResp> for Pdu {
    fn from(value: DeliverSmResp) -> Self {
        Self::DeliverSmResp(value)
    }
}

impl From<DataSmResp> for Pdu {
    fn from(value: DataSmResp) -> Self {
        Self::DataSmResp(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<DeliverSmResp>();
        crate::ende::tests::default_encode_decode_with_length::<DataSmResp>();
    }
}
