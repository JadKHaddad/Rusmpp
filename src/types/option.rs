//! [`Length`] and [`AsyncEncode`] implementation for [`Option`]

use crate::ende::{
    encode::{Encode, EncodeError},
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

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(value) => value.encode_to(writer),
            None => Ok(()),
        }
    }
}
