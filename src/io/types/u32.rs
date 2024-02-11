//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 4-octet integer with the decimal value of 31022623
//! would be encoded as 4 octets with the value 0x1D95E1F

use crate::io::traits::{
    decode::{AsyncDecode, DecodeError},
    encode::{AsyncEncode, EncodeError},
    length::Length,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl Length for u32 {
    fn length(&self) -> usize {
        4
    }
}

impl AsyncEncode for u32 {
    async fn encode_to<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        writer.write_u32(*self).await?;

        Ok(())
    }
}

impl AsyncDecode for u32 {
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = reader.read_u32().await?;

        Ok(value)
    }
}
