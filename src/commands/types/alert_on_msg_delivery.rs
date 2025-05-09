crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub enum AlertOnMsgDelivery {
        #[default]
        UseMobileDefaultAlert = 0,
        UseLowPriorityAlert = 1,
        UseMediumPriorityAlert = 2,
        UseHighPriorityAlert = 3,
        Other(u8),
    }

}

impl From<u8> for AlertOnMsgDelivery {
    fn from(value: u8) -> Self {
        match value {
            0 => AlertOnMsgDelivery::UseMobileDefaultAlert,
            1 => AlertOnMsgDelivery::UseLowPriorityAlert,
            2 => AlertOnMsgDelivery::UseMediumPriorityAlert,
            3 => AlertOnMsgDelivery::UseHighPriorityAlert,
            value => AlertOnMsgDelivery::Other(value),
        }
    }
}

impl From<AlertOnMsgDelivery> for u8 {
    fn from(value: AlertOnMsgDelivery) -> Self {
        match value {
            AlertOnMsgDelivery::UseMobileDefaultAlert => 0,
            AlertOnMsgDelivery::UseLowPriorityAlert => 1,
            AlertOnMsgDelivery::UseMediumPriorityAlert => 2,
            AlertOnMsgDelivery::UseHighPriorityAlert => 3,
            AlertOnMsgDelivery::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode::<AlertOnMsgDelivery>();
    }
}
