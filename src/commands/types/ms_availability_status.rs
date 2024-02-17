use crate::types::u8::EndeU8;

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

impl EndeU8 for MsAvailabilityStatus {}
