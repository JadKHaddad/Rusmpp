crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum AlertOnMessageDelivery {
        #[default]
        UseMobileDefaultAlert = 0,
        UseLowPriorityAlert = 1,
        UseMediumPriorityAlert = 2,
        UseHighPriorityAlert = 3,
        Other(u8),
    }
}

impl From<u8> for AlertOnMessageDelivery {
    fn from(value: u8) -> Self {
        match value {
            0 => AlertOnMessageDelivery::UseMobileDefaultAlert,
            1 => AlertOnMessageDelivery::UseLowPriorityAlert,
            2 => AlertOnMessageDelivery::UseMediumPriorityAlert,
            3 => AlertOnMessageDelivery::UseHighPriorityAlert,
            value => AlertOnMessageDelivery::Other(value),
        }
    }
}

impl From<AlertOnMessageDelivery> for u8 {
    fn from(value: AlertOnMessageDelivery) -> Self {
        match value {
            AlertOnMessageDelivery::UseMobileDefaultAlert => 0,
            AlertOnMessageDelivery::UseLowPriorityAlert => 1,
            AlertOnMessageDelivery::UseMediumPriorityAlert => 2,
            AlertOnMessageDelivery::UseHighPriorityAlert => 3,
            AlertOnMessageDelivery::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<AlertOnMessageDelivery>();
    }
}
