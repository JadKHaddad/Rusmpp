use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MoreMessagesToSend {
    #[default]
    NoMoreMessagesToFollow = 0,
    MoreMessagesToFollow = 1,
    Other(u8),
}

impl From<u8> for MoreMessagesToSend {
    fn from(value: u8) -> Self {
        match value {
            0 => MoreMessagesToSend::NoMoreMessagesToFollow,
            1 => MoreMessagesToSend::MoreMessagesToFollow,
            value => MoreMessagesToSend::Other(value),
        }
    }
}

impl From<MoreMessagesToSend> for u8 {
    fn from(value: MoreMessagesToSend) -> Self {
        match value {
            MoreMessagesToSend::NoMoreMessagesToFollow => 0,
            MoreMessagesToSend::MoreMessagesToFollow => 1,
            MoreMessagesToSend::Other(value) => value,
        }
    }
}

impl Length for MoreMessagesToSend {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for MoreMessagesToSend {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for MoreMessagesToSend {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
