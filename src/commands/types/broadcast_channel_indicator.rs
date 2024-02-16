use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum BroadcastChannelIndicator {
    #[default]
    Basic = 0,
    Extended = 1,
    Other(u8),
}

impl From<u8> for BroadcastChannelIndicator {
    fn from(value: u8) -> Self {
        match value {
            0 => BroadcastChannelIndicator::Basic,
            1 => BroadcastChannelIndicator::Extended,
            value => BroadcastChannelIndicator::Other(value),
        }
    }
}

impl From<BroadcastChannelIndicator> for u8 {
    fn from(value: BroadcastChannelIndicator) -> Self {
        match value {
            BroadcastChannelIndicator::Basic => 0,
            BroadcastChannelIndicator::Extended => 1,
            BroadcastChannelIndicator::Other(value) => value,
        }
    }
}

impl Length for BroadcastChannelIndicator {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for BroadcastChannelIndicator {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for BroadcastChannelIndicator {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
