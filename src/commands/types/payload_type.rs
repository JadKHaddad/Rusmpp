crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub enum PayloadType {
        #[default]
        Default = 0,
        WcmpMessage = 1,
        Other(u8),
    }
}

impl From<u8> for PayloadType {
    fn from(value: u8) -> Self {
        match value {
            0 => PayloadType::Default,
            1 => PayloadType::WcmpMessage,
            value => PayloadType::Other(value),
        }
    }
}

impl From<PayloadType> for u8 {
    fn from(value: PayloadType) -> Self {
        match value {
            PayloadType::Default => 0,
            PayloadType::WcmpMessage => 1,
            PayloadType::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::tests::default_encode_decode::<PayloadType>();
    }
}
