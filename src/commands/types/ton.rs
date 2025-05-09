// crate::create! {

#[repr(u8)]
/// Type of Number.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Ton {
    #[default]
    Unknown = 0b00000000,
    International = 0b00000001,
    National = 0b00000010,
    NetworkSpecific = 0b00000011,
    SubscriberNumber = 0b00000100,
    Alphanumeric = 0b00000101,
    Abbreviated = 0b00000110,
    Other(u8),
}
// }

impl crate::Length for Ton {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}
impl crate::Encode for Ton {
    fn encode(&self, dst: &mut [u8]) -> usize {
        u8::from(*self).encode(dst)
    }
}
impl crate::Decode for Ton {
    fn decode(src: &mut [u8]) -> Result<(Self, usize), crate::errors::DecodeError> {
        u8::decode(src).map(|(this, size)| (Self::from(this), size))
    }
}

impl Ton {
    /// Create a new [`Ton`] with a value of 0.
    ///
    /// Equivalent to [`Ton::Unknown`].
    pub fn null() -> Self {
        Self::default()
    }
}

impl From<u8> for Ton {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Ton::Unknown,
            0b00000001 => Ton::International,
            0b00000010 => Ton::National,
            0b00000011 => Ton::NetworkSpecific,
            0b00000100 => Ton::SubscriberNumber,
            0b00000101 => Ton::Alphanumeric,
            0b00000110 => Ton::Abbreviated,
            value => Ton::Other(value),
        }
    }
}

impl From<Ton> for u8 {
    fn from(value: Ton) -> Self {
        match value {
            Ton::Unknown => 0b00000000,
            Ton::International => 0b00000001,
            Ton::National => 0b00000010,
            Ton::NetworkSpecific => 0b00000011,
            Ton::SubscriberNumber => 0b00000100,
            Ton::Alphanumeric => 0b00000101,
            Ton::Abbreviated => 0b00000110,
            Ton::Other(value) => value,
        }
    }
}
