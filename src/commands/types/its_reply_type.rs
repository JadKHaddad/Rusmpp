use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum ItsReplyType {
    #[default]
    Digit = 0,
    Number = 1,
    TelephoneNo = 2,
    Password = 3,
    CharacterLine = 4,
    Menu = 5,
    Date = 6,
    Time = 7,
    Continue = 8,
    Other(u8),
}

impl From<u8> for ItsReplyType {
    fn from(value: u8) -> Self {
        match value {
            0 => ItsReplyType::Digit,
            1 => ItsReplyType::Number,
            2 => ItsReplyType::TelephoneNo,
            3 => ItsReplyType::Password,
            4 => ItsReplyType::CharacterLine,
            5 => ItsReplyType::Menu,
            6 => ItsReplyType::Date,
            7 => ItsReplyType::Time,
            8 => ItsReplyType::Continue,
            value => ItsReplyType::Other(value),
        }
    }
}

impl From<ItsReplyType> for u8 {
    fn from(value: ItsReplyType) -> Self {
        match value {
            ItsReplyType::Digit => 0,
            ItsReplyType::Number => 1,
            ItsReplyType::TelephoneNo => 2,
            ItsReplyType::Password => 3,
            ItsReplyType::CharacterLine => 4,
            ItsReplyType::Menu => 5,
            ItsReplyType::Date => 6,
            ItsReplyType::Time => 7,
            ItsReplyType::Continue => 8,
            ItsReplyType::Other(value) => value,
        }
    }
}

impl Length for ItsReplyType {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for ItsReplyType {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for ItsReplyType {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
