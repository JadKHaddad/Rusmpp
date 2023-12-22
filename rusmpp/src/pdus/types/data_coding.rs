use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum DataCoding {
    McSpesific = 0b00000000,
    Ia5 = 0b00000001,
    OctetUnspecified = 0b00000010,
    Latin1 = 0b00000011,
    OctetUnspecified2 = 0b00000100,
    Jis = 0b00000101,
    Cyrillic = 0b00000110,
    LatinHebrew = 0b00000111,
    Ucs2 = 0b00001000,
    PictogramEncoding = 0b00001001,
    Iso2022JpMusicCodes = 0b00001010,
    ExtendedKanjiJis = 0b00001101,
    Ksc5601 = 0b00001110,
    GsmMwiControl = 0b11000000,
    GsmMwiControl2 = 0b11010000,
    GsmMessageClassControl = 0b11100000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for DataCoding {
    fn default() -> Self {
        DataCoding::McSpesific
    }
}

impl IoLength for DataCoding {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for DataCoding {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for DataCoding {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}
