use crate::{
    decode::borrowed::{Decode, DecodeWithLength},
    encode::Encode,
    tests::TestInstance,
};

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + Decode<'static>,
{
    for original in T::instances() {
        let buf = &mut [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(buf);

        let buf = buf[..size].to_vec().leak();

        let (decoded, _size) = T::decode(buf).expect("Failed to decode");

        assert_eq!(original, decoded);
    }
}

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_with_length_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + DecodeWithLength<'static>,
{
    for original in T::instances() {
        let mut buf = [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(&mut buf);

        let buf = buf[..size].to_vec().leak();

        let (decoded, _size) = T::decode(buf, original.length()).expect("Failed to decode");

        assert_eq!(original, decoded);
    }
}
