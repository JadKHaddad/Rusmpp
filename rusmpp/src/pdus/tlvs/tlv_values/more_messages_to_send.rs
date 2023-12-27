use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU8;

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU8,
)]
pub enum MoreMessagesToSend {
    NoMoreMessagesToFollow = 0,
    MoreMessagesToFollow = 1,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for MoreMessagesToSend {
    fn default() -> Self {
        MoreMessagesToSend::MoreMessagesToFollow
    }
}
