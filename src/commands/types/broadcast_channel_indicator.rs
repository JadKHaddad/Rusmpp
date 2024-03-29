use crate::types::u8::EndeU8;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum BroadcastChannelIndicator {
    #[default]
    Basic = 0,
    Extended = 1,
    Other(u8),
}

impl From<u8> for BroadcastChannelIndicator {
    fn from(value: u8) -> Self {
        match value {
            0 => BroadcastChannelIndicator::Basic,
            1 => BroadcastChannelIndicator::Extended,
            value => BroadcastChannelIndicator::Other(value),
        }
    }
}

impl From<BroadcastChannelIndicator> for u8 {
    fn from(value: BroadcastChannelIndicator) -> Self {
        match value {
            BroadcastChannelIndicator::Basic => 0,
            BroadcastChannelIndicator::Extended => 1,
            BroadcastChannelIndicator::Other(value) => value,
        }
    }
}

impl EndeU8 for BroadcastChannelIndicator {}
