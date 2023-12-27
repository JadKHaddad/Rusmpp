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
pub enum AlertOnMsgDelivery {
    UseMobileDefaultAlert = 0,
    UseLowPriorityAlert = 1,
    UseMediumPriorityAlert = 2,
    UseHighPriorityAlert = 3,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for AlertOnMsgDelivery {
    fn default() -> Self {
        AlertOnMsgDelivery::UseMobileDefaultAlert
    }
}
