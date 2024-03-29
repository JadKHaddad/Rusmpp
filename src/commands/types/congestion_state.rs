use crate::types::u8::EndeU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum CongestionState {
    #[default]
    Idle,
    LowLoad(u8),
    MediumLoad(u8),
    HighLoad(u8),
    OptimunLoad(u8),
    NearingCongestion(u8),
    Congested,
    Other(u8),
}

impl From<u8> for CongestionState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Idle,
            1..=29 => Self::LowLoad(value),
            30..=49 => Self::MediumLoad(value),
            50..=79 => Self::HighLoad(value),
            80..=89 => Self::OptimunLoad(value),
            90..=99 => Self::NearingCongestion(value),
            100 => Self::Congested,
            _ => Self::Other(value),
        }
    }
}

impl From<CongestionState> for u8 {
    fn from(value: CongestionState) -> Self {
        match value {
            CongestionState::Idle => 0,
            CongestionState::LowLoad(v) => v,
            CongestionState::MediumLoad(v) => v,
            CongestionState::HighLoad(v) => v,
            CongestionState::OptimunLoad(v) => v,
            CongestionState::NearingCongestion(v) => v,
            CongestionState::Congested => 100,
            CongestionState::Other(v) => v,
        }
    }
}

impl EndeU8 for CongestionState {}
