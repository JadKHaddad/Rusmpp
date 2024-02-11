//! [`Length`] and [`AsyncEncode`] implementation for [`Option`]

use crate::io::{
    encode::{AsyncEncode, EncodeError},
    length::Length,
};

impl<T> Length for Option<T>
where
    T: Length,
{
    fn length(&self) -> usize {
        match self {
            Some(value) => value.length(),
            None => 0,
        }
    }
}

impl<T> AsyncEncode for Option<T>
where
    T: AsyncEncode,
{
    async fn encode_to<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        match self {
            Some(value) => value.encode_to(writer).await,
            None => Ok(()),
        }
    }
}
