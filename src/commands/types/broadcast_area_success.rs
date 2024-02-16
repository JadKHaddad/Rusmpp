// use rusmpp_macros::RusmppIoU8;

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, RusmppIoU8)]
// pub enum BroadcastAreaSuccess {
//     #[default]
//     InformationNotAvailable,
//     ZeroToHundred(u8),
//     Other(u8),
// }

// impl From<BroadcastAreaSuccess> for u8 {
//     fn from(value: BroadcastAreaSuccess) -> Self {
//         match value {
//             BroadcastAreaSuccess::InformationNotAvailable => 255,
//             BroadcastAreaSuccess::ZeroToHundred(value) => value,
//             BroadcastAreaSuccess::Other(value) => value,
//         }
//     }
// }

// impl From<u8> for BroadcastAreaSuccess {
//     fn from(value: u8) -> Self {
//         match value {
//             0..=100 => Self::ZeroToHundred(value),
//             255 => Self::InformationNotAvailable,
//             _ => Self::Other(value),
//         }
//     }
// }
