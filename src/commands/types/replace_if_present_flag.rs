use crate::types::u8::EndeU8;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum ReplaceIfPresentFlag {
    #[default]
    DoNotReplace = 0,
    Replace = 1,
    Other(u8),
}

impl From<u8> for ReplaceIfPresentFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => ReplaceIfPresentFlag::DoNotReplace,
            1 => ReplaceIfPresentFlag::Replace,
            value => ReplaceIfPresentFlag::Other(value),
        }
    }
}

impl From<ReplaceIfPresentFlag> for u8 {
    fn from(value: ReplaceIfPresentFlag) -> Self {
        match value {
            ReplaceIfPresentFlag::DoNotReplace => 0,
            ReplaceIfPresentFlag::Replace => 1,
            ReplaceIfPresentFlag::Other(value) => value,
        }
    }
}

impl EndeU8 for ReplaceIfPresentFlag {}
