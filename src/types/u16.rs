//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 2-octet integer with the decimal value of 41746 would
//! be encoded as 2 octets with the value 0xA312

use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

impl Length for u16 {
    fn length(&self) -> usize {
        2
    }
}

impl Encode for u16 {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(self.to_be_bytes().as_ref())?;

        Ok(())
    }
}

impl Decode for u16 {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut bytes = [0; 2];
        reader.read_exact(&mut bytes)?;

        let value = u16::from_be_bytes(bytes);

        Ok(value)
    }
}

/// A trait for encoding and decoding a value as [`u16`]
pub trait EndeU16
where
    Self: From<u16> + Copy,
    u16: From<Self>,
{
    fn length(&self) -> usize {
        2
    }

    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u16::from(*self).encode_to(writer)
    }

    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        u16::decode_from(reader).map(Self::from)
    }
}
