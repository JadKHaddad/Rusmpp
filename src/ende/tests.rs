use crate::{Decode, DecodeWithLength, Encode};

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn default_encode_decode<T>()
where
    T: Default + core::fmt::Debug + PartialEq + Encode + Decode,
{
    let original = T::default();

    let buf = &mut [0u8; 1024];

    if original.length() > buf.len() {
        panic!("Buffer is too small to hold the encoded data");
    }

    let size = original.encode(buf);

    let (decoded, _size) = T::decode(&buf[..size]).expect("Failed to decode");

    assert_eq!(original, decoded);
}

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn default_encode_decode_with_length<T>()
where
    T: Default + core::fmt::Debug + PartialEq + Encode + DecodeWithLength,
{
    let original = T::default();

    let buf = &mut [0u8; 1024];

    if original.length() > buf.len() {
        panic!("Buffer is too small to hold the encoded data");
    }

    let size = original.encode(buf);

    let (decoded, _size) = T::decode(&buf[..size], original.length()).expect("Failed to decode");

    assert_eq!(original, decoded);
}
