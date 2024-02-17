use crate::types::u8::EndeU8;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum SetDpf {
    NotRequested = 0,
    #[default]
    Requested = 1,
    Other(u8),
}

impl From<u8> for SetDpf {
    fn from(value: u8) -> Self {
        match value {
            0 => SetDpf::NotRequested,
            1 => SetDpf::Requested,
            value => SetDpf::Other(value),
        }
    }
}

impl From<SetDpf> for u8 {
    fn from(value: SetDpf) -> Self {
        match value {
            SetDpf::NotRequested => 0,
            SetDpf::Requested => 1,
            SetDpf::Other(value) => value,
        }
    }
}

impl EndeU8 for SetDpf {}
