crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum AddrSubunit {
        #[default]
        Unknown = 0x00,
        MSDisplay = 0x01,
        MobileEquipment = 0x02,
        SmartCard = 0x03,
        ExternalUnit = 0x04,
        Other(u8),
    }
}

impl From<u8> for AddrSubunit {
    fn from(value: u8) -> Self {
        match value {
            0x00 => AddrSubunit::Unknown,
            0x01 => AddrSubunit::MSDisplay,
            0x02 => AddrSubunit::MobileEquipment,
            0x03 => AddrSubunit::SmartCard,
            0x04 => AddrSubunit::ExternalUnit,
            value => AddrSubunit::Other(value),
        }
    }
}

impl From<AddrSubunit> for u8 {
    fn from(value: AddrSubunit) -> Self {
        match value {
            AddrSubunit::Unknown => 0x00,
            AddrSubunit::MSDisplay => 0x01,
            AddrSubunit::MobileEquipment => 0x02,
            AddrSubunit::SmartCard => 0x03,
            AddrSubunit::ExternalUnit => 0x04,
            AddrSubunit::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<AddrSubunit>();
    }
}
