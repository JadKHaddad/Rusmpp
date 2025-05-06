//! Core SMPP types.

pub mod c_octet_string;
pub use c_octet_string::COctetString;

pub mod empty_or_full_c_octet_string;
pub use empty_or_full_c_octet_string::EmptyOrFullCOctetString;

// AnyOctetString is only available with the "alloc" feature because it uses Vec<u8> internally.
// If "alloc" is not enabled, use OctetString<1, MAX> instead.
#[cfg(feature = "alloc")]
pub mod any_octet_string;
#[cfg(feature = "alloc")]
pub use any_octet_string::AnyOctetString;

pub mod octet_string;
pub use octet_string::OctetString;

mod option;

pub mod u16;

pub mod u32;

pub mod u8;

#[cfg(feature = "alloc")]
mod vec;

mod heapless_vec;
