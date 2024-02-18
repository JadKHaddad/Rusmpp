use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::u8::EndeU8,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MsValidity {
    pub validity_behaviour: MsValidityBehaviour,
    pub validity_information: Option<MsValidityInformation>,
}

impl MsValidity {
    pub fn new(
        validity_behaviour: MsValidityBehaviour,
        validity_information: Option<MsValidityInformation>,
    ) -> Self {
        Self {
            validity_behaviour,
            validity_information,
        }
    }
}

impl Length for MsValidity {
    fn length(&self) -> usize {
        self.validity_behaviour.length() + self.validity_information.length()
    }
}

impl Encode for MsValidity {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.validity_behaviour.encode_to(writer));
        tri!(self.validity_information.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for MsValidity {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let validity_behaviour = tri!(MsValidityBehaviour::decode_from(reader));

        let validity_information_length = length.saturating_sub(validity_behaviour.length());

        let validity_information = tri!(MsValidityInformation::length_checked_decode_from(
            reader,
            validity_information_length
        ));

        Ok(Self {
            validity_behaviour,
            validity_information,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MsValidityInformation {
    pub units_of_time: UnitsOfTime,
    pub number_of_time_units: u16,
}

impl MsValidityInformation {
    pub fn new(units_of_time: UnitsOfTime, number_of_time_units: u16) -> Self {
        Self {
            units_of_time,
            number_of_time_units,
        }
    }
}

impl Length for MsValidityInformation {
    fn length(&self) -> usize {
        self.units_of_time.length() + self.number_of_time_units.length()
    }
}

impl Encode for MsValidityInformation {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.units_of_time.encode_to(writer));
        tri!(self.number_of_time_units.encode_to(writer));

        Ok(())
    }
}

impl Decode for MsValidityInformation {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let units_of_time = tri!(UnitsOfTime::decode_from(reader));
        let number_of_time_units = tri!(u16::decode_from(reader));

        Ok(Self {
            units_of_time,
            number_of_time_units,
        })
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MsValidityBehaviour {
    #[default]
    StoreIndefinitely = 0,
    PowerDown = 1,
    ValidUntilRegistrationAreaChanges = 2,
    DisplayOnly = 3,
    RelativeTimePeriod = 4,
    Other(u8),
}

impl From<u8> for MsValidityBehaviour {
    fn from(value: u8) -> Self {
        match value {
            0 => MsValidityBehaviour::StoreIndefinitely,
            1 => MsValidityBehaviour::PowerDown,
            2 => MsValidityBehaviour::ValidUntilRegistrationAreaChanges,
            3 => MsValidityBehaviour::DisplayOnly,
            4 => MsValidityBehaviour::RelativeTimePeriod,
            value => MsValidityBehaviour::Other(value),
        }
    }
}

impl From<MsValidityBehaviour> for u8 {
    fn from(value: MsValidityBehaviour) -> Self {
        match value {
            MsValidityBehaviour::StoreIndefinitely => 0,
            MsValidityBehaviour::PowerDown => 1,
            MsValidityBehaviour::ValidUntilRegistrationAreaChanges => 2,
            MsValidityBehaviour::DisplayOnly => 3,
            MsValidityBehaviour::RelativeTimePeriod => 4,
            MsValidityBehaviour::Other(value) => value,
        }
    }
}

impl EndeU8 for MsValidityBehaviour {}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl EndeU8 for UnitsOfTime {}
