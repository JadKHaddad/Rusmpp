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
pub enum BroadcastMessageClass {
    NoClassSpecified = 0x00,
    Class1 = 0x01,
    Class2 = 0x02,
    Class3 = 0x03,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for BroadcastMessageClass {
    fn default() -> Self {
        BroadcastMessageClass::NoClassSpecified
    }
}
