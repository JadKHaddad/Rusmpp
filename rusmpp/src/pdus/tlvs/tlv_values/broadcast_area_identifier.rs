use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIoLength, RusmppIoReadLength, RusmppIoU8, RusmppIoWrite};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, IoRead},
    },
    types::octet_string::OctetString,
};

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
pub enum BroadcastAreaFormat {
    AliasName = 0x00,
    EllipsoidArc = 0x01,
    Polygon = 0x02,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for BroadcastAreaFormat {
    fn default() -> Self {
        BroadcastAreaFormat::AliasName
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoReadLength,
)]
pub struct BroadcastAreaIdentifier {
    pub format: BroadcastAreaFormat,
    #[rusmpp_io_read(length=(length - all_before))]
    pub area: OctetString<0, 100>,
}

impl BroadcastAreaIdentifier {
    pub fn new(format: BroadcastAreaFormat, area: OctetString<0, 100>) -> Self {
        Self { format, area }
    }
}
