crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum BroadcastChannelIndicator {
        #[default]
        Basic = 0,
        Extended = 1,
        Other(u8),
    }
}

impl From<u8> for BroadcastChannelIndicator {
    fn from(value: u8) -> Self {
        match value {
            0 => BroadcastChannelIndicator::Basic,
            1 => BroadcastChannelIndicator::Extended,
            value => BroadcastChannelIndicator::Other(value),
        }
    }
}

impl From<BroadcastChannelIndicator> for u8 {
    fn from(value: BroadcastChannelIndicator) -> Self {
        match value {
            BroadcastChannelIndicator::Basic => 0,
            BroadcastChannelIndicator::Extended => 1,
            BroadcastChannelIndicator::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<BroadcastChannelIndicator>();
    }
}
