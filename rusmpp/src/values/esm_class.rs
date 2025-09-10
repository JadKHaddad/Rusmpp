crate::create! {
    @[repr = u8]
    @[skip_test]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct EsmClass {
        pub messaging_mode: MessagingMode,
        pub message_type: MessageType,
        pub ansi41_specific: Ansi41Specific,
        pub gsm_features: GsmFeatures,
    }
}

impl EsmClass {
    pub fn new(
        messaging_mode: MessagingMode,
        message_type: MessageType,
        ansi41_specific: Ansi41Specific,
        gsm_features: GsmFeatures,
    ) -> Self {
        Self {
            messaging_mode,
            message_type,
            ansi41_specific,
            gsm_features,
        }
    }
}

impl From<u8> for EsmClass {
    fn from(value: u8) -> Self {
        Self {
            messaging_mode: MessagingMode::from(value & 0b00_00_00_11),
            message_type: MessageType::from(value & 0b00_10_01_00),
            ansi41_specific: Ansi41Specific::from(value & 0b00_01_10_00),
            gsm_features: GsmFeatures::from(value & 0b11_00_00_00),
        }
    }
}

impl From<EsmClass> for u8 {
    fn from(value: EsmClass) -> Self {
        u8::from(value.messaging_mode)
            | u8::from(value.message_type)
            | u8::from(value.ansi41_specific)
            | u8::from(value.gsm_features)
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum MessagingMode {
        #[default]
        Default = 0b00_00_00_00,
        Datagram = 0b00_00_00_01,
        Forward = 0b00_00_00_10,
        StoreAndForward = 0b00_00_00_11,
        Other(u8),
    }
}

impl From<u8> for MessagingMode {
    fn from(value: u8) -> Self {
        match value {
            0b00_00_00_00 => MessagingMode::Default,
            0b00_00_00_01 => MessagingMode::Datagram,
            0b00_00_00_10 => MessagingMode::Forward,
            0b00_00_00_11 => MessagingMode::StoreAndForward,
            _ => MessagingMode::Other(value),
        }
    }
}

impl From<MessagingMode> for u8 {
    fn from(value: MessagingMode) -> Self {
        match value {
            MessagingMode::Default => 0b00_00_00_00,
            MessagingMode::Datagram => 0b00_00_00_01,
            MessagingMode::Forward => 0b00_00_00_10,
            MessagingMode::StoreAndForward => 0b00_00_00_11,
            MessagingMode::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum MessageType {
        #[default]
        Default = 0b00_00_00_00,
        ShortMessageContainsMCDeliveryReceipt = 0b00_00_01_00,
        ShortMessageContainsIntermediateDeliveryNotification = 0b00_10_00_00,
        Other(u8),
    }
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0b00_00_00_00 => MessageType::Default,
            0b00_00_01_00 => MessageType::ShortMessageContainsMCDeliveryReceipt,
            0b00_10_00_00 => MessageType::ShortMessageContainsIntermediateDeliveryNotification,
            _ => MessageType::Other(value),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        match value {
            MessageType::Default => 0b00_00_00_00,
            MessageType::ShortMessageContainsMCDeliveryReceipt => 0b00_00_01_00,
            MessageType::ShortMessageContainsIntermediateDeliveryNotification => 0b00_10_00_00,
            MessageType::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum Ansi41Specific {
        #[default]
        ShortMessageContainsDeliveryAcknowledgement = 0b00_00_10_00,
        ShortMessageContainsUserAcknowledgment = 0b00_01_00_00,
        ShortMessageContainsConversationAbort = 0b00_01_10_00,
        Other(u8),
    }
}

impl From<u8> for Ansi41Specific {
    fn from(value: u8) -> Self {
        match value {
            0b00_00_10_00 => Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement,
            0b00_01_00_00 => Ansi41Specific::ShortMessageContainsUserAcknowledgment,
            0b00_01_10_00 => Ansi41Specific::ShortMessageContainsConversationAbort,
            _ => Ansi41Specific::Other(value),
        }
    }
}

impl From<Ansi41Specific> for u8 {
    fn from(value: Ansi41Specific) -> Self {
        match value {
            Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement => 0b00_00_10_00,
            Ansi41Specific::ShortMessageContainsUserAcknowledgment => 0b00_01_00_00,
            Ansi41Specific::ShortMessageContainsConversationAbort => 0b00_01_10_00,
            Ansi41Specific::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum GsmFeatures {
        #[default]
        NotSelected = 0b00_00_00_00,
        UdhiIndicator = 0b01_00_00_00,
        SetReplyPath = 0b10_00_00_00,
        SetUdhiAndReplyPath = 0b11_00_00_00,
        Other(u8),
    }
}

impl From<u8> for GsmFeatures {
    fn from(value: u8) -> Self {
        match value {
            0b00_00_00_00 => GsmFeatures::NotSelected,
            0b01_00_00_00 => GsmFeatures::UdhiIndicator,
            0b10_00_00_00 => GsmFeatures::SetReplyPath,
            0b11_00_00_00 => GsmFeatures::SetUdhiAndReplyPath,
            _ => GsmFeatures::Other(value),
        }
    }
}

impl From<GsmFeatures> for u8 {
    fn from(value: GsmFeatures) -> Self {
        match value {
            GsmFeatures::NotSelected => 0b00_00_00_00,
            GsmFeatures::UdhiIndicator => 0b01_00_00_00,
            GsmFeatures::SetReplyPath => 0b10_00_00_00,
            GsmFeatures::SetUdhiAndReplyPath => 0b11_00_00_00,
            GsmFeatures::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestInstance;

    use super::*;

    impl TestInstance for EsmClass {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::default(),
                Self::new(
                    MessagingMode::Forward,
                    MessageType::ShortMessageContainsIntermediateDeliveryNotification,
                    Ansi41Specific::ShortMessageContainsConversationAbort,
                    GsmFeatures::UdhiIndicator,
                ),
                Self::new(
                    MessagingMode::Datagram,
                    MessageType::ShortMessageContainsMCDeliveryReceipt,
                    Ansi41Specific::ShortMessageContainsUserAcknowledgment,
                    GsmFeatures::UdhiIndicator,
                ),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<EsmClass>();
        crate::tests::encode_decode_test_instances::<MessagingMode>();
        crate::tests::encode_decode_test_instances::<MessageType>();
        crate::tests::encode_decode_test_instances::<Ansi41Specific>();
        crate::tests::encode_decode_test_instances::<GsmFeatures>();
    }
}
