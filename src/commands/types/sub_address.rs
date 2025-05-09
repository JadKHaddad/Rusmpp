use crate::{create, types::octet_string::OctetString};

create! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub struct Subaddress {
        pub tag: SubaddressTag,
        @[length = unchecked]
        pub addr: OctetString<1, 22>,
    }
}

impl Subaddress {
    pub fn new(tag: SubaddressTag, addr: OctetString<1, 22>) -> Self {
        Self { tag, addr }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub enum SubaddressTag {
        #[default]
        NsapEven = 0b10000000,
        NsapOdd = 0b10001000,
        UserSpecified = 0b10100000,
        Other(u8),
    }
}

impl From<u8> for SubaddressTag {
    fn from(value: u8) -> Self {
        match value {
            0b10000000 => SubaddressTag::NsapEven,
            0b10001000 => SubaddressTag::NsapOdd,
            0b10100000 => SubaddressTag::UserSpecified,
            value => SubaddressTag::Other(value),
        }
    }
}

impl From<SubaddressTag> for u8 {
    fn from(value: SubaddressTag) -> Self {
        match value {
            SubaddressTag::NsapEven => 0b10000000,
            SubaddressTag::NsapOdd => 0b10001000,
            SubaddressTag::UserSpecified => 0b10100000,
            SubaddressTag::Other(value) => value,
        }
    }
}
