crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum DpfResult {
        #[default]
        NotSet = 0,
        Set = 1,
        Other(u8),
    }
}

impl From<u8> for DpfResult {
    fn from(value: u8) -> Self {
        match value {
            0 => DpfResult::NotSet,
            1 => DpfResult::Set,
            value => DpfResult::Other(value),
        }
    }
}

impl From<DpfResult> for u8 {
    fn from(value: DpfResult) -> Self {
        match value {
            DpfResult::NotSet => 0,
            DpfResult::Set => 1,
            DpfResult::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<DpfResult>();
    }
}
