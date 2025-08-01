use crate::{
    decode::{Decode, DecodeError, DecodeExt, DecodeResultExt},
    encode::{Encode, Length},
    types::COctetString,
};

use super::{npi::Npi, ton::Ton};

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum DestFlag {
        #[default]
        SmeAddress = 0x01,
        DistributionListName = 0x02,
        Other(u8),
    }
}

impl From<u8> for DestFlag {
    fn from(value: u8) -> Self {
        match value {
            0x01 => DestFlag::SmeAddress,
            0x02 => DestFlag::DistributionListName,
            value => DestFlag::Other(value),
        }
    }
}

impl From<DestFlag> for u8 {
    fn from(value: DestFlag) -> Self {
        match value {
            DestFlag::SmeAddress => 0x01,
            DestFlag::DistributionListName => 0x02,
            DestFlag::Other(value) => value,
        }
    }
}

impl From<DestFlag> for u32 {
    fn from(value: DestFlag) -> Self {
        u8::from(value).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum DestAddress {
    /// SME Format Destination Address.
    SmeAddress(SmeAddress),
    /// Distribution List Format Destination Address.
    DistributionListName(DistributionListName),
}

impl Length for DestAddress {
    fn length(&self) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.length(),
            Self::DistributionListName(dlm) => dlm.length(),
        }
    }
}

impl Encode for DestAddress {
    fn encode(&self, dst: &mut [u8]) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.encode(dst),
            Self::DistributionListName(dlm) => dlm.encode(dst),
        }
    }
}

impl Decode for DestAddress {
    fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
        let size = 0;

        let (flag, size) = DestFlag::decode_move(src, size)?;

        match flag {
            DestFlag::SmeAddress => {
                SmeAddress::decode_move(src, size).map_decoded(Self::SmeAddress)
            }
            DestFlag::DistributionListName => {
                DistributionListName::decode_move(src, size).map_decoded(Self::DistributionListName)
            }
            DestFlag::Other(flag) => Err(DecodeError::unsupported_key(flag.into())),
        }
    }
}

crate::create! {
    @[skip_test]
    /// SME Format Destination Address.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct SmeAddress {
        @[skip]
        /// 0x01 (SME Address).
        ///
        /// Can't and shouldn't be updated
        dest_flag: DestFlag,
        /// Type of Number for destination.
        pub dest_addr_ton: Ton,
        /// Numbering Plan Indicator for destination.
        pub dest_addr_npi: Npi,
        /// Destination address of this short message. For mobile
        /// terminated messages, this is the directory number of the
        /// recipient MS.
        pub destination_addr: COctetString<1, 21>,
    }
}

impl SmeAddress {
    pub const fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
    ) -> Self {
        Self {
            dest_flag: DestFlag::SmeAddress,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
        }
    }

    pub fn dest_flag(&self) -> DestFlag {
        self.dest_flag
    }
}

crate::create! {
    @[skip_test]
    /// Distribution List Format Destination Address.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct DistributionListName {
        @[skip]
        /// 0x02 (Distribution List).
        ///
        /// Can't and shouldn't be updated.
        dest_flag: DestFlag,
        /// Name of Distribution List.
        pub dl_name: COctetString<1, 21>,
    }
}

impl DistributionListName {
    pub fn new(dl_name: COctetString<1, 21>) -> Self {
        Self {
            dest_flag: DestFlag::DistributionListName,
            dl_name,
        }
    }

    pub fn dest_flag(&self) -> DestFlag {
        self.dest_flag
    }
}

/// # Note
///
/// [`encode_decode_test_instances`](crate::tests::encode_decode_test_instances) will fail for [`SmeAddress`] and [`DistributionListName`]
/// because they encode the `dest_flag` field but skip decoding it, since it will be extracted while decoding [`DestAddress`].
///
/// Another implementation for [`DestAddress`] that looks like `Pdu` or `Tlv` requires using the [`DestFlag`] as a key for decoding the `DestAddressVariant`,
/// and making [`DestAddress`] a struct with a `dest_flag` field and a `variant` field.
/// This means we should implement `DecodeWithKey` for `DestAddressVariant` with the [`DestFlag`] as a key.
/// But `DecodeWithKey` needs a `length` parameter, and our macro supports only @key and @length attributes.
/// So we have to create new trait that uses a key but does not require a length, and we have to update our macro to support only a key without a length.
#[cfg(test)]
mod tests {
    use super::*;

    impl crate::tests::TestInstance for DestAddress {
        fn instances() -> alloc::vec::Vec<Self> {
            alloc::vec![
                Self::SmeAddress(SmeAddress::new(
                    Ton::International,
                    Npi::Isdn,
                    COctetString::new(b"1234567890123456789\0").unwrap(),
                )),
                Self::DistributionListName(DistributionListName::new(
                    COctetString::new(b"1234567890123456789\0").unwrap(),
                )),
            ]
        }
    }

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<DestFlag>();
        crate::tests::encode_decode_test_instances::<DestAddress>();
    }
}
