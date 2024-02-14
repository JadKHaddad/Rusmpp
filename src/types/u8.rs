//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 1-octet Integer with a value 5, would be encoded in a
//! single octet with the value 0x05

use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

impl Length for u8 {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for u8 {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(self.to_be_bytes().as_ref())?;

        Ok(())
    }
}

impl Decode for u8 {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut bytes = [0; 1];
        reader.read_exact(&mut bytes)?;

        let value = u8::from_be_bytes(bytes);

        Ok(value)
    }
}
