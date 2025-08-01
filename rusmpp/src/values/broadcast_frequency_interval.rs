crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum UnitOfTime {
        #[default]
        AsFrequentlyAsPossible = 0x00,
        Seconds = 0x08,
        Minutes = 0x09,
        Hours = 0x0A,
        Days = 0x0B,
        Weeks = 0x0C,
        Months = 0x0D,
        Years = 0x0E,
        Other(u8),
    }
}

impl From<u8> for UnitOfTime {
    fn from(value: u8) -> Self {
        match value {
            0x00 => UnitOfTime::AsFrequentlyAsPossible,
            0x08 => UnitOfTime::Seconds,
            0x09 => UnitOfTime::Minutes,
            0x0A => UnitOfTime::Hours,
            0x0B => UnitOfTime::Days,
            0x0C => UnitOfTime::Weeks,
            0x0D => UnitOfTime::Months,
            0x0E => UnitOfTime::Years,
            value => UnitOfTime::Other(value),
        }
    }
}

impl From<UnitOfTime> for u8 {
    fn from(value: UnitOfTime) -> Self {
        match value {
            UnitOfTime::AsFrequentlyAsPossible => 0x00,
            UnitOfTime::Seconds => 0x08,
            UnitOfTime::Minutes => 0x09,
            UnitOfTime::Hours => 0x0A,
            UnitOfTime::Days => 0x0B,
            UnitOfTime::Weeks => 0x0C,
            UnitOfTime::Months => 0x0D,
            UnitOfTime::Years => 0x0E,
            UnitOfTime::Other(value) => value,
        }
    }
}

crate::create! {
    /// This field indicates the frequency interval at which
    /// the broadcasts of a message should be repeated.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct BroadcastFrequencyInterval {
        pub unit: UnitOfTime,
        pub value: u16,
    }
}

impl BroadcastFrequencyInterval {
    pub fn new(unit: UnitOfTime, value: u16) -> Self {
        Self { unit, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<UnitOfTime>();
        crate::tests::encode_decode_test_instances::<BroadcastFrequencyInterval>();
    }
}
