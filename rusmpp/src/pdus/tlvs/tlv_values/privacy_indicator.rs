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
pub enum PrivacyIndicator {
    NotRestricted = 0,
    Restricted = 1,
    Confidential = 2,
    Secret = 3,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for PrivacyIndicator {
    fn default() -> Self {
        PrivacyIndicator::NotRestricted
    }
}
