crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum BearerType {
        #[default]
        Unknown = 0x00,
        Sms = 0x01,
        Csd = 0x02,
        PacketData = 0x03,
        Ussd = 0x04,
        Cdpd = 0x05,
        DataTac = 0x06,
        FlexReFlex = 0x07,
        CellBroadcast = 0x08,
        Other(u8),
    }
}

impl From<u8> for BearerType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BearerType::Unknown,
            0x01 => BearerType::Sms,
            0x02 => BearerType::Csd,
            0x03 => BearerType::PacketData,
            0x04 => BearerType::Ussd,
            0x05 => BearerType::Cdpd,
            0x06 => BearerType::DataTac,
            0x07 => BearerType::FlexReFlex,
            0x08 => BearerType::CellBroadcast,
            value => BearerType::Other(value),
        }
    }
}

impl From<BearerType> for u8 {
    fn from(value: BearerType) -> Self {
        match value {
            BearerType::Unknown => 0x00,
            BearerType::Sms => 0x01,
            BearerType::Csd => 0x02,
            BearerType::PacketData => 0x03,
            BearerType::Ussd => 0x04,
            BearerType::Cdpd => 0x05,
            BearerType::DataTac => 0x06,
            BearerType::FlexReFlex => 0x07,
            BearerType::CellBroadcast => 0x08,
            BearerType::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<BearerType>();
    }
}
