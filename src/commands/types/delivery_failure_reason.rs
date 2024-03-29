use crate::types::u8::EndeU8;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum DeliveryFailureReason {
    #[default]
    DestinationUnavailable = 0,
    DestinationAddressInvalid = 1,
    PermanentNetworkError = 2,
    TemporaryNetworkError = 3,
    Other(u8),
}

impl From<u8> for DeliveryFailureReason {
    fn from(value: u8) -> Self {
        match value {
            0 => DeliveryFailureReason::DestinationUnavailable,
            1 => DeliveryFailureReason::DestinationAddressInvalid,
            2 => DeliveryFailureReason::PermanentNetworkError,
            3 => DeliveryFailureReason::TemporaryNetworkError,
            value => DeliveryFailureReason::Other(value),
        }
    }
}

impl From<DeliveryFailureReason> for u8 {
    fn from(value: DeliveryFailureReason) -> Self {
        match value {
            DeliveryFailureReason::DestinationUnavailable => 0,
            DeliveryFailureReason::DestinationAddressInvalid => 1,
            DeliveryFailureReason::PermanentNetworkError => 2,
            DeliveryFailureReason::TemporaryNetworkError => 3,
            DeliveryFailureReason::Other(value) => value,
        }
    }
}

impl EndeU8 for DeliveryFailureReason {}
