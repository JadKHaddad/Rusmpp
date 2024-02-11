//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 1-octet Integer with a value 5, would be encoded in a
//! single octet with the value 0x05

use crate::io::{
    decode::{AsyncDecode, DecodeError},
    encode::{AsyncEncode, EncodeError},
    length::Length,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl Length for u8 {
    fn length(&self) -> usize {
        1
    }
}

impl AsyncEncode for u8 {
    async fn encode_to<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        writer.write_u8(*self).await?;

        Ok(())
    }
}

impl AsyncDecode for u8 {
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = reader.read_u8().await?;

        Ok(value)
    }
}
