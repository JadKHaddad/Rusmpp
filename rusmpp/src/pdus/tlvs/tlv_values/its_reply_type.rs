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
pub enum ItsReplyType {
    Digit = 0,
    Number = 1,
    TelephoneNo = 2,
    Password = 3,
    CharacterLine = 4,
    Menu = 5,
    Date = 6,
    Time = 7,
    Continue = 8,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for ItsReplyType {
    fn default() -> Self {
        ItsReplyType::Digit
    }
}
