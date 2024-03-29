use crate::types::u8::EndeU8;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MoreMessagesToSend {
    #[default]
    NoMoreMessagesToFollow = 0,
    MoreMessagesToFollow = 1,
    Other(u8),
}

impl From<u8> for MoreMessagesToSend {
    fn from(value: u8) -> Self {
        match value {
            0 => MoreMessagesToSend::NoMoreMessagesToFollow,
            1 => MoreMessagesToSend::MoreMessagesToFollow,
            value => MoreMessagesToSend::Other(value),
        }
    }
}

impl From<MoreMessagesToSend> for u8 {
    fn from(value: MoreMessagesToSend) -> Self {
        match value {
            MoreMessagesToSend::NoMoreMessagesToFollow => 0,
            MoreMessagesToSend::MoreMessagesToFollow => 1,
            MoreMessagesToSend::Other(value) => value,
        }
    }
}

impl EndeU8 for MoreMessagesToSend {}
