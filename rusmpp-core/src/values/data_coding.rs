use rusmpp_macros::Rusmpp;

use crate::coding::Udh;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum DataCoding {
    /// GSM 7-bit default alphabet
    #[default]
    McSpecific = 0b00000000,
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
    Other(u8),
}

impl DataCoding {
    /// Max characters for no UDH, based on encoding.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` if the encoding has a known max character count.
    /// - `None` if the encoding does not have a known max character count.
    pub(crate) const fn max_chars(self) -> Option<usize> {
        match self {
            DataCoding::McSpecific => Some(160),
            DataCoding::Ucs2 => Some(70),
            _ => None,
        }
    }

    /// Max characters for UDH, based on encoding.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` if the encoding has a known max character count with UDH.
    /// - `None` if the encoding does not have a known max character count with UDH.
    /// - `None` if the UDH length exceeds the maximum allowed bytes `140`.
    pub(crate) const fn max_chars_with_udh(self, udh: Udh) -> Option<usize> {
        const TP_UD_MAX_BYTES: usize = 140;

        let udh_len = udh.length();

        if udh_len >= TP_UD_MAX_BYTES {
            return Some(0);
        }

        let payload_bytes = TP_UD_MAX_BYTES - udh_len;

        match self {
            DataCoding::McSpecific => Some((payload_bytes * 8) / 7),
            DataCoding::Ucs2 => Some(payload_bytes / 2),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_test_instances::<DataCoding>();
        crate::tests::borrowed::encode_decode_test_instances::<DataCoding>();
    }
}
