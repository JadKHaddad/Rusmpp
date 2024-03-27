use crate::{
    ende::decode::{Decode, DecodeError},
    impl_length_encode, tri,
    types::u8::EndeU8,
};

impl_length_encode! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct NetworkErrorCode {
        pub network_type: ErrorCodeNetworkType,
        pub error_code: u16,
    }
}

impl NetworkErrorCode {
    pub fn new(network_type: ErrorCodeNetworkType, error_code: u16) -> Self {
        Self {
            network_type,
            error_code,
        }
    }
}

impl Decode for NetworkErrorCode {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let network_type = tri!(ErrorCodeNetworkType::decode_from(reader));
        let error_code = tri!(u16::decode_from(reader));

        Ok(Self {
            network_type,
            error_code,
        })
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ErrorCodeNetworkType {
    Ansi136AccessDeniedReason = 1,
    Is95AccessDeniedReason = 2,
    Gsm = 3,
    Ansi136CauseCode = 4,
    Is95CauseCode = 5,
    Ansi41Error = 6,
    SmppError = 7,
    MessageCenterSpecific = 8,
    Other(u8),
}

impl From<u8> for ErrorCodeNetworkType {
    fn from(value: u8) -> Self {
        match value {
            1 => ErrorCodeNetworkType::Ansi136AccessDeniedReason,
            2 => ErrorCodeNetworkType::Is95AccessDeniedReason,
            3 => ErrorCodeNetworkType::Gsm,
            4 => ErrorCodeNetworkType::Ansi136CauseCode,
            5 => ErrorCodeNetworkType::Is95CauseCode,
            6 => ErrorCodeNetworkType::Ansi41Error,
            7 => ErrorCodeNetworkType::SmppError,
            8 => ErrorCodeNetworkType::MessageCenterSpecific,
            value => ErrorCodeNetworkType::Other(value),
        }
    }
}

impl From<ErrorCodeNetworkType> for u8 {
    fn from(value: ErrorCodeNetworkType) -> Self {
        match value {
            ErrorCodeNetworkType::Ansi136AccessDeniedReason => 1,
            ErrorCodeNetworkType::Is95AccessDeniedReason => 2,
            ErrorCodeNetworkType::Gsm => 3,
            ErrorCodeNetworkType::Ansi136CauseCode => 4,
            ErrorCodeNetworkType::Is95CauseCode => 5,
            ErrorCodeNetworkType::Ansi41Error => 6,
            ErrorCodeNetworkType::SmppError => 7,
            ErrorCodeNetworkType::MessageCenterSpecific => 8,
            ErrorCodeNetworkType::Other(value) => value,
        }
    }
}

impl EndeU8 for ErrorCodeNetworkType {}
