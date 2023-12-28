pub mod c_octet_string;
pub mod empty_or_full_c_octet_string;
pub mod no_fixed_size_octet_string;
pub mod octet_string;
pub mod option;
/// An unsigned integer value, which can be 1, 2 or 4 octets
/// in size. The octets are always encoded in Most Significant
/// Byte (MSB) first order, otherwise known as Big Endian
/// Encoding.
///
/// A 2-octet integer with the decimal value of 41746 would
/// be encoded as 2 octets with the value 0xA312
pub mod u16;
/// An unsigned integer value, which can be 1, 2 or 4 octets
/// in size. The octets are always encoded in Most Significant
/// Byte (MSB) first order, otherwise known as Big Endian
/// Encoding.
///
/// A 4-octet integer with the decimal value of 31022623
/// would be encoded as 4 octets with the value 0x1D95E1F
pub mod u32;
/// An unsigned integer value, which can be 1, 2 or 4 octets
/// in size. The octets are always encoded in Most Significant
/// Byte (MSB) first order, otherwise known as Big Endian
/// Encoding.
///
/// A 1-octet Integer with a value 5, would be encoded in a
/// single octet with the value 0x05
pub mod u8;
pub mod vec;
