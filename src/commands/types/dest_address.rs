use super::{npi::Npi, ton::Ton};
use crate::{
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    impl_length_encode, tri,
    types::{c_octet_string::COctetString, u8::EndeU8},
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum DestFlag {
    #[default]
    SmeAddress = 0x01,
    DistributionListName = 0x02,
    Other(u8),
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

impl EndeU8 for DestFlag {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Self::SmeAddress(sa) => sa.encode_to(writer),
            Self::DistributionListName(dlm) => dlm.encode_to(writer),
        }
    }
}

impl Decode for DestAddress {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let flag = tri!(DestFlag::decode_from(reader));

        match flag {
            DestFlag::SmeAddress => {
                let sa = tri!(SmeAddress::decode_from(reader));

                Ok(Self::SmeAddress(sa))
            }
            DestFlag::DistributionListName => {
                let dlm = tri!(DistributionListName::decode_from(reader));

                Ok(Self::DistributionListName(dlm))
            }
            DestFlag::Other(flag) => Err(DecodeError::UnsupportedKey { key: flag.into() }),
        }
    }
}

impl_length_encode! {
    /// SME Format Destination Address.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct SmeAddress {
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
    pub fn new(
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

impl Decode for SmeAddress {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        // flag is already read
        let dest_addr_ton = tri!(Ton::decode_from(reader));
        let dest_addr_npi = tri!(Npi::decode_from(reader));
        let destination_addr = tri!(COctetString::decode_from(reader));

        Ok(Self::new(dest_addr_ton, dest_addr_npi, destination_addr))
    }
}

impl_length_encode! {
    /// Distribution List Format Destination Address.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct DistributionListName {
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

impl Decode for DistributionListName {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        // flag is already read
        let dl_name = tri!(COctetString::decode_from(reader));

        Ok(Self::new(dl_name))
    }
}
