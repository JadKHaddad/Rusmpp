use crate::types::u8::EndeU8;

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

impl EndeU8 for UssdServiceOp {}
