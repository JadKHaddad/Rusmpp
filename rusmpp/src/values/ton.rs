crate::create! {
    #[repr(u8)]
    /// Type of Number.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum Ton {
        #[default]
        Unknown = 0b00000000,
        International = 0b00000001,
        National = 0b00000010,
        NetworkSpecific = 0b00000011,
        SubscriberNumber = 0b00000100,
        Alphanumeric = 0b00000101,
        Abbreviated = 0b00000110,
        Other(u8),
    }
}

impl Ton {
    /// Create a new [`Ton`] with a value of 0.
    ///
    /// Equivalent to [`Ton::Unknown`].
    pub fn null() -> Self {
        Self::default()
    }
}

impl From<u8> for Ton {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Ton::Unknown,
            0b00000001 => Ton::International,
            0b00000010 => Ton::National,
            0b00000011 => Ton::NetworkSpecific,
            0b00000100 => Ton::SubscriberNumber,
            0b00000101 => Ton::Alphanumeric,
            0b00000110 => Ton::Abbreviated,
            value => Ton::Other(value),
        }
    }
}

impl From<Ton> for u8 {
    fn from(value: Ton) -> Self {
        match value {
            Ton::Unknown => 0b00000000,
            Ton::International => 0b00000001,
            Ton::National => 0b00000010,
            Ton::NetworkSpecific => 0b00000011,
            Ton::SubscriberNumber => 0b00000100,
            Ton::Alphanumeric => 0b00000101,
            Ton::Abbreviated => 0b00000110,
            Ton::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<Ton>();
    }
}
