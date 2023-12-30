use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoRead,
)]
pub struct ItsSessionInfo {
    pub session_number: u8,
    pub sequence_number: u8,
}

impl ItsSessionInfo {
    pub fn new(session_number: u8, sequence_number: u8) -> Self {
        Self {
            session_number,
            sequence_number,
        }
    }
}
