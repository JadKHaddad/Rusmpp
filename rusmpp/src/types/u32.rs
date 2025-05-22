//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 4-octet integer with the decimal value of 31022623
//! would be encoded as 4 octets with the value 0x1D95E1F

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, Length},
};

impl Length for u32 {
    fn length(&self) -> usize {
        4
    }
}

impl Encode for u32 {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let bytes = self.to_be_bytes();

        dst[0] = bytes[0];
        dst[1] = bytes[1];
        dst[2] = bytes[2];
        dst[3] = bytes[3];

        4
    }
}

impl Decode for u32 {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        if src.len() < 4 {
            return Err(DecodeError::unexpected_eof());
        }

        let mut bytes = [0; 4];

        bytes.copy_from_slice(&src[..4]);

        let value = u32::from_be_bytes(bytes);

        Ok((value, 4))
    }
}
