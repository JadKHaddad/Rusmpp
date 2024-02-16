use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum SetDpf {
    NotRequested = 0,
    #[default]
    Requested = 1,
    Other(u8),
}

impl From<u8> for SetDpf {
    fn from(value: u8) -> Self {
        match value {
            0 => SetDpf::NotRequested,
            1 => SetDpf::Requested,
            value => SetDpf::Other(value),
        }
    }
}

impl From<SetDpf> for u8 {
    fn from(value: SetDpf) -> Self {
        match value {
            SetDpf::NotRequested => 0,
            SetDpf::Requested => 1,
            SetDpf::Other(value) => value,
        }
    }
}

impl Length for SetDpf {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for SetDpf {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for SetDpf {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
