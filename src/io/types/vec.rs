//! Implementations of [`Length`], [`AsyncEncode`] and [`AsyncDecodeWithLength`] for [`Vec`].

use crate::io::{
    decode::{AsyncDecode, AsyncDecodeWithLength, DecodeError},
    encode::{AsyncEncode, EncodeError},
    length::Length,
};

impl<T> Length for Vec<T>
where
    T: Length,
{
    fn length(&self) -> usize {
        self.iter().map(|x| x.length()).sum()
    }
}

impl<T> AsyncEncode for Vec<T>
where
    T: AsyncEncode,
{
    async fn encode_to<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        for item in self {
            item.encode_to(writer).await?;
        }

        Ok(())
    }
}

impl<T> AsyncDecodeWithLength for Vec<T>
where
    T: AsyncDecode + Length,
{
    async fn decode_from<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
        length: usize,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        let mut remaining_length = length;

        while remaining_length > 0 {
            let v = T::decode_from(reader).await?;
            remaining_length = remaining_length.saturating_sub(v.length());
            vec.push(v);
        }

        Ok(vec)
    }
}
