crate::create! {
    @[repr = u8]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct MsMsgWaitFacilities {
        pub indicator: Indicator,
        pub type_of_message: TypeOfMessage,
    }
}

impl MsMsgWaitFacilities {
    pub fn new(indicator: Indicator, type_of_message: TypeOfMessage) -> Self {
        Self {
            indicator,
            type_of_message,
        }
    }
}

impl From<u8> for MsMsgWaitFacilities {
    fn from(value: u8) -> Self {
        Self {
            indicator: Indicator::from(value & 0b10000000),
            type_of_message: TypeOfMessage::from(value & 0b00000011),
        }
    }
}

impl From<MsMsgWaitFacilities> for u8 {
    fn from(value: MsMsgWaitFacilities) -> Self {
        u8::from(value.indicator) | u8::from(value.type_of_message)
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum Indicator {
        #[default]
        Inactive = 0b00000000,
        Active = 0b10000000,
        Other(u8),
    }
}

impl From<u8> for Indicator {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Indicator::Inactive,
            0b10000000 => Indicator::Active,
            value => Indicator::Other(value),
        }
    }
}

impl From<Indicator> for u8 {
    fn from(value: Indicator) -> Self {
        match value {
            Indicator::Inactive => 0b00000000,
            Indicator::Active => 0b10000000,
            Indicator::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum TypeOfMessage {
        #[default]
        VoicemailMessageWaiting = 0b00000000,
        FaxMessageWaiting = 0b00000001,
        ElectronicMailMessageWaiting = 0b00000010,
        OtherMessageWaiting = 0b00000011,
        Other(u8),
    }
}

impl From<u8> for TypeOfMessage {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => TypeOfMessage::VoicemailMessageWaiting,
            0b00000001 => TypeOfMessage::FaxMessageWaiting,
            0b00000010 => TypeOfMessage::ElectronicMailMessageWaiting,
            0b00000011 => TypeOfMessage::OtherMessageWaiting,
            value => TypeOfMessage::Other(value),
        }
    }
}

impl From<TypeOfMessage> for u8 {
    fn from(value: TypeOfMessage) -> Self {
        match value {
            TypeOfMessage::VoicemailMessageWaiting => 0b00000000,
            TypeOfMessage::FaxMessageWaiting => 0b00000001,
            TypeOfMessage::ElectronicMailMessageWaiting => 0b00000010,
            TypeOfMessage::OtherMessageWaiting => 0b00000011,
            TypeOfMessage::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_u8() {
        let ms_message_wait_facilities = MsMsgWaitFacilities::new(
            Indicator::Active,
            TypeOfMessage::ElectronicMailMessageWaiting,
        );

        assert_eq!(u8::from(ms_message_wait_facilities), 0b10000010);
    }

    #[test]
    fn from_u8() {
        let ms_message_wait_facilities = MsMsgWaitFacilities::from(0b10000010);

        assert_eq!(ms_message_wait_facilities.indicator, Indicator::Active);
        assert_eq!(
            ms_message_wait_facilities.type_of_message,
            TypeOfMessage::ElectronicMailMessageWaiting
        );
    }

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<MsMsgWaitFacilities>();
        crate::tests::encode_decode_test_instances::<Indicator>();
        crate::tests::encode_decode_test_instances::<TypeOfMessage>();
    }
}
