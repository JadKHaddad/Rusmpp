//! [`Length`], [`Encode`] and [`DecodeWithLength`] implementation for [`heapless::Vec`]

use heapless::Vec;

use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength, VecCapacityError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

impl<T, const N: usize> Length for Vec<T, N>
where
    T: Length,
{
    fn length(&self) -> usize {
        self.iter().map(|x| x.length()).sum()
    }
}

impl<T, const N: usize> Encode for Vec<T, N>
where
    T: Encode,
{
    fn encode_to<W: crate::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        for item in self {
            tri!(item.encode_to(writer));
        }

        Ok(())
    }
}

impl<T, const N: usize> DecodeWithLength for Vec<T, N>
where
    T: Decode + Length,
{
    fn decode_from<R: crate::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        let mut remaining_length = length;

        while remaining_length > 0 {
            let v = tri!(T::decode_from(reader));
            remaining_length = remaining_length.saturating_sub(v.length());

            tri!(vec
                .push(v)
                .map_err(|_| DecodeError::VecCapacityError(VecCapacityError { capacity: N })));
        }

        Ok(vec)
    }
}
