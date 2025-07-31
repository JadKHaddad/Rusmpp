crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum MoreMessagesToSend {
        #[default]
        NoMoreMessagesToFollow = 0,
        MoreMessagesToFollow = 1,
        Other(u8),
    }
}

impl From<u8> for MoreMessagesToSend {
    fn from(value: u8) -> Self {
        match value {
            0 => MoreMessagesToSend::NoMoreMessagesToFollow,
            1 => MoreMessagesToSend::MoreMessagesToFollow,
            value => MoreMessagesToSend::Other(value),
        }
    }
}

impl From<MoreMessagesToSend> for u8 {
    fn from(value: MoreMessagesToSend) -> Self {
        match value {
            MoreMessagesToSend::NoMoreMessagesToFollow => 0,
            MoreMessagesToSend::MoreMessagesToFollow => 1,
            MoreMessagesToSend::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<MoreMessagesToSend>();
    }
}
