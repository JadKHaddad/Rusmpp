use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UssdServiceOp {
    PssdIndication = 0,
    PssrIndication = 1,
    UssrRequest = 2,
    UssnRequest = 3,
    PssdResponse = 16,
    PssrResponse = 17,
    UssrConfirm = 18,
    UssnConfirm = 19,
    Other(u8),
}

impl From<u8> for UssdServiceOp {
    fn from(value: u8) -> Self {
        match value {
            0 => UssdServiceOp::PssdIndication,
            1 => UssdServiceOp::PssrIndication,
            2 => UssdServiceOp::UssrRequest,
            3 => UssdServiceOp::UssnRequest,
            16 => UssdServiceOp::PssdResponse,
            17 => UssdServiceOp::PssrResponse,
            18 => UssdServiceOp::UssrConfirm,
            19 => UssdServiceOp::UssnConfirm,
            value => UssdServiceOp::Other(value),
        }
    }
}

impl From<UssdServiceOp> for u8 {
    fn from(value: UssdServiceOp) -> Self {
        match value {
            UssdServiceOp::PssdIndication => 0,
            UssdServiceOp::PssrIndication => 1,
            UssdServiceOp::UssrRequest => 2,
            UssdServiceOp::UssnRequest => 3,
            UssdServiceOp::PssdResponse => 16,
            UssdServiceOp::PssrResponse => 17,
            UssdServiceOp::UssrConfirm => 18,
            UssdServiceOp::UssnConfirm => 19,
            UssdServiceOp::Other(value) => value,
        }
    }
}

impl Length for UssdServiceOp {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for UssdServiceOp {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for UssdServiceOp {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
