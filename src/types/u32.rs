//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 4-octet integer with the decimal value of 31022623
//! would be encoded as 4 octets with the value 0x1D95E1F

use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

impl Length for u32 {
    fn length(&self) -> usize {
        4
    }
}

impl Encode for u32 {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(self.to_be_bytes().as_ref())?;

        Ok(())
    }
}

impl Decode for u32 {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes)?;

        let value = u32::from_be_bytes(bytes);

        Ok(value)
    }
}

/// A trait for encoding and decoding a value as [`u32`]
pub trait EndeU32
where
    Self: From<u32> + Copy,
    u32: From<Self>,
{
    fn length(&self) -> usize {
        4
    }

    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u32::from(*self).encode_to(writer)
    }

    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        u32::decode_from(reader).map(Self::from)
    }
}
