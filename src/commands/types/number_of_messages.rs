use crate::types::u8::EndeU8;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumberOfMessages {
    Allowed(u8),
    Other(u8),
}

impl From<u8> for NumberOfMessages {
    fn from(value: u8) -> Self {
        match value {
            0..=99 => NumberOfMessages::Allowed(value),
            _ => NumberOfMessages::Other(value),
        }
    }
}

impl From<NumberOfMessages> for u8 {
    fn from(value: NumberOfMessages) -> Self {
        match value {
            NumberOfMessages::Allowed(value) => value,
            NumberOfMessages::Other(value) => value,
        }
    }
}

impl EndeU8 for NumberOfMessages {}
