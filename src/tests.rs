use crate::{
    decode::{Decode, DecodeWithLength},
    encode::Encode,
};

// TODO: now test every pdu with weird create! attributes like @[length = attr] and @[count = attr]

/// Trait for creating test instances of a type.
pub trait TestInstance: Sized {
    /// Create test instances of the type.
    fn instances() -> Vec<Self>;
}

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + Decode,
{
    for original in T::instances() {
        #[cfg(feature = "tracing")]
        {
            tracing::debug!(encoding=?original);
        }

        let buf = &mut [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(buf);

        #[cfg(feature = "tracing")]
        {
            tracing::debug!(encoded=?original);
            tracing::debug!(encoded=?crate::utils::HexFormatter(&buf[..size]), encoded_length=size);
        }

        let (decoded, _size) = T::decode(&buf[..size]).expect("Failed to decode");

        #[cfg(feature = "tracing")]
        {
            tracing::debug!(decoded=?decoded, decoded_length=_size);
        }

        assert_eq!(original, decoded);
    }
}

/// Test encoding and decoding of a type.
///
/// Encode a type to bytes and then decode it back to the original type.
pub fn encode_decode_with_length_test_instances<T>()
where
    T: TestInstance + core::fmt::Debug + PartialEq + Encode + DecodeWithLength,
{
    for original in T::instances() {
        #[cfg(feature = "tracing")]
        {
            tracing::debug!(encoding=?original);
        }

        let buf = &mut [0u8; 1024];

        if original.length() > buf.len() {
            panic!("Buffer is too small to hold the encoded data");
        }

        let size = original.encode(buf);

        #[cfg(feature = "tracing")]
        {
            tracing::debug!(encoded=?original);
            tracing::debug!(encoded=?crate::utils::HexFormatter(&buf[..size]), encoded_length=size);
        }

        let (decoded, _size) =
            T::decode(&buf[..size], original.length()).expect("Failed to decode");

        #[cfg(feature = "tracing")]
        {
            tracing::debug!(decoded=?decoded, decoded_length=_size);
        }

        assert_eq!(original, decoded);
    }
}
