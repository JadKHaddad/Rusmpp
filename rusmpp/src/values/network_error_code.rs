crate::create! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
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

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum ErrorCodeNetworkType {
        Ansi136AccessDeniedReason = 1,
        Is95AccessDeniedReason = 2,
        Gsm = 3,
        Ansi136CauseCode = 4,
        Is95CauseCode = 5,
        Ansi41Error = 6,
        #[default]
        SmppError = 7,
        MessageCenterSpecific = 8,
        Other(u8),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<NetworkErrorCode>();
        crate::tests::encode_decode_test_instances::<ErrorCodeNetworkType>();
    }
}
