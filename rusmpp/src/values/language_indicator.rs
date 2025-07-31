crate::create! {
    #[repr(u8)]
    /// Refer to [CMT-136] for other values.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum LanguageIndicator {
        #[default]
        Unspecified = 0,
        English = 1,
        French = 2,
        Spanish = 3,
        German = 4,
        Portuguese = 5,
        Other(u8),
    }
}

impl From<u8> for LanguageIndicator {
    fn from(value: u8) -> Self {
        match value {
            0 => LanguageIndicator::Unspecified,
            1 => LanguageIndicator::English,
            2 => LanguageIndicator::French,
            3 => LanguageIndicator::Spanish,
            4 => LanguageIndicator::German,
            5 => LanguageIndicator::Portuguese,
            value => LanguageIndicator::Other(value),
        }
    }
}

impl From<LanguageIndicator> for u8 {
    fn from(value: LanguageIndicator) -> Self {
        match value {
            LanguageIndicator::Unspecified => 0,
            LanguageIndicator::English => 1,
            LanguageIndicator::French => 2,
            LanguageIndicator::Spanish => 3,
            LanguageIndicator::German => 4,
            LanguageIndicator::Portuguese => 5,
            LanguageIndicator::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<LanguageIndicator>();
    }
}
