use crate::types::octet_string::OctetString;

crate::create! {
    // https://smpp.org/SMPP_v5.pdf#page=165
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub struct Subaddress {
        pub tag: SubaddressTag,
        // addr can not be empty, because the whole source_subaddress tlv value is between 2 and 23 bytes long, and the tag is 1 byte long
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode_with_length::<Subaddress>();
        crate::ende::tests::default_encode_decode::<SubaddressTag>();
    }
}
