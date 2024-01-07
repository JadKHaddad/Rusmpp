use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    pdus::{
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{npi::Npi, registered_delivery::RegisteredDelivery, ton::Ton},
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        no_fixed_size_octet_string::NoFixedSizeOctetString, octet_string::OctetString,
    },
};
use derive_builder::Builder;
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Default,
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
/// Short message is always superceded by the message payload.
/// Clear message payload to use the short message.
pub struct ReplaceSm {
    pub message_id: COctetString<1, 65>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: COctetString<1, 21>,
    pub schedule_delivery_time: EmptyOrFullCOctetString<17>,
    pub validity_period: EmptyOrFullCOctetString<17>,
    pub registered_delivery: RegisteredDelivery,
    pub sm_default_msg_id: u8,
    #[builder(setter(custom))]
    sm_length: u8,
    #[rusmpp_io_read(length=(sm_length))]
    #[builder(setter(custom))]
    short_message: OctetString<0, 255>,
    #[rusmpp_io_read(length=(length - all_before))]
    #[builder(setter(custom))]
    message_payload: Option<TLV>,
}

impl ReplaceSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        message_payload: Option<NoFixedSizeOctetString>,
    ) -> Self {
        let message_payload = message_payload.map(|v| TLV::new(TLVValue::MessagePayload(v)));
        let sm_length = short_message.length() as u8;

        let mut replace_sm = Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            sm_length,
            short_message,
            message_payload,
        };

        replace_sm.clear_short_message_if_message_payload_exists();

        replace_sm
    }

    pub fn sm_length(&self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
    }

    /// Sets the short message and short message length.
    /// Updates the short message and short message length accordingly.
    /// Has no effect if the message payload is set.
    /// Returns true if the short message and short message length were set.
    pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) -> bool {
        self.sm_length = short_message.length() as u8;
        self.short_message = short_message;

        !self.clear_short_message_if_message_payload_exists()
    }

    pub fn message_payload(&self) -> Option<&TLV> {
        self.message_payload.as_ref()
    }

    /// Sets the message payload.
    /// Updates the short message and short message length accordingly.
    pub fn set_message_payload(&mut self, message_payload: Option<NoFixedSizeOctetString>) {
        self.message_payload = message_payload.map(|v| TLV::new(TLVValue::MessagePayload(v)));

        self.clear_short_message_if_message_payload_exists();
    }

    /// Clears the short message and short message length if the message payload is set.
    /// Returns true if the short message and short message length were cleared.
    fn clear_short_message_if_message_payload_exists(&mut self) -> bool {
        if self.message_payload.is_some() {
            self.short_message = OctetString::empty();
            self.sm_length = 0;

            return true;
        };

        false
    }
}

impl ReplaceSmBuilder {
    /// Clears the short message and short message length if the message payload is set.
    fn clear_short_message_if_message_payload_exists(&mut self) {
        if let Some(ref message_payload) = self.message_payload {
            if message_payload.is_some() {
                self.short_message = None;
                self.sm_length = None;
            };
        }
    }

    /// Sets the short message and short message length.
    /// Updates the short message and short message length accordingly.
    /// Has no effect if the message payload is set.
    pub fn short_message(&mut self, short_message: OctetString<0, 255>) -> &mut Self {
        self.sm_length = Some(short_message.length() as u8);
        self.short_message = Some(short_message);

        self.clear_short_message_if_message_payload_exists();
        self
    }

    /// Sets the message payload.
    /// Updates the short message and short message length accordingly.
    pub fn message_payload(
        &mut self,
        message_payload: Option<NoFixedSizeOctetString>,
    ) -> &mut Self {
        self.message_payload = message_payload
            .map(|v| TLV::new(TLVValue::MessagePayload(v)))
            .into();

        self.clear_short_message_if_message_payload_exists();
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::test_utils::defaut_write_read_with_length_compare;

    #[tokio::test]
    async fn write_read_compare() {
        defaut_write_read_with_length_compare::<ReplaceSm>().await;
    }

    #[test]
    fn set_short_message() {
        let mut replace_sm = ReplaceSm::default();

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());

        replace_sm.set_short_message(OctetString::from_str("hello").unwrap());

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 5);
        assert_eq!(
            replace_sm.short_message(),
            &OctetString::from_str("hello").unwrap()
        );

        replace_sm.set_message_payload(Some(NoFixedSizeOctetString::from_str("hello").unwrap()));

        assert!(replace_sm.message_payload().is_some());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());

        replace_sm.set_short_message(OctetString::from_str("hello").unwrap());

        assert!(replace_sm.message_payload().is_some());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());

        replace_sm.set_message_payload(None);

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());

        replace_sm.set_message_payload(Some(NoFixedSizeOctetString::from_str("hello").unwrap()));

        assert!(replace_sm.message_payload().is_some());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());
    }

    #[test]
    fn builder_set_short_message() {
        let replace_sm = ReplaceSmBuilder::default().build().unwrap();

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());

        let replace_sm = ReplaceSmBuilder::default()
            .short_message(OctetString::from_str("hello").unwrap())
            .build()
            .unwrap();

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 5);
        assert_eq!(
            replace_sm.short_message(),
            &OctetString::from_str("hello").unwrap()
        );

        let replace_sm = ReplaceSmBuilder::default()
            .message_payload(Some(NoFixedSizeOctetString::from_str("hello").unwrap()))
            .short_message(OctetString::from_str("hello").unwrap())
            .build()
            .unwrap();

        assert!(replace_sm.message_payload().is_some());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());

        let replace_sm = ReplaceSmBuilder::default()
            .message_payload(Some(NoFixedSizeOctetString::from_str("hello").unwrap()))
            .short_message(OctetString::from_str("hello").unwrap())
            .message_payload(None)
            .build()
            .unwrap();

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());

        let replace_sm = ReplaceSmBuilder::default()
            .short_message(OctetString::from_str("hello").unwrap())
            .message_payload(None)
            .build()
            .unwrap();

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 5);
        assert_eq!(
            replace_sm.short_message(),
            &OctetString::from_str("hello").unwrap()
        );

        let replace_sm = ReplaceSmBuilder::default()
            .message_payload(Some(NoFixedSizeOctetString::from_str("hello").unwrap()))
            .short_message(OctetString::from_str("").unwrap())
            .build()
            .unwrap();

        assert!(replace_sm.message_payload().is_some());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());
    }
}
