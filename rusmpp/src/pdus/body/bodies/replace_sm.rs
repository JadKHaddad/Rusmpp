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
use getset::{CopyGetters, Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoWrite};

#[derive(
    Default,
    Getters,
    CopyGetters,
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
pub struct ReplaceSm {
    #[getset(get = "pub", set = "pub")]
    message_id: COctetString<1, 65>,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_ton: Ton,
    #[getset(get_copy = "pub", set = "pub")]
    source_addr_npi: Npi,
    #[getset(get = "pub", set = "pub")]
    source_addr: COctetString<1, 21>,
    #[getset(get = "pub", set = "pub")]
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    #[getset(get = "pub", set = "pub")]
    validity_period: EmptyOrFullCOctetString<17>,
    #[getset(get_copy = "pub", set = "pub")]
    registered_delivery: RegisteredDelivery,
    #[getset(get_copy = "pub", set = "pub")]
    sm_default_msg_id: u8,
    #[getset(get_copy = "pub")]
    #[builder(setter(custom))]
    sm_length: u8,
    #[getset(get = "pub")]
    #[rusmpp_io_read(length=(sm_length))]
    #[builder(setter(custom))]
    short_message: OctetString<0, 255>,
    #[getset(get = "pub")]
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

        let short_message = if message_payload.is_some() {
            OctetString::empty()
        } else {
            short_message
        };

        let sm_length = short_message.length() as u8;

        Self {
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
        }
    }

    /// Sets the short message and short message length and clears the message payload.
    /// Always clears the message payload.
    pub fn set_short_message(&mut self, short_message: OctetString<0, 255>) {
        self.sm_length = short_message.length() as u8;
        self.short_message = short_message;
        self.message_payload = None;
    }

    /// Sets the message payload.
    /// If Some, the short message and short message length are cleared.
    /// If None, the short message and short message length will not be touched.
    pub fn set_message_payload(&mut self, message_payload: Option<NoFixedSizeOctetString>) {
        self.message_payload = message_payload.map(|v| TLV::new(TLVValue::MessagePayload(v)));

        if self.message_payload.is_some() {
            self.short_message = OctetString::empty();
            self.sm_length = 0;
        };
    }
}

impl ReplaceSmBuilder {
    pub fn short_message(&mut self, short_message: OctetString<0, 255>) -> &mut Self {
        self.sm_length = Some(short_message.length() as u8);
        self.short_message = Some(short_message);
        self.message_payload = None;
        self
    }

    pub fn message_payload(
        &mut self,
        message_payload: Option<NoFixedSizeOctetString>,
    ) -> &mut Self {
        if self.message_payload.is_some() {
            self.short_message = None;
            self.sm_length = None;
        };

        self.message_payload = message_payload
            .map(|v| TLV::new(TLVValue::MessagePayload(v)))
            .into();

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

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 5);
        assert_eq!(
            replace_sm.short_message(),
            &OctetString::from_str("hello").unwrap()
        );

        replace_sm.set_message_payload(None);

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 5);
        assert_eq!(
            replace_sm.short_message(),
            &OctetString::from_str("hello").unwrap()
        );

        replace_sm.set_message_payload(Some(NoFixedSizeOctetString::from_str("hello").unwrap()));
        replace_sm.set_short_message(OctetString::from_str("").unwrap());

        assert!(replace_sm.message_payload().is_none());
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

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 5);
        assert_eq!(
            replace_sm.short_message(),
            &OctetString::from_str("hello").unwrap()
        );

        let replace_sm = ReplaceSmBuilder::default()
            .message_payload(Some(NoFixedSizeOctetString::from_str("hello").unwrap()))
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

        assert!(replace_sm.message_payload().is_none());
        assert_eq!(replace_sm.sm_length(), 0);
        assert_eq!(replace_sm.short_message(), &OctetString::empty());
    }
}
