crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum NetworkType {
        #[default]
        Unknown = 0x00,
        Gsm = 0x01,
        Ansi136 = 0x02,
        Is95 = 0x03,
        Pdc = 0x04,
        Phs = 0x05,
        IDen = 0x06,
        Amps = 0x07,
        PagingNetwork = 0x08,
        Other(u8),
    }
}

impl From<u8> for NetworkType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => NetworkType::Unknown,
            0x01 => NetworkType::Gsm,
            0x02 => NetworkType::Ansi136,
            0x03 => NetworkType::Is95,
            0x04 => NetworkType::Pdc,
            0x05 => NetworkType::Phs,
            0x06 => NetworkType::IDen,
            0x07 => NetworkType::Amps,
            0x08 => NetworkType::PagingNetwork,
            value => NetworkType::Other(value),
        }
    }
}

impl From<NetworkType> for u8 {
    fn from(value: NetworkType) -> Self {
        match value {
            NetworkType::Unknown => 0x00,
            NetworkType::Gsm => 0x01,
            NetworkType::Ansi136 => 0x02,
            NetworkType::Is95 => 0x03,
            NetworkType::Pdc => 0x04,
            NetworkType::Phs => 0x05,
            NetworkType::IDen => 0x06,
            NetworkType::Amps => 0x07,
            NetworkType::PagingNetwork => 0x08,
            NetworkType::Other(value) => value,
        }
    }
}
