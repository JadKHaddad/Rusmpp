use crate::types::u8::EndeU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct EsmClass {
    pub messaging_mode: MessagingMode,
    pub message_type: MessageType,
    pub ansi41_specific: Ansi41Specific,
    pub gsm_features: GsmFeatures,
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
            messaging_mode: MessagingMode::from(value & 0b00000011),
            message_type: MessageType::from(value & 0b00001100),
            ansi41_specific: Ansi41Specific::from(value & 0b00110000),
            gsm_features: GsmFeatures::from(value & 0b11000000),
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

impl EndeU8 for EsmClass {}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MessagingMode {
    #[default]
    Default = 0b00000000,
    Datagram = 0b00000001,
    Forward = 0b00000010,
    StoreAndForward = 0b00000011,
    Other(u8),
}

impl From<u8> for MessagingMode {
    fn from(value: u8) -> Self {
        match value {
            0b00000001 => MessagingMode::Datagram,
            0b00000010 => MessagingMode::Forward,
            0b00000011 => MessagingMode::StoreAndForward,
            _ => MessagingMode::Other(value),
        }
    }
}

impl From<MessagingMode> for u8 {
    fn from(value: MessagingMode) -> Self {
        match value {
            MessagingMode::Default => 0b00000000,
            MessagingMode::Datagram => 0b00000001,
            MessagingMode::Forward => 0b00000010,
            MessagingMode::StoreAndForward => 0b00000011,
            MessagingMode::Other(value) => value,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MessageType {
    #[default]
    Default = 0b00000000,
    ShortMessageContainsMCDeliveryReceip = 0b00000100,
    ShortMessageContainsIntermediateDeliveryNotification = 0b00001000,
    Other(u8),
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0b00000100 => MessageType::ShortMessageContainsMCDeliveryReceip,
            0b00001000 => MessageType::ShortMessageContainsIntermediateDeliveryNotification,
            _ => MessageType::Other(value),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        match value {
            MessageType::Default => 0b00000000,
            MessageType::ShortMessageContainsMCDeliveryReceip => 0b00000100,
            MessageType::ShortMessageContainsIntermediateDeliveryNotification => 0b00001000,
            MessageType::Other(value) => value,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Ansi41Specific {
    #[default]
    ShortMessageContainsDeliveryAcknowledgement = 0b00010000,
    ShortMessageContainsUserAcknowlegment = 0b00100000,
    ShortMessageContainsConversationAbort = 0b00110000,
    Other(u8),
}

impl From<u8> for Ansi41Specific {
    fn from(value: u8) -> Self {
        match value {
            0b00010000 => Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement,
            0b00100000 => Ansi41Specific::ShortMessageContainsUserAcknowlegment,
            0b00110000 => Ansi41Specific::ShortMessageContainsConversationAbort,
            _ => Ansi41Specific::Other(value),
        }
    }
}

impl From<Ansi41Specific> for u8 {
    fn from(value: Ansi41Specific) -> Self {
        match value {
            Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement => 0b00010000,
            Ansi41Specific::ShortMessageContainsUserAcknowlegment => 0b00100000,
            Ansi41Specific::ShortMessageContainsConversationAbort => 0b00110000,
            Ansi41Specific::Other(value) => value,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum GsmFeatures {
    #[default]
    NotSelected = 0b00000000,
    UDHIIndicator = 0b01000000,
    SetReplyPath = 0b10000000,
    SetUDHIAndReplyPath = 0b11000000,
    Other(u8),
}

impl From<u8> for GsmFeatures {
    fn from(value: u8) -> Self {
        match value {
            0b01000000 => GsmFeatures::UDHIIndicator,
            0b10000000 => GsmFeatures::SetReplyPath,
            0b11000000 => GsmFeatures::SetUDHIAndReplyPath,
            _ => GsmFeatures::Other(value),
        }
    }
}

impl From<GsmFeatures> for u8 {
    fn from(value: GsmFeatures) -> Self {
        match value {
            GsmFeatures::NotSelected => 0b00000000,
            GsmFeatures::UDHIIndicator => 0b01000000,
            GsmFeatures::SetReplyPath => 0b10000000,
            GsmFeatures::SetUDHIAndReplyPath => 0b11000000,
            GsmFeatures::Other(value) => value,
        }
    }
}
