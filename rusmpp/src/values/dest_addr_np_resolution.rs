crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum DestAddrNpResolution {
        #[default]
        QueryNotPerformed = 0,
        QueryPerformedNumberNotPorted = 1,
        QueryPerformedNumberPorted = 2,
        Other(u8),
    }
}

impl From<u8> for DestAddrNpResolution {
    fn from(value: u8) -> Self {
        match value {
            0 => DestAddrNpResolution::QueryNotPerformed,
            1 => DestAddrNpResolution::QueryPerformedNumberNotPorted,
            2 => DestAddrNpResolution::QueryPerformedNumberPorted,
            value => DestAddrNpResolution::Other(value),
        }
    }
}

impl From<DestAddrNpResolution> for u8 {
    fn from(value: DestAddrNpResolution) -> Self {
        match value {
            DestAddrNpResolution::QueryNotPerformed => 0,
            DestAddrNpResolution::QueryPerformedNumberNotPorted => 1,
            DestAddrNpResolution::QueryPerformedNumberPorted => 2,
            DestAddrNpResolution::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<DestAddrNpResolution>();
    }
}
