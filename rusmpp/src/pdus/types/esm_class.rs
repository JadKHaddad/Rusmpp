use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, RusmppIoU8)]
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

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum MessagingMode {
    Default = 0b00000000,
    Datagram = 0b00000001,
    Forward = 0b00000010,
    StoreAndForward = 0b00000011,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for MessagingMode {
    fn default() -> Self {
        MessagingMode::Default
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum MessageType {
    Default = 0b00000000,
    ShortMessageContainsMCDeliveryReceip = 0b00000100,
    ShortMessageContainsIntermediateDeliveryNotification = 0b00001000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for MessageType {
    fn default() -> Self {
        MessageType::Default
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum Ansi41Specific {
    ShortMessageContainsDeliveryAcknowledgement = 0b00010000,
    ShortMessageContainsUserAcknowlegment = 0b00100000,
    ShortMessageContainsConversationAbort = 0b00110000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Ansi41Specific {
    fn default() -> Self {
        Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum GsmFeatures {
    NotSelected = 0b00000000,
    UDHIIndicator = 0b01000000,
    SetReplyPath = 0b10000000,
    SetUDHIAndReplyPath = 0b11000000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for GsmFeatures {
    fn default() -> Self {
        GsmFeatures::NotSelected
    }
}
