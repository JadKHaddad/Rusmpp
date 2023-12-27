use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIo, RusmppIoU8};

use crate::io::read::{AsyncIoRead, AsyncIoReadable, IoReadError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
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

#[async_trait::async_trait]
impl AsyncIoRead for NetworkErrorCode {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            network_type: ErrorCodeNetworkType::async_io_read(buf).await?,
            error_code: u16::async_io_read(buf).await?,
        })
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
