//! An unsigned integer value, which can be 1, 2 or 4 octets
//! in size. The octets are always encoded in Most Significant
//! Byte (MSB) first order, otherwise known as Big Endian
//! Encoding.
//!
//! A 2-octet integer with the decimal value of 41746 would
//! be encoded as 2 octets with the value 0xA312

use crate::io::{
    decode::{AsyncDecode, DecodeError},
    encode::{AsyncEncode, EncodeError},
    length::Length,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl Length for u16 {
    fn length(&self) -> usize {
        2
    }
}

impl AsyncEncode for u16 {
    async fn encode_to<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        writer.write_u16(*self).await?;

        Ok(())
    }
}

impl AsyncDecode for u16 {
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = reader.read_u16().await?;

        Ok(value)
    }
}
