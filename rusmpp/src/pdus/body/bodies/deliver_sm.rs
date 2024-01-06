use super::s_sm::SSm;
use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::tlvs::tlv::{MessageDeliveryRequestTLV, TLV},
};
use derive_builder::Builder;
use getset::{Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Default,
    Getters,
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
pub struct DeliverSm {
    #[getset(get = "pub")]
    #[builder(setter(custom))]
    ssm: SSm,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
    tlvs: Vec<TLV>,
}

impl DeliverSm {
    pub fn new(ssm: SSm, tlvs: Vec<MessageDeliveryRequestTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();

        let mut ssm = ssm;
        ssm.check_for_message_payload_and_clear_short_message(&tlvs);

        Self { ssm, tlvs }
    }

    /// Sets the ssm and updates it accordingly.
    /// Short message and message length will be updated if a message payload is present.
    pub fn set_ssm(&mut self, ssm: SSm) {
        self.ssm = ssm;
        self.ssm
            .check_for_message_payload_and_clear_short_message(&self.tlvs);
    }

    /// Sets the TLVs and updates the ssm accordingly.
    /// Short message and message length will be updated if a message payload is present.
    pub fn set_tlvs(&mut self, tlvs: Vec<MessageDeliveryRequestTLV>) {
        self.tlvs = tlvs.into_iter().map(|v| v.into()).collect();
        self.ssm
            .check_for_message_payload_and_clear_short_message(&self.tlvs);
    }

    /// Pushes a TLV and updates the ssm accordingly.
    /// Short message and message length will be updated if a message payload is present.
    pub fn push_tlv(&mut self, tlv: MessageDeliveryRequestTLV) {
        self.tlvs.push(tlv.into());
        self.ssm
            .check_for_message_payload_and_clear_short_message(&self.tlvs);
    }
}

impl DeliverSmBuilder {
    pub fn ssm(&mut self, ssm: SSm) -> &mut Self {
        let mut ssm = ssm;

        if let Some(tlvs) = &self.tlvs {
            ssm.check_for_message_payload_and_clear_short_message(tlvs);
        }

        self.ssm = Some(ssm);

        self
    }

    pub fn tlvs(&mut self, tlvs: Vec<MessageDeliveryRequestTLV>) -> &mut Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();

        if let Some(ssm) = &mut self.ssm {
            ssm.check_for_message_payload_and_clear_short_message(&tlvs);
        }

        self.tlvs = Some(tlvs);

        self
    }

    pub fn push_tlv(&mut self, tlv: MessageDeliveryRequestTLV) -> &mut Self {
        let self_tlvs = self.tlvs.get_or_insert_with(Vec::new);
        self_tlvs.push(tlv.into());

        if let Some(ssm) = &mut self.ssm {
            ssm.check_for_message_payload_and_clear_short_message(self_tlvs);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rusmpp_io::types::{
        no_fixed_size_octet_string::NoFixedSizeOctetString, octet_string::OctetString,
    };

    use super::*;
    use crate::{
        pdus::{body::bodies::s_sm::SSmBuilder, tlvs::tlv_value::MessageDeliveryRequestTLVValue},
        test_utils::defaut_write_read_with_length_compare,
    };

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<DeliverSm>().await;
    }

    #[test]
    fn set_short_message() {
        let mut deliver_sm = DeliverSm::default();

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(deliver_sm.ssm().short_message(), &OctetString::empty());
        assert!(deliver_sm.tlvs().is_empty());

        deliver_sm.set_ssm(SSmBuilder::default().build().unwrap());

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(deliver_sm.ssm().short_message(), &OctetString::empty());
        assert!(deliver_sm.tlvs().is_empty());

        deliver_sm.set_ssm(
            SSmBuilder::default()
                .short_message(OctetString::from_str("hello").unwrap())
                .build()
                .unwrap(),
        );

        assert_eq!(deliver_sm.ssm().sm_length(), 5);
        assert_eq!(
            deliver_sm.ssm().short_message(),
            &OctetString::from_str("hello").unwrap()
        );
        assert!(deliver_sm.tlvs().is_empty());

        deliver_sm.push_tlv(MessageDeliveryRequestTLV::new(
            MessageDeliveryRequestTLVValue::MessagePayload(
                NoFixedSizeOctetString::from_str("hello").unwrap(),
            ),
        ));

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(deliver_sm.ssm().short_message(), &OctetString::empty());
        assert_eq!(deliver_sm.tlvs().len(), 1);

        deliver_sm.set_ssm(
            SSmBuilder::default()
                .short_message(OctetString::from_str("hello").unwrap())
                .build()
                .unwrap(),
        );

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(
            deliver_sm.ssm().short_message(),
            &OctetString::from_str("").unwrap()
        );
        assert_eq!(deliver_sm.tlvs().len(), 1);

        deliver_sm.set_tlvs(vec![]);

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(
            deliver_sm.ssm().short_message(),
            &OctetString::from_str("").unwrap()
        );
        assert!(deliver_sm.tlvs().is_empty());

        deliver_sm.set_ssm(
            SSmBuilder::default()
                .short_message(OctetString::from_str("hello").unwrap())
                .build()
                .unwrap(),
        );

        assert_eq!(deliver_sm.ssm().sm_length(), 5);
        assert_eq!(
            deliver_sm.ssm().short_message(),
            &OctetString::from_str("hello").unwrap()
        );
        assert!(deliver_sm.tlvs().is_empty());
    }

    #[test]
    fn builder_set_short_message() {
        let deliver_sm = DeliverSmBuilder::default().build().unwrap();

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(deliver_sm.ssm().short_message(), &OctetString::empty());
        assert!(deliver_sm.tlvs().is_empty());

        let deliver_sm = DeliverSmBuilder::default()
            .ssm(SSmBuilder::default().build().unwrap())
            .build()
            .unwrap();

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(deliver_sm.ssm().short_message(), &OctetString::empty());
        assert!(deliver_sm.tlvs().is_empty());

        let deliver_sm = DeliverSmBuilder::default()
            .ssm(
                SSmBuilder::default()
                    .short_message(OctetString::from_str("hello").unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(deliver_sm.ssm().sm_length(), 5);
        assert_eq!(
            deliver_sm.ssm().short_message(),
            &OctetString::from_str("hello").unwrap()
        );
        assert!(deliver_sm.tlvs().is_empty());

        let deliver_sm = DeliverSmBuilder::default()
            .ssm(
                SSmBuilder::default()
                    .short_message(OctetString::from_str("hello").unwrap())
                    .build()
                    .unwrap(),
            )
            .tlvs(vec![MessageDeliveryRequestTLV::new(
                MessageDeliveryRequestTLVValue::MessagePayload(
                    NoFixedSizeOctetString::from_str("hello").unwrap(),
                ),
            )])
            .build()
            .unwrap();

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(deliver_sm.ssm().short_message(), &OctetString::empty());
        assert_eq!(deliver_sm.tlvs().len(), 1);

        let deliver_sm = DeliverSmBuilder::default()
            .ssm(
                SSmBuilder::default()
                    .short_message(OctetString::from_str("hello").unwrap())
                    .build()
                    .unwrap(),
            )
            .push_tlv(MessageDeliveryRequestTLV::new(
                MessageDeliveryRequestTLVValue::MessagePayload(
                    NoFixedSizeOctetString::from_str("hello").unwrap(),
                ),
            ))
            .build()
            .unwrap();

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(deliver_sm.ssm().short_message(), &OctetString::empty());
        assert_eq!(deliver_sm.tlvs().len(), 1);

        let deliver_sm = DeliverSmBuilder::default()
            .ssm(
                SSmBuilder::default()
                    .short_message(OctetString::from_str("hello").unwrap())
                    .build()
                    .unwrap(),
            )
            .tlvs(vec![])
            .build()
            .unwrap();

        assert_eq!(deliver_sm.ssm().sm_length(), 5);
        assert_eq!(
            deliver_sm.ssm().short_message(),
            &OctetString::from_str("hello").unwrap()
        );
        assert!(deliver_sm.tlvs().is_empty());

        let deliver_sm = DeliverSmBuilder::default()
            .ssm(
                SSmBuilder::default()
                    .short_message(OctetString::from_str("hello").unwrap())
                    .build()
                    .unwrap(),
            )
            .push_tlv(MessageDeliveryRequestTLV::new(
                MessageDeliveryRequestTLVValue::MessagePayload(
                    NoFixedSizeOctetString::from_str("hello").unwrap(),
                ),
            ))
            .ssm(
                SSmBuilder::default()
                    .short_message(OctetString::from_str("hello").unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(deliver_sm.ssm().sm_length(), 0);
        assert_eq!(
            deliver_sm.ssm().short_message(),
            &OctetString::from_str("").unwrap()
        );
        assert_eq!(deliver_sm.tlvs().len(), 1);
    }
}
