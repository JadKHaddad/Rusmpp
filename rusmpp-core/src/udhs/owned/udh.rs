use rusmpp_macros::Rusmpp;

use crate::{
    encode::{Encode, Length},
    types::owned::AnyOctetString,
    udhs::{ConcatenatedShortMessage8Bit, ConcatenatedShortMessage16Bit, UdhId},
};

/// User Data Header (UDH).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = skip, test = skip)]
pub struct Udh {
    /// UDH length (excluding the length field itself).
    length: u8,
    /// UDH identifier.
    id: UdhId,
    /// UDH value.
    value: Option<UdhValue>,
}

impl Udh {
    pub fn new(value: impl Into<UdhValue>) -> Self {
        let value = value.into();
        let id = value.id();
        let length = value.length() as u8 + id.length() as u8;

        Self {
            id,
            length,
            value: Some(value),
        }
    }

    pub const fn id(&self) -> UdhId {
        self.id
    }

    pub const fn length(&self) -> u8 {
        self.length
    }

    pub const fn value(&self) -> Option<&UdhValue> {
        self.value.as_ref()
    }
}

impl From<UdhValue> for Udh {
    fn from(value: UdhValue) -> Self {
        Self::new(value)
    }
}

/// User Data Header (UDH) value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UdhValue {
    /// 8-bit Concatenated Short Message UDH.
    ConcatenatedShortMessage8Bit(ConcatenatedShortMessage8Bit),
    /// 16-bit Concatenated Short Message UDH.
    ConcatenatedShortMessage16Bit(ConcatenatedShortMessage16Bit),
    /// Other UDH types.
    Other { udh_id: UdhId, body: AnyOctetString },
}

impl UdhValue {
    /// Returns the UDH identifier.
    pub const fn id(&self) -> UdhId {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(_) => UdhId::ConcatenatedShortMessages8Bit,
            UdhValue::ConcatenatedShortMessage16Bit(_) => UdhId::ConcatenatedShortMessages16Bit,
            UdhValue::Other { udh_id, .. } => *udh_id,
        }
    }
}

impl Length for UdhValue {
    fn length(&self) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.length(),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.length(),
            UdhValue::Other { body, .. } => body.length(),
        }
    }
}

impl Encode for UdhValue {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            UdhValue::ConcatenatedShortMessage8Bit(udh) => udh.encode(dst),
            UdhValue::ConcatenatedShortMessage16Bit(udh) => udh.encode(dst),
            UdhValue::Other { body, .. } => body.encode(dst),
        }
    }
}

// TODO: decode this guy
