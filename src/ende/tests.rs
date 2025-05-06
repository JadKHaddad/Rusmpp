use core::fmt::Debug;

use crate::io::Cursor;

use super::{
    decode::{Decode, DecodeWithLength},
    encode::Encode,
};

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn default_encode_decode<T>()
where
    T: Default + Debug + PartialEq + Encode + Decode,
{
    let original = T::default();

    let mut curser = Cursor::new(::std::vec::Vec::new());

    original.encode_to(&mut curser).expect("Failed to encode");

    curser.set_position(0);

    let decoded = T::decode_from(&mut curser).expect("Failed to decode");

    assert_eq!(original, decoded);
}

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn default_encode_decode_with_length<T>()
where
    T: Default + Debug + PartialEq + Encode + DecodeWithLength,
{
    let original = T::default();

    let mut curser = Cursor::new(::std::vec::Vec::new());

    original.encode_to(&mut curser).expect("Failed to encode");

    curser.set_position(0);

    let decoded = T::decode_from(&mut curser, original.length()).expect("Failed to decode");

    assert_eq!(original, decoded);
}
