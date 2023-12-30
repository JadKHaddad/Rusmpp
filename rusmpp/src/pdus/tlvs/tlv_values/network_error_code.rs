use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoU8, RusmppIoWrite};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct NetworkErrorCode {
    pub network_type: ErrorCodeNetworkType,
    pub error_code: u16,
}

impl NetworkErrorCode {
    pub fn new(network_type: ErrorCodeNetworkType, error_code: u16) -> Self {
        Self {
            network_type,
            error_code,
        }
    }
}

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
pub enum ErrorCodeNetworkType {
    Ansi136AccessDeniedReason = 1,
    Is95AccessDeniedReason = 2,
    Gsm = 3,
    Ansi136CauseCode = 4,
    Is95CauseCode = 5,
    Ansi41Error = 6,
    SmppError = 7,
    MessageCenterSpecific = 8,
    #[num_enum(catch_all)]
    Other(u8),
}
