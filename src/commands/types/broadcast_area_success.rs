use crate::types::u8::EndeU8;

/// The success rate indicator, defined as the ratio of the
/// number of BTSs that accepted the message and the total
/// number of BTSs that should have accepted the message, for
/// a particular broadcast_area_identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum BroadcastAreaSuccess {
    #[default]
    InformationNotAvailable,
    ZeroToHundred(u8),
    Other(u8),
}

impl From<BroadcastAreaSuccess> for u8 {
    fn from(value: BroadcastAreaSuccess) -> Self {
        match value {
            BroadcastAreaSuccess::InformationNotAvailable => 255,
            BroadcastAreaSuccess::ZeroToHundred(value) => value,
            BroadcastAreaSuccess::Other(value) => value,
        }
    }
}

impl From<u8> for BroadcastAreaSuccess {
    fn from(value: u8) -> Self {
        match value {
            0..=100 => Self::ZeroToHundred(value),
            255 => Self::InformationNotAvailable,
            _ => Self::Other(value),
        }
    }
}

impl EndeU8 for BroadcastAreaSuccess {}
