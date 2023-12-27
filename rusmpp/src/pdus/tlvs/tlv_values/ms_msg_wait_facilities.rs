use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, RusmppIoU8)]
pub struct MsMsgWaitFacilities {
    pub indicator: Indicator,
    pub type_of_message: TypeOfMessage,
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

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum Indicator {
    Inactive = 0b00000000,
    Active = 0b10000000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Indicator {
    fn default() -> Self {
        Indicator::Inactive
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum TypeOfMessage {
    VoicemailMessageWaiting = 0b00000000,
    FaxMessageWaiting = 0b00000001,
    ElectronicMailMessageWaiting = 0b00000010,
    OtherMessageWaiting = 0b00000011,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for TypeOfMessage {
    fn default() -> Self {
        TypeOfMessage::VoicemailMessageWaiting
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
}
