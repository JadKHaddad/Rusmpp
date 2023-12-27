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
pub enum UssdServiceOp {
    PssdIndication = 0,
    PssrIndication = 1,
    UssrRequest = 2,
    UssnRequest = 3,
    PssdResponse = 16,
    PssrResponse = 17,
    UssrConfirm = 18,
    UssnConfirm = 19,
    #[num_enum(catch_all)]
    Other(u8),
}
