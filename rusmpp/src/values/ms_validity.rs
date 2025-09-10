crate::create! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct MsValidity {
        pub validity_behavior: MsValidityBehavior,
        @[length = checked]
        pub validity_information: Option<MsValidityInformation>,
    }
}

impl MsValidity {
    pub fn new(
        validity_behavior: MsValidityBehavior,
        validity_information: Option<MsValidityInformation>,
    ) -> Self {
        Self {
            validity_behavior,
            validity_information,
        }
    }
}

crate::create! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct MsValidityInformation {
        pub units_of_time: UnitsOfTime,
        pub number_of_time_units: u16,
    }
}

impl MsValidityInformation {
    pub fn new(units_of_time: UnitsOfTime, number_of_time_units: u16) -> Self {
        Self {
            units_of_time,
            number_of_time_units,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum MsValidityBehavior {
        #[default]
        StoreIndefinitely = 0,
        PowerDown = 1,
        ValidUntilRegistrationAreaChanges = 2,
        DisplayOnly = 3,
        RelativeTimePeriod = 4,
        Other(u8),
    }
}

impl From<u8> for MsValidityBehavior {
    fn from(value: u8) -> Self {
        match value {
            0 => MsValidityBehavior::StoreIndefinitely,
            1 => MsValidityBehavior::PowerDown,
            2 => MsValidityBehavior::ValidUntilRegistrationAreaChanges,
            3 => MsValidityBehavior::DisplayOnly,
            4 => MsValidityBehavior::RelativeTimePeriod,
            value => MsValidityBehavior::Other(value),
        }
    }
}

impl From<MsValidityBehavior> for u8 {
    fn from(value: MsValidityBehavior) -> Self {
        match value {
            MsValidityBehavior::StoreIndefinitely => 0,
            MsValidityBehavior::PowerDown => 1,
            MsValidityBehavior::ValidUntilRegistrationAreaChanges => 2,
            MsValidityBehavior::DisplayOnly => 3,
            MsValidityBehavior::RelativeTimePeriod => 4,
            MsValidityBehavior::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum UnitsOfTime {
        #[default]
        Seconds = 0b00000000,
        Minutes = 0b00000001,
        Hours = 0b00000010,
        Days = 0b00000011,
        Weeks = 0b00000100,
        Months = 0b00000101,
        Years = 0b00000110,
        Other(u8),
    }
}

impl From<u8> for UnitsOfTime {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => UnitsOfTime::Seconds,
            0b00000001 => UnitsOfTime::Minutes,
            0b00000010 => UnitsOfTime::Hours,
            0b00000011 => UnitsOfTime::Days,
            0b00000100 => UnitsOfTime::Weeks,
            0b00000101 => UnitsOfTime::Months,
            0b00000110 => UnitsOfTime::Years,
            value => UnitsOfTime::Other(value),
        }
    }
}

impl From<UnitsOfTime> for u8 {
    fn from(value: UnitsOfTime) -> Self {
        match value {
            UnitsOfTime::Seconds => 0b00000000,
            UnitsOfTime::Minutes => 0b00000001,
            UnitsOfTime::Hours => 0b00000010,
            UnitsOfTime::Days => 0b00000011,
            UnitsOfTime::Weeks => 0b00000100,
            UnitsOfTime::Months => 0b00000101,
            UnitsOfTime::Years => 0b00000110,
            UnitsOfTime::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<MsValidity>();
        crate::tests::encode_decode_test_instances::<MsValidityInformation>();
        crate::tests::encode_decode_test_instances::<MsValidityBehavior>();
        crate::tests::encode_decode_test_instances::<UnitsOfTime>();
    }
}
