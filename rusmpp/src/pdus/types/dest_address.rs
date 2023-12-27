use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIo, RusmppIoU8};

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    types::c_octet_string::COctetString,
};

use super::{npi::Npi, ton::Ton};

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
pub enum DestFlag {
    SmeAddress = 0x01,
    DistributionListName = 0x02,
    #[num_enum(catch_all)]
    Other(u8),
}

impl From<u32> for DestFlag {
    fn from(v: u32) -> Self {
        Self::from(v as u8)
    }
}

impl From<DestFlag> for u32 {
    fn from(v: DestFlag) -> Self {
        v.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DestAddress {
    SmeAddress(SmeAddress),
    DistributionListName(DistributionListName),
}

impl IoLength for DestAddress {
    fn length(&self) -> usize {
        match self {
            Self::SmeAddress(sa) => sa.length(),
            Self::DistributionListName(dlm) => dlm.length(),
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for DestAddress {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        match self {
            Self::SmeAddress(sa) => sa.async_io_write(buf).await,
            Self::DistributionListName(dlm) => dlm.async_io_write(buf).await,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for DestAddress {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        let flag = DestFlag::async_io_read(buf).await?;

        match flag {
            DestFlag::SmeAddress => {
                let sa = SmeAddress::async_io_read(buf).await?;

                Ok(Self::SmeAddress(sa))
            }
            DestFlag::DistributionListName => {
                let dlm = DistributionListName::async_io_read(buf).await?;

                Ok(Self::DistributionListName(dlm))
            }
            DestFlag::Other(flag) => Err(IoReadError::UnsupportedKey { key: flag.into() }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct SmeAddress {
    dest_flag: DestFlag,
    dest_addr_ton: Ton,
    dest_addr_npi: Npi,
    destination_addr: COctetString<1, 21>,
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

    pub fn dest_addr_ton(&self) -> Ton {
        self.dest_addr_ton
    }

    pub fn dest_addr_npi(&self) -> Npi {
        self.dest_addr_npi
    }

    pub fn destination_addr(&self) -> &COctetString<1, 21> {
        &self.destination_addr
    }

    pub fn into_parts(self) -> (Ton, Npi, COctetString<1, 21>) {
        (
            self.dest_addr_ton,
            self.dest_addr_npi,
            self.destination_addr,
        )
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for SmeAddress {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        // flag is already read
        let dest_addr_ton = Ton::async_io_read(buf).await?;
        let dest_addr_npi = Npi::async_io_read(buf).await?;
        let destination_addr = COctetString::<1, 21>::async_io_read(buf).await?;

        Ok(Self::new(dest_addr_ton, dest_addr_npi, destination_addr))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct DistributionListName {
    dest_flag: DestFlag,
    dl_name: COctetString<1, 21>,
}

impl DistributionListName {
    pub fn new(dl_name: COctetString<1, 21>) -> Self {
        Self {
            dest_flag: DestFlag::DistributionListName,
            dl_name,
        }
    }

    pub fn dl_name(&self) -> &COctetString<1, 21> {
        &self.dl_name
    }

    pub fn into_parts(self) -> COctetString<1, 21> {
        self.dl_name
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for DistributionListName {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        // flag is already read
        let dl_name = COctetString::<1, 21>::async_io_read(buf).await?;

        Ok(Self::new(dl_name))
    }
}