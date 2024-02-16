use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

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

impl Length for DeliveryFailureReason {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for DeliveryFailureReason {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for DeliveryFailureReason {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
