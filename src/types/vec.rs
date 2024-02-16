//! [`Length`], [`Encode`] and [`DecodeWithLength`] implementation for [`Vec`]

use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

impl<T> Length for Vec<T>
where
    T: Length,
{
    fn length(&self) -> usize {
        self.iter().map(|x| x.length()).sum()
    }
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        for item in self {
            tri!(item.encode_to(writer));
        }

        Ok(())
    }
}

impl<T> DecodeWithLength for Vec<T>
where
    T: Decode + Length,
{
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        let mut remaining_length = length;

        while remaining_length > 0 {
            let v = tri!(T::decode_from(reader));
            remaining_length = remaining_length.saturating_sub(v.length());
            vec.push(v);
        }

        Ok(vec)
    }
}
