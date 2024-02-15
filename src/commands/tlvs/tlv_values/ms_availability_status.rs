use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

/// The ms_availability_status parameter is used in the alert_notification operation to indicate the
/// availability state of the MS to the ESME.

/// If the MC does not include the parameter in the alert_notification operation, the ESME should
/// assume that the MS is in an “available” state.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MsAvailabilityStatus {
    #[default]
    Available = 0,
    Denied = 1,
    Unavailable = 2,
    Other(u8),
}

impl From<u8> for MsAvailabilityStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => MsAvailabilityStatus::Available,
            1 => MsAvailabilityStatus::Denied,
            2 => MsAvailabilityStatus::Unavailable,
            other => MsAvailabilityStatus::Other(other),
        }
    }
}

impl From<MsAvailabilityStatus> for u8 {
    fn from(value: MsAvailabilityStatus) -> Self {
        match value {
            MsAvailabilityStatus::Available => 0,
            MsAvailabilityStatus::Denied => 1,
            MsAvailabilityStatus::Unavailable => 2,
            MsAvailabilityStatus::Other(other) => other,
        }
    }
}

impl Length for MsAvailabilityStatus {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for MsAvailabilityStatus {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for MsAvailabilityStatus {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
