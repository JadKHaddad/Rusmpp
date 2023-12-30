use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoU8, RusmppIoWrite};

use crate::{
    io::{length::IoLength, read::AsyncIoRead},
    types::octet_string::OctetString,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoReadLength,
)]
pub struct Subaddress {
    pub tag: SubaddressTag,
    #[rusmpp_io_read(length=(length - all_before))]
    pub addr: OctetString<1, 22>,
}

impl Subaddress {
    pub fn new(tag: SubaddressTag, addr: OctetString<1, 22>) -> Self {
        Self { tag, addr }
    }
}

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU8,
)]
pub enum SubaddressTag {
    NsapEven = 0b10000000,
    NsapOdd = 0b10001000,
    UserSpecified = 0b10100000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for SubaddressTag {
    fn default() -> Self {
        SubaddressTag::NsapEven
    }
}
