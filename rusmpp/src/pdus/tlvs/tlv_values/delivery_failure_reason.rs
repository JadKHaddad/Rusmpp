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
pub enum DeliveryFailureReason {
    DestinationUnavailable = 0,
    DestinationAddressInvalid = 1,
    PermanentNetworkError = 2,
    TemporaryNetworkError = 3,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for DeliveryFailureReason {
    fn default() -> Self {
        DeliveryFailureReason::DestinationUnavailable
    }
}
