//! Core `SMPP` types.

mod c_octet_string;
pub use c_octet_string::{COctetString, Error as COctetStringError};

mod empty_or_full_c_octet_string;
pub use empty_or_full_c_octet_string::{
    EmptyOrFullCOctetString, Error as EmptyOrFullCOctetStringError,
};

mod any_octet_string;
pub use any_octet_string::AnyOctetString;

mod octet_string;
pub use octet_string::{Error as OctetStringError, OctetString};

pub mod u16;
pub mod u32;
pub mod u8;
